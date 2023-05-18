use crate::protocol::{Body, Payload};
use std::collections::HashSet;

use crate::protocol::Message;

pub struct Broadcast {
    // values received from Broadcast
    messages: HashSet<usize>,

    // neighbors we receive from Topology messages
    neighborhood: HashSet<String>,
}

impl Broadcast {
    pub fn new(messages: HashSet<usize>, neighborhood: HashSet<String>) -> Self {
        Self {
            messages,
            neighborhood,
        }
    }

    pub fn add_neighbor(&mut self, neighbor: String) {
        self.neighborhood.insert(neighbor);
    }

    pub fn add_message(&mut self, message: usize) {
        self.messages.insert(message);
    }

    pub fn gossip(&mut self, source: &String, dest: &String, message: usize) -> anyhow::Result<()> {
        if self.messages.contains(&message) {
            // we've seen this message before, bail
            return Ok(());
        }

        // ok we've NOT seen this before, tell everyone
        for (generated_msg_id, neighbor) in self.neighborhood.iter().enumerate() {
            let gossip_message = Message {
                src: source.clone(),
                dest: neighbor.clone(),
                body: Body {
                    msg_id: Some(generated_msg_id),
                    in_reply_to: None,
                    payload: Payload::Broadcast { message },
                },
            };
        }
        Ok(())
    }
}
