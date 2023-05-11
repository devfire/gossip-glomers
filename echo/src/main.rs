use std::io::{self, BufRead, Write};

use anyhow::{bail, Context};
use echo::protocol::{Body, Message, Payload};

mod protocol;

// use env_logger::Env;
// use log::{error, info};

fn main() -> anyhow::Result<()> {
    // Setup the logging framework
    // let env = Env::default()
    //     .filter_or("LOG_LEVEL", "info")
    //     .write_style_or("LOG_STYLE", "always");

    // env_logger::init_from_env(env);

    // info!("Starting the maelstrom client.");

    // let stdin = std::io::stdin().lock();
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    // let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();

    // let stdout = std::io::stdout().lock();

    // let mut output = serde_json::Serializer::new(stdout);

    // Message IDs should be unique on the node which sent them.
    // For instance, each node can use a monotonically increasing integer as their source of message IDs.
    let mut generated_msg_id: usize = 0;

    // Use the lines iterator from the io::BufRead trait. 
    // This iterator yields lines from a buffer, where each line is terminated by a newline character
    for line in stdin.lock().lines() {
        // let input = input.context("Unable to deserialize input.")?;

        // info!("Received {:?}", input);

        if let Ok(line) = line {
            // To deserialize a JSON line, use the from_str function from the serde_json crate. 
            // This function takes a string and deserializes it into a struct. 
            let input: Message = serde_json::from_str(&line)?;
            match input.body.payload {
                echo::protocol::Payload::Init { .. } => {
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
                    let reply_json = serde_json::to_string(&reply)?;

                    // Send the reply back as byte array
                    stdout.write_all(reply_json.as_bytes())?;
                }
                echo::protocol::Payload::Echo { msg } => {
                    let reply = Message {
                        src: input.dest,
                        dest: input.src,
                        body: Body {
                            msg_id: Some(generated_msg_id),
                            in_reply_to: input.body.msg_id,
                            payload: Payload::EchoOk { msg },
                        },
                    };

                    // Serialize the reply into a JSON string
                    let reply_json = serde_json::to_string(&reply)?;

                    // Send the reply back as byte array
                    stdout.write_all(reply_json.as_bytes())?;
                }
                echo::protocol::Payload::EchoOk { .. } => {}
                echo::protocol::Payload::InitOk { .. } => bail!("Unexpected InitOk received"),
            }
        }
        generated_msg_id += 1;
    } // end of for loop

    Ok(())
}
