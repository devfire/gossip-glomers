use std::io::{self, Write};

use anyhow::{bail, Context};
use echo::protocol::{Body, Message, Payload};

mod protocol;

use env_logger::Env;
use log::{error, info};

fn main() -> anyhow::Result<()> {
    // Setup the logging framework
    let env = Env::default()
        .filter_or("LOG_LEVEL", "info")
        .write_style_or("LOG_STYLE", "always");

    env_logger::init_from_env(env);

    info!("Starting the maelstrom client.");

    let std_reader = std::io::stdin().lock();
    let mut std_writer = io::BufWriter::new(std::io::stdout().lock());

    let inputs = serde_json::Deserializer::from_reader(std_reader).into_iter::<Message>();
    // let mut output = serde_json::Serializer::new(std_writer);

    for input in inputs {
        let input = input.context("Unable to deserialize input.")?;

        info!("Received {:?}", input);

        match input.body.payload {
            echo::protocol::Payload::Init { .. } => {
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        msg_id: input.body.msg_id,
                        in_reply_to: input.body.msg_id,
                        payload: Payload::InitOk,
                    },
                };

                // Serialize the struct as JSON
                let reply_json = serde_json::to_string(&reply)?;

                info!("Sending back {:?}", reply_json);

                // Print the JSON to stdout
                writeln!(std_writer, "{}", reply_json)?;
            }
            echo::protocol::Payload::Echo { msg } => {
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        msg_id: input.body.msg_id,
                        in_reply_to: input.body.msg_id,
                        payload: Payload::EchoOk { msg },
                    },
                };

                // Serialize the struct as JSON
                let reply_json = serde_json::to_string(&reply)?;

                // Print the JSON to stdout
                writeln!(std_writer, "{}", reply_json)?;
            }
            echo::protocol::Payload::EchoOk { .. } => {}
            echo::protocol::Payload::InitOk { .. } => bail!("Unexpected InitOk received"),
        }
    }

    Ok(())
}
