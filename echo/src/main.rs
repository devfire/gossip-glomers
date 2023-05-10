use anyhow::Context;
// use serde::Deserialize;
// use std::io::{self, BufRead};

use crate::protocol::{Message, Payload::Echo, Payload::EchoOk};

mod protocol;

fn main() -> anyhow::Result<()> {
    let std_reader = std::io::stdin().lock();

    let inputs = serde_json::Deserializer::from_reader(std_reader).into_iter::<Message>();

    for input in inputs {
        let input = input.context("Unable to deserialize input.")?;

        match input.body.payload {
            Echo { msg } => todo!(),
            EchoOk { .. } => {}
        }
    }

    Ok(())
}
