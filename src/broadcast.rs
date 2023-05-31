use crate::protocol::{Body, Payload};
use std::{
    collections::{HashMap, HashSet},
    io::Write,
};

use crate::protocol::Message;

pub struct Broadcast {
    // values received from Broadcast
    pub messages: HashSet<usize>,

    // neighbors we receive from Topology messages
    pub neighborhood: HashSet<String>,

    // this keeps track of who sent what msg
    pub received: HashMap<usize, String>,
}

impl Broadcast {
    pub fn new() -> Broadcast {
        Broadcast {
            messages: HashSet::new(),
            neighborhood: HashSet::new(),
            received: HashMap::new(),
        }
    }
    pub fn send_reply(&self, reply: Message) -> anyhow::Result<()> {
        let mut stdout = std::io::stdout();

        // Serialize the reply into a JSON string
        let mut reply_json = serde_json::to_string(&reply)?;

        reply_json.push('\n');

        // Send the reply back as byte array
        stdout.write_all(reply_json.as_bytes())?;

        Ok(())
    }

    pub fn add_neighbor(&mut self, neighbor: String) {
        self.neighborhood.insert(neighbor);
    }

    pub fn add_message(&mut self, message: usize) {
        self.messages.insert(message);
    }

    pub fn get_messages(&self) -> Result<HashSet<usize>, anyhow::Error> {
        Ok(self.messages.clone())
    }

    pub fn gossip(&mut self, source: &str, message: usize) -> anyhow::Result<()> {
        if self.messages.contains(&message) {
            // we've seen this message before, bail
            // println!("Seen {message} bailing");
            return Ok(());
        }

        // ok we've NOT seen this before, tell everyone
        for (generated_msg_id, neighbor) in self.neighborhood.iter().enumerate() {
            let gossip_message = Message {
                src: source.to_owned(),
                dest: neighbor.clone(),
                body: Body {
                    msg_id: Some(generated_msg_id),
                    in_reply_to: None,
                    payload: Payload::Broadcast { message },
                },
            };

            if let Some(node) = self.received .get(&message) {
                // someone sent us this msg, let's see if it's the node we are about to send it back to
                if node != neighbor {
                    // someone else sent this to us, so it's ok to send it back
                    self.send_reply(gossip_message)?;
                } else {
                    // oh no, it is the same node, skip it
                }
            }
        }

        // save it for the future
        self.add_message(message);

        Ok(())
    }
}
