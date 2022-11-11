use libp2p::{
    identity::Keypair, 
    PeerId,
};

fn main() {
    let local_keys = Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_keys.public());
    println!("PeerId: {}", local_peer_id);

}
