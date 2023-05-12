use std::io::{BufRead, Write};

use anyhow::bail;
use chrono::Utc;
use gossip_glomers::protocol::{Body, Message, Payload};
use rand::Rng;

mod protocol;

// use env_logger::Env;
// use log::{error, info};

pub fn get_unix_timestamp_us() -> i64 {
    let now = Utc::now();
    now.timestamp_nanos()
}

fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();

    let mut rng = rand::thread_rng();
    // println!("nanos time: {}", get_unix_timestamp_us());

    // Use the lines iterator from the io::BufRead trait.
    // This iterator yields lines from a buffer, where each line is terminated by a newline character
    for (generated_msg_id, line) in stdin.lock().lines().enumerate() {
        if let Ok(line) = line {
            // To deserialize a JSON line, use the from_str function from the serde_json crate.
            // This function takes a string and deserializes it into a struct.
            let input: Message = serde_json::from_str(&line)?;
            match input.body.payload {
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

                    // Serialize the reply into a JSON string
                    let mut reply_json = serde_json::to_string(&reply)?;

                    reply_json.push('\n');

                    // Send the reply back as byte array
                    stdout.write_all(reply_json.as_bytes())?;
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

                    // Serialize the reply into a JSON string
                    let mut reply_json = serde_json::to_string(&reply)?;

                    reply_json.push('\n');

                    // Send the reply back as byte array
                    stdout.write_all(reply_json.as_bytes())?;
                }
                Payload::EchoOk { .. } => {}
                Payload::InitOk { .. } => bail!("Unexpected InitOk received"),
                Payload::Generate => {
                    let first_part: String = get_unix_timestamp_us().to_string();
                    let second_part: String = rng.gen::<i32>().to_string();
                    let id = format!("{}{}", first_part, second_part);

                    let reply = Message {
                        src: input.dest,
                        dest: input.src,
                        body: Body {
                            msg_id: Some(generated_msg_id),
                            in_reply_to: input.body.msg_id,
                            payload: Payload::GenerateOk { id },
                        },
                    };

                    // Serialize the reply into a JSON string
                    let mut reply_json = serde_json::to_string(&reply)?;

                    reply_json.push('\n');

                    // Send the reply back as byte array
                    stdout.write_all(reply_json.as_bytes())?;
                }
                Payload::GenerateOk { .. } => {}
            }
        }
    } // end of for loop

    Ok(())
}
