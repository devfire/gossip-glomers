use std::{io::{BufRead, Write}, collections::{HashSet, HashMap}};

use gossip_glomers::protocol::{Body, Message, Payload};

use ulid::{self, Ulid};

mod protocol;

fn send_reply(reply: Message) -> anyhow::Result<()> {
    let mut stdout = std::io::stdout();

    // Serialize the reply into a JSON string
    let mut reply_json = serde_json::to_string(&reply)?;

    reply_json.push('\n');

    // Send the reply back as byte array
    stdout.write_all(reply_json.as_bytes())?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin();

    // values received from Broadcast
    let mut messages: HashSet<usize> = HashSet::new();

    // messages we know the neighbors have received
    let mut confirmed_received: HashMap<String, HashSet<usize>>;

    // Use the lines iterator from the io::BufRead trait.
    // This iterator yields lines from a buffer, where each line is terminated by a newline character
    for (generated_msg_id, line) in stdin.lock().lines().enumerate() {
        if let Ok(line) = line {
            // To deserialize a JSON line, use the from_str function from the serde_json crate.
            // This function takes a string and deserializes it into a struct.
            let input: Message = serde_json::from_str(&line)?;
            match input.body.payload {
                Payload::Topology { .. } => {
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
                    send_reply(reply)?;
                }
                Payload::Broadcast { message } => {
                    // preserve the message
                    messages.insert(message);

                    // ack it
                    let reply = Message {
                        src: input.dest,
                        dest: input.src,
                        body: Body {
                            msg_id: Some(generated_msg_id),
                            in_reply_to: input.body.msg_id,
                            payload: Payload::BroadcastOk,
                        },
                    };
                    send_reply(reply)?;
                }

                Payload::Read => {
                    let messages = messages.clone();

                    // ack it
                    let reply = Message {
                        src: input.dest,
                        dest: input.src,
                        body: Body {
                            msg_id: Some(generated_msg_id),
                            in_reply_to: input.body.msg_id,
                            payload: Payload::ReadOk { messages },
                        },
                    };
                    send_reply(reply)?;
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

                    send_reply(reply)?;
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

                    send_reply(reply)?;
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

                    send_reply(reply)?;
                }
            }
        }
    } // end of for loop

    Ok(())
}
