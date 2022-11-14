# Mainet
A p2p chat written in rust using libp2p.

## How it works?
Using `Kademlia` for storing the username.

Using `Gossipsub` for sending messages.

Using `Gossipsub Topic` for subscribing to groups.

## Commands
`help` clear and print the commands.

`send: <message>` send a message.

`sendg: <group> <message>` send a message in a group.

`set_name: <name>` set the name using `kademlia.start_providing(name)`.

`sub: <group>` subscribe to a group.

`unsub: <group>` unsubscribe to a group.

`ls ps` list the peers.

`ls ts` list the topics.

