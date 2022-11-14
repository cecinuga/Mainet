use libp2p::{ 
    NetworkBehaviour, 
    gossipsub::{ Gossipsub, GossipsubEvent },
    kad::{ record::store::MemoryStore, Kademlia, KademliaEvent},
    mdns::{ Mdns, MdnsEvent },
};

#[derive(NetworkBehaviour)]
#[behaviour(out_event="MyBehaviourEvent")]
pub struct MyBehaviour{
    pub gossipsub: Gossipsub,
    pub kademlia: Kademlia<MemoryStore>,
    pub mdns: Mdns,
}

impl MyBehaviour {
    pub fn new(gossipsub: Gossipsub, mdns: Mdns, kademlia: Kademlia<MemoryStore>) -> Self {
        Self{ 
            gossipsub, mdns, kademlia,
        }
    }
}

#[allow(clippy::large_enum_variant)]
pub enum MyBehaviourEvent{
    Gossipsub(GossipsubEvent),
    Kademlia(KademliaEvent),
    Mdns(MdnsEvent),
}

impl From<GossipsubEvent> for MyBehaviourEvent{
    fn from(v: GossipsubEvent) -> Self {
        Self::Gossipsub(v)
    }
}

impl From<KademliaEvent> for MyBehaviourEvent{
    fn from(v: KademliaEvent) -> Self {
        Self::Kademlia(v)
    }
}

impl From<MdnsEvent> for MyBehaviourEvent{
    fn from(v: MdnsEvent) -> Self{
        Self::Mdns(v)
    }
}
