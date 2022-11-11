use libp2p::{
    identity,
    identity::Keypair, 
    gossipsub,
    gossipsub::{
        DataTransform, subscription_filter::TopicSubscriptionFilter,
        Gossipsub, GossipsubEvent, GossipsubMessage, IdentTopic as Topic, MessageAuthenticity,
        ValidationMode,},
    NetworkBehaviour,
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    swarm::{Swarm, SwarmEvent},
    PeerId,
};
use async_std::io;
use std::error::Error;
use std::time::Duration;

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "OutEvent")]
struct MyBehaviour{
    gossipsub: Gossipsub,
    mdns: Mdns,
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
enum OutEvent {
    Floodsub(GossipsubEvent),
    Mdns(MdnsEvent),
}

impl From<MdnsEvent> for OutEvent {
    fn from(v: MdnsEvent) -> Self {
        Self::Mdns(v)
    }
}

impl From<GossipsubEvent> for OutEvent {
    fn from(v: GossipsubEvent) -> Self {
        Self::Floodsub(v)
    }
}

#[async_std::main] 
async fn main() -> Result<(), Box<dyn Error>>{
    let local_keys = Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_keys.public());
    let topic = Topic::new("mainet");

    println!("PeerId: {}", local_peer_id);

    let transport = libp2p::development_transport(local_keys.clone()).await?;
    

    let mut swarm = {
        let gossipsub_config = gossipsub::GossipsubConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(10))
            .validation_mode(ValidationMode::Strict)
            .build()
            .expect("Valid config"); 
        let mut gossipsub: Gossipsub = Gossipsub::new(MessageAuthenticity::Signed(local_keys), gossipsub_config)
            .expect("Correct configuration");
        gossipsub.subscribe(&topic)?;
        let mdns = Mdns::new(MdnsConfig::default()).await?;
        let behaviour = MyBehaviour { gossipsub, mdns };

        Swarm::new(transport, behaviour, local_peer_id);
    };

    let mut stdin = io::BufReader::new(io::stdin()).lines().fuse();

    Ok(())
}
