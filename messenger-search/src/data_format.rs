/// The root struct that represents a messenger conversation.
#[derive(Deserialize, Debug)]
pub struct Conversation {
    pub participants: Vec<Participant>,
    pub messages: Vec<Message>,
    pub title: String,
    pub is_still_participant: bool,
    pub thread_type: ThreadType,
    pub thread_path: String
}

#[derive(Deserialize, Debug)]
pub struct Participant {
    pub name: String
}

#[derive(Deserialize, Debug)]
pub struct Message {
    pub sender_name: String,
    pub timestamp_ms: i64,
    pub content: String,
    pub sticker: Option<Sticker>,
    #[serde(rename = "type")]
    pub message_type: MessageType
}

#[derive(Deserialize, Debug)]
pub struct Sticker {
    pub uri: String
}

#[derive(Deserialize, Debug)]
pub enum MessageType {
    Generic
}

#[derive(Deserialize, Debug)]
pub enum ThreadType {
    Regular
}

#[cfg(test)]
mod test {
    use std::fs;
    use super::Conversation;
    use ::serde_json;

    #[test]
    fn test_parsing() {
        let json_string = fs::read_to_string("sample-data/message.json").expect("file");
        let _: Conversation = serde_json::from_str(&json_string).expect("deserialization");
    }
}
