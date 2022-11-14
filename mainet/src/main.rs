use libp2p::gossipsub::{
    GossipsubEvent, IdentTopic as Topic, MessageAuthenticity, ValidationMode,
};
use libp2p::kad::record::store::MemoryStore;
use libp2p::kad::{
    record::Key, Kademlia, KademliaEvent, QueryResult,
};
use libp2p::mdns::{Mdns, MdnsConfig, MdnsEvent};
use libp2p::swarm::SwarmBuilder;
use libp2p::tcp::{GenTcpConfig, TokioTcpTransport};
use libp2p::{
    core::upgrade, futures::StreamExt, gossipsub, identity, mplex, noise, swarm::SwarmEvent, PeerId,
};
use libp2p::{Multiaddr, Transport, };
use std::error::Error;
use std::time::Duration;
use tokio::io::{self, AsyncBufReadExt, };
use tokio::{self, select};
use once_cell::sync::Lazy;
use mainet::{set_addr, handle_input_command, behaviour::{MyBehaviour, MyBehaviourEvent} };

static KEYS: Lazy<identity::Keypair> = Lazy::new(identity::Keypair::generate_ed25519);
static PEER_ID: Lazy<PeerId> = Lazy::new(|| PeerId::from(KEYS.public()));
static TOPIC: Lazy<Topic> = Lazy::new(|| Topic::new("mainet"));

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    pretty_env_logger::init();

    println!("PeerID: {}", PEER_ID.clone());
    let mut stdin = io::BufReader::new(io::stdin()).lines();

    let transport = TokioTcpTransport::new(GenTcpConfig::default().nodelay(true))
        .upgrade(upgrade::Version::V1)
        .authenticate(
            noise::NoiseAuthenticated::xx(&KEYS)
                .expect("Signing libp2p-noise static DH keypair failed"),
        )
        .multiplex(mplex::MplexConfig::new())
        .boxed();

    let mut swarm = {
        let store = MemoryStore::new(PEER_ID.clone());
        let kademlia = Kademlia::new(PEER_ID.clone(), store);
        let mdns = Mdns::new(MdnsConfig::default())?;
        let gossipsub_config = gossipsub::GossipsubConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(10))
            .validation_mode(ValidationMode::Strict)
            .build()
            .expect("Valid config");
        let mut gossipsub: gossipsub::Gossipsub = 
            gossipsub::Gossipsub::new(MessageAuthenticity::Signed(KEYS.clone()), gossipsub_config)
                .expect("Correct configuration");

        gossipsub.subscribe(&TOPIC).unwrap();

        let behaviour = MyBehaviour{ gossipsub, mdns, kademlia };
        SwarmBuilder::new(transport, behaviour, PEER_ID.clone())
            .executor(Box::new(|fut| {
                tokio::spawn(fut);
            }))
            .build()
    };
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    let mut name = String::from("anonymous");
    let addr = set_addr(&mut swarm, &mut stdin).await.parse::<Multiaddr>().unwrap();

    print!("\x1B[2J\x1B[1;1H");
    println!("type 'help'");
    println!("PeerID: {}", PEER_ID.clone());
 
    loop {
        select! {
            line = stdin.next_line() => handle_input_command(&mut swarm, &mut stdin, &mut name, &line.unwrap().expect("Message not sended."), TOPIC.clone()).await,
            event = swarm.select_next_some() => match event {
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("{}", address)
                },
                SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(MdnsEvent::Discovered(list))) => {
                    for (peer_id, multiaddr) in list{
                        println!("mDNS discovered a new peer: {}", peer_id);
                        swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                        swarm.behaviour_mut().kademlia.add_address(&peer_id, multiaddr);
                    }
                },
                SwarmEvent::Behaviour(MyBehaviourEvent::Mdns(MdnsEvent::Expired(list))) => {
                    for (peer_id, multiaddr) in list{
                        println!("mDNS expired a new peer: {}", peer_id);
                        swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                        swarm.behaviour_mut().kademlia.remove_address(&peer_id, &multiaddr);
                    }
                },
                SwarmEvent::Behaviour(MyBehaviourEvent::Gossipsub(GossipsubEvent::Message { 
                    propagation_source: _,
                    message_id: _,
                    message,
                })) => println!("{}", String::from_utf8_lossy(&message.data)),
                SwarmEvent::Behaviour(MyBehaviourEvent::Kademlia(KademliaEvent::OutboundQueryCompleted{ result, .. })) => {
                    match result {
                        QueryResult::GetProviders(Ok(ok)) => {
                            if ok.providers.is_empty() {    
                                swarm.behaviour_mut().kademlia.start_providing(Key::new(&name)).expect("Name not saved."); 
                            } else { 
                                println!("[#] Error Name taken, name resetted.");
                                swarm.behaviour_mut().kademlia.stop_providing(&Key::new(&name));
                                name = "anonymous".to_string();
                            }
                        },
                        _=>{}
                    }
                }
                _=>{}
            }
        }
    }
}