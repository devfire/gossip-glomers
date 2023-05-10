use serde::{Deserialize, Serialize};

/// Both STDIN and STDOUT messages are JSON objects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// A string identifying the node this message came from
    pub src: String,

    /// A string identifying the node this message is to
    pub dest: String,

    /// An object: the payload of the message
    pub body: MessageBody,
}

/// RPC messages exchanged with Maelstrom's clients
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageBody {
    /// A string identifying the type of message this is
    pub msg_type: String,

    /// An optional unique integer identifier
    pub msg_id: Option<usize>,

    /// For req/response, optional msg_id of the request
    pub in_reply_to: Option<usize>,

    /// Actual payload with various msg types
    #[serde(flatten)]
    pub payload: Payload,
}

/// The actual payload gets flattened and then converted to json in snake_case
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Payload {
    Echo { msg: String },
    EchoOk { msg: String },
}
