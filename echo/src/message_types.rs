use serde::Deserialize;

/// Both STDIN and STDOUT messages are JSON objects
#[derive(Debug, Deserialize)]
pub struct Message {
    /// A string identifying the node this message came from
    pub src: String,

    /// A string identifying the node this message is to
    pub dest: String,

    /// An object: the payload of the message
    body: MessageBody,
}

#[derive(Debug, Deserialize)]
struct MessageBody {
    /// A string identifying the type of message this is
    msg_type: String,

    /// An optional unique integer identifier
    msg_id: Option<usize>,

    /// For req/response, optional msg_id of the request
    in_reply_to: Option<usize>,

}