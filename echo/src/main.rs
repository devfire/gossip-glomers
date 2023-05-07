use serde::Deserialize;
use std::io::{self, BufRead};

use crate::message_types::Message;

mod message_types;

fn main() {
    let input = std::io::stdin();
    for line in input.lock().lines() {
        // here line is a String without the trailing newline
        // Deserialize the JSON object into a struct
        let message: Message = serde_json::from_str(&line).unwrap();

        // Do something with the struct
        println!("Read person: {:?}", message);
    }
}
