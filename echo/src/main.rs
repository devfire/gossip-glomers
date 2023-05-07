use std::io::{self, BufRead};
use serde::{Deserialize};

mod message_types;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut reader = io::BufReader::new(stdout.lock());

    let mut line = String::new();
    loop {
        // Read the next line from stdout
        reader.read_line(&mut line).unwrap();

        // If the line ends in "\n", we're done
        if line.ends_with("\n") {
            break;
        }
    }

    // Remove the trailing "\n" character from the line
    line.pop();

    // Deserialize the JSON object into a struct
    let message: Person = serde_json::from_str(&line).unwrap();

    // Do something with the struct
    println!("Read person: {:?}", person);
}
