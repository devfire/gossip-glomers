use serde::{Deserialize, Serialize};

/// Both STDIN and STDOUT messages are JSON objects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// A string identifying the node this message came from
    pub src: String,

    /// A string identifying the node this message is to
    pub dest: String,

    /// An object: the payload of the message
    body: MessageBody,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MessageBody {
    /// A string identifying the type of message this is
    msg_type: String,

    /// An optional unique integer identifier
    msg_id: Option<usize>,

    /// For req/response, optional msg_id of the request
    in_reply_to: Option<usize>,

    /// Actual payload with various msg types
    #[serde(flatten)]
    payload: Payload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Payload {
    Echo { echo: String },
}
