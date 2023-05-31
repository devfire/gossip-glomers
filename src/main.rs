use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
    sync::mpsc::channel,
    thread,
};

use gossip_glomers::{
    broadcast::Broadcast,
    protocol::{Body, Message, Payload},
};

use ulid::{self, Ulid};

mod protocol;

fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin();

    let (tx, mut rx) = channel::<Message>();


    thread::spawn(move || while let Ok(msg) = rx.recv() {

    }); // end of thread::spawn



    // Use the lines iterator from the io::BufRead trait.
    // This iterator yields lines from a buffer, where each line is terminated by a newline character
    for (generated_msg_id, line) in stdin.lock().lines().enumerate() {
        if let Ok(line) = line {
            let tx_channel = tx.clone();
            thread::spawn(move || {
                // To deserialize a JSON line, use the from_str function from the serde_json crate.
                // This function takes a string and deserializes it into a struct.
                let input: Message = serde_json::from_str(&line)?;
                match input.body.payload {
                    Payload::Topology { mut topology } => {
                        // Removes a key from the map, returning the value at the key,
                        // if the key was previously in the map.
                        //
                        // We need this to figure out who our neighbors are.
                        broadcast.neighborhood = topology
                            .remove(&input.dest)
                            .unwrap_or_else(|| panic!("Failed extracting our hood from topology"));

                        // println!(
                        //     "I am {} my hood is {:?} ",
                        //     input.dest, broadcast.neighborhood
                        // );

                        // ack it
                        let reply = Message {
                            src: input.dest,
                            dest: input.src,
                            body: Body {
                                msg_id: Some(generated_msg_id),
                                in_reply_to: input.body.msg_id,
                                payload: Payload::TopologyOk,
                            },
                        };
                        tx_channel.send(reply)?;
                    }
                    Payload::Broadcast { message } => {
                        // ack it
                        let reply = Message {
                            src: input.dest.clone(),
                            dest: input.src.clone(),
                            body: Body {
                                msg_id: Some(generated_msg_id),
                                in_reply_to: input.body.msg_id,
                                payload: Payload::BroadcastOk,
                            },
                        };

                        tx_channel.send(reply)?;

                        // // persist the message and its origin
                        // broadcast.received_from.insert(message, input.src);

                        // // ack the received broadcast
                        // broadcast.send_reply(reply)?;

                        // // tell everyone else what we just got
                        // broadcast.gossip(&input.dest, message)?;
                    }

                    Payload::Read => {
                        // since we don't have access to Broadcast, 
                        // we set messages to None for now.
                        // we'll fill in the correct value at the receiver end
                        let messages = None;

                        // ack it
                        let reply = Message {
                            src: input.dest,
                            dest: input.src,
                            body: Body {
                                msg_id: Some(generated_msg_id),
                                in_reply_to: input.body.msg_id,
                                payload: Payload::ReadOk {messages},
                            },
                        };
                        tx_channel.send(reply)?;
                    }
                    Payload::Init { .. } => {
                        let reply = Message {
                            src: input.dest,
                            dest: input.src,
                            body: Body {
                                msg_id: Some(generated_msg_id),
                                in_reply_to: input.body.msg_id,
                                payload: Payload::InitOk,
                            },
                        };

                        tx_channel.send(reply)?;
                    }
                    Payload::Echo { echo } => {
                        let reply = Message {
                            src: input.dest,
                            dest: input.src,
                            body: Body {
                                msg_id: Some(generated_msg_id),
                                in_reply_to: input.body.msg_id,
                                payload: Payload::EchoOk { echo },
                            },
                        };

                        tx_channel.send(reply)?;
                    }
                    Payload::ReadOk { .. }
                    | Payload::TopologyOk
                    | Payload::EchoOk { .. }
                    | Payload::GenerateOk { .. }
                    | Payload::InitOk { .. }
                    | Payload::BroadcastOk => {}

                    Payload::Generate => {
                        // Generate a ulid
                        let ulid = Ulid::new();

                        // Generate a string for a ulid
                        let id = ulid.to_string();

                        let reply = Message {
                            src: input.dest,
                            dest: input.src,
                            body: Body {
                                msg_id: Some(generated_msg_id),
                                in_reply_to: input.body.msg_id,
                                payload: Payload::GenerateOk { id },
                            },
                        };

                        tx_channel.send(reply)?;
                    }
                } // end of match input.body.payload
            }); // end of thread::spawn
        } // end of if let Ok
    } // end of for loop

    Ok(())
}
