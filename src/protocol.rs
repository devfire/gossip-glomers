use serde::{Deserialize, Serialize};

/// Both STDIN and STDOUT messages are JSON objects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// A string identifying the node this message came from
    pub src: String,

    /// A string identifying the node this message is to
    pub dest: String,

    /// An object: the payload of the message
    pub body: Body,
}

/// RPC messages exchanged with Maelstrom's clients
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body {
    //"type" is defined below with a serde tag
    /// An optional unique integer identifier
    pub msg_id: Option<usize>,

    /// For req/response, optional msg_id of the request
    pub in_reply_to: Option<usize>,

    /// Actual payload with various msg types
    #[serde(flatten)]
    pub payload: Payload,
}

/// A string identifying the type of message this is
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Payload {
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
    Generate,
    GenerateOk {
        id: String,
    },
}
