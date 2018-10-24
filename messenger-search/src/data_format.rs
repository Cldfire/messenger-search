use error::Error;
use std::path::Path;
use std::fs;

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

impl Conversation {
    pub fn from_json_str(json: &str) -> Result<Self, Error> {
        Ok(::serde_json::from_str(json)?)
    }

    pub fn from_json_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let json_string = fs::read_to_string(path)?;
        Self::from_json_str(&json_string)
    }
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

/// This message struct is how the documents tantivy gives as after executing a query
/// are interpreted.
// TODO: Get rid of vecs
#[derive(Deserialize, Debug)]
pub struct StoredMessage {
    pub timestamp: Vec<i64>,
    pub content: Vec<String>
}

impl StoredMessage {
    pub fn from_json_str(json: &str) -> Result<Self, Error> {
        Ok(::serde_json::from_str(json)?)
    }
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
    use super::Conversation;

    #[test]
    fn test_parsing() {
        let _: Conversation = Conversation::from_json_file("sample-data/message.json").unwrap();
    }
}
