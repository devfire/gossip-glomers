/// Both STDIN and STDOUT messages are JSON objects
struct Message {
    /// A string identifying the node this message came from
    src: String,

    /// A string identifying the node this message is to
    dest: String,

    /// An object: the payload of the message
    body: MessageBody,
}

struct MessageBody {
    /// A string identifying the type of message this is
    msg_type: String,

    /// An optional unique integer identifier
    msg_id: Option<usize>,

    /// For req/response, optional msg_id of the request
    in_reply_to: Option<usize>,

}