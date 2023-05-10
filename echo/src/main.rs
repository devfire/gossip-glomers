use anyhow::{bail, Context};
use echo::protocol::{Body, Message, Payload};
use serde::Serialize;

mod protocol;

fn main() -> anyhow::Result<()> {
    let std_reader = std::io::stdin().lock();
    let std_writer = std::io::stdout().lock();

    let inputs = serde_json::Deserializer::from_reader(std_reader).into_iter::<Message>();
    let mut output = serde_json::Serializer::new(std_writer);

    for input in inputs {
        let input = input.context("Unable to deserialize input.")?;

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
                reply.serialize(&mut output)?;
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
                reply.serialize(&mut output)?;
            }
            echo::protocol::Payload::EchoOk { .. } => {}
            echo::protocol::Payload::InitOk { .. } => bail!("Unexpected InitOk received"),
        }
    }

    Ok(())
}
