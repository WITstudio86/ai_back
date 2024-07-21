use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::write;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub model: String,
    pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        self.model == other.model && self.messages == other.messages
    }
}
impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        self.role == other.role && self.content == other.content
    }
}

pub fn save_message(data: Data) -> Result<()> {
    let content = serde_json::to_string(&data).unwrap();
    let path = Path::new("data.json");
    write(path, content)?;
    Ok(())
}

pub fn read_message() -> Result<Data> {
    let path = Path::new("data.json");
    let content = std::fs::read_to_string(path)?;
    let data: Data = serde_json::from_str(&content)?;
    Ok(data)
}
