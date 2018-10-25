use error::Error;
use std::path::Path;
use std::fs;
use serde_derive::Deserialize;

/// The root struct that represents a messenger conversation.
#[derive(Deserialize, Debug)]
pub struct Conversation {
    #[serde(flatten)]
    pub header: ConversationHeader,
    pub messages: Vec<Message>
}

impl Conversation {
    pub fn from_json_str(json: &str) -> Result<Self, Error> {
        Ok(serde_json::from_str(json)?)
    }

    pub fn from_json_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let json_string = fs::read_to_string(path)?;
        Self::from_json_str(&json_string)
    }
}

/// Top-level information (everything but messages).
#[derive(Deserialize, Debug)]
pub struct ConversationHeader {
    pub participants: Vec<Participant>,
    pub title: String,
    pub is_still_participant: bool,
    pub thread_type: ThreadType,
    pub thread_path: String
}

impl ConversationHeader {
    pub fn from_json_str(json: &str) -> Result<Self, Error> {
        Ok(serde_json::from_str(json)?)
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
    /// This field should be `Some` when the `MessageType` is `Share`.
    pub share: Option<Share>,
    pub photos: Option<Vec<Photo>>,
    #[serde(rename = "type")]
    pub message_type: MessageType
}

/// This message struct is how the documents tantivy gives as after executing a query
/// are interpreted.
// TODO: Get rid of vecs
#[derive(Deserialize, Debug)]
pub struct StoredMessage {
    pub sender_name: Vec<String>,
    pub timestamp_ms: Vec<i64>,
    pub content: Vec<String>
}

impl StoredMessage {
    pub fn from_json_str(json: &str) -> Result<Self, Error> {
        Ok(serde_json::from_str(json)?)
    }
}

#[derive(Deserialize, Debug)]
pub struct Sticker {
    pub uri: String
}

#[derive(Deserialize, Debug)]
pub struct Share {
    pub link: String
}

#[derive(Deserialize, Debug)]
pub struct Photo {
    pub uri: String,
    pub creation_timestamp: i64
}

#[derive(Deserialize, Debug)]
pub enum MessageType {
    Generic,
    Share
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
