use std::io::StdoutLock;

use anyhow::Context;
use echo::protocol::{Message,Body, Payload};
use serde::Serialize;

mod protocol;

fn main() -> anyhow::Result<()> {
    let std_reader = std::io::stdin().lock();
    let std_writer = std::io::stdout().lock();

    let inputs = serde_json::Deserializer::from_reader(std_reader).into_iter::<Message>();

    for input in inputs {
        let input = input.context("Unable to deserialize input.")?;

        match input.body.payload {
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
                let reply_msg = serde_json::to_string(&reply)?;
            },
            echo::protocol::Payload::EchoOk { .. } => {}
        }
    }

    Ok(())
}
