# Mainet
A p2p chat written in rust using libp2p.

## How it works?
Using `Kademlia` for storing the username.
Using `Gossipsub` for sending messages.
Using `Gossipsub Topic` for subscribing to groups.

## Commands
`send: <message>` send a message.
`set_name: <name>` set the name using `kademlia.start_providing(name)`.

