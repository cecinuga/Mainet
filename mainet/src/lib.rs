use std::hash::Hash;

use libp2p::{ Swarm, Multiaddr, 
    kad::record::{ Key },
    gossipsub::IdentTopic as Topic,
};
use tokio::io::{Lines, BufReader, Stdin};

use crate::behaviour::MyBehaviour;


pub async  fn set_addr(swarm: &mut Swarm<MyBehaviour>,stdin: &mut Lines<BufReader<Stdin>>) -> String {
    let mut valid_addr = false;
    let mut _address_ : String = String::from("");

    while !valid_addr {
        println!("Enter an address (blank to get a new one)");
        let address = stdin
            .next_line()
            .await
            .expect("Valid address").unwrap()
            .to_owned();

        if address == String::new() {
            break;
        }
        if let Ok(addr) = address.parse::<Multiaddr>() {
            match swarm.dial(addr.clone()){
                Ok(_address_) => {
                    valid_addr = true;
                    println!("Dialed {:?}", address);
                }
                Err(err) => println!("Dialed error{}",err)
            }
        };
        _address_ = address;
    };
    _address_
}

pub async fn handle_input_command(swarm: &mut Swarm<MyBehaviour>, stdin: &mut Lines<BufReader<Stdin>>,name: &mut String, line: &String, topic: Topic){
    let mut args = line.split(' ');

    match args.next(){
        Some("set_name:")=>{
            let mut name_ = args.collect::<Vec<&str>>().join(" ");

            //swarm.behaviour_mut().kademlia.stop_providing(&Key::new(&name));
            swarm.behaviour_mut().kademlia.get_providers(Key::new(&name_));
            
            *name = name_;
        }
        Some("send:")=>{
            let body = args.collect::<Vec<&str>>().join(" ");
            let message = format!("{}: {}", name, body);
            if let Err(e) = 
                swarm
                    .behaviour_mut()
                    .gossipsub.publish(topic, message.as_bytes()){
                        println!("Publish error: {}", e);
                    } 
        },
        Some("sendg:")=>{
            let new_topic = Topic::new(args.next().expect("Topic not setted."));
            let body = args.collect::<Vec<&str>>().join(" ");
            let message = format!("{}: {}", name, body);
            if let Err(e) = 
                swarm
                    .behaviour_mut()
                    .gossipsub.publish(new_topic, message.as_bytes()){
                        println!("Publish error: {}", e);
                    } 
        },
        Some("sub:")=>{
            let topic = Topic::new(args.next().expect("Topic not subscribed"));
            swarm.behaviour_mut().gossipsub.subscribe(&topic).unwrap();
        }
        Some("unsub:")=>{
            let topic = Topic::new(args.next().expect("Topic not unsubscribed"));
            swarm.behaviour_mut().gossipsub.unsubscribe(&topic).unwrap();
        }
        Some("ls") => {
            match args.next() {
                Some("ps") => {
                    for (i, (peer, _)) in swarm.behaviour_mut().gossipsub.all_peers().enumerate(){
                        println!("[{}] {:?}", i+1, peer)
                    }
                },
                Some("ts") => {
                    for (i, topic) in swarm.behaviour_mut().gossipsub.topics().enumerate(){
                        println!("[{}] {:?}",i+1, topic);
                    }
                }
                _=>{}
            }
        }
        Some("help") => {
            println!("Commands...");
            println!("help");
            println!("ls ps");
            println!("ls ts");
            println!("set_name: <name>");
            println!("send: <message>");
            println!("sendg: <group> <message>");
            println!("sub: <group>");
            println!("unsub: <group>");
        }
        Some("clear")=>{
            println!("\x1B[2J\x1B[1;1H");
        }
        _=>{println!("[#]No command found.")}
    }
}

pub mod behaviour;