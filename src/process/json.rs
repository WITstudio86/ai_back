use anyhow::Result;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs::write;
use std::path::Path;

// 定义一个结构体Data，用于存储模型和消息
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    pub model: String,
    pub messages: Vec<Message>,
}

// 定义一个结构体Message，用于存储角色和内容
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

// 实现Data结构体的相等性比较
impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        self.model == other.model && self.messages == other.messages
    }
}
// 实现Message结构体的相等性比较
impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        self.role == other.role && self.content == other.content
    }
}

// 将Data结构体保存到文件中
pub fn save_message(data: Data) -> Result<()> {
    // 将Data结构体转换为JSON字符串
    let content = serde_json::to_string(&data).unwrap();
    // 定义文件路径
    let path = Path::new("data.json");
    // 将JSON字符串写入文件
    write(path, content)?;
    Ok(())
}

// 从文件中读取Data结构体
pub fn read_message() -> Result<Data> {
    // 定义文件路径
    let path = Path::new("data.json");
    // 从文件中读取JSON字符串
    let content = std::fs::read_to_string(path)?;
    // 将JSON字符串转换为Data结构体
    let data: Data = serde_json::from_str(&content)?;
    Ok(data)
}

// 展示文件到终端作为历史记录
pub fn show_message(data: &Data) -> Result<()> {
    for message in &data.messages[2..] {
        if message.role == "assistant" {
            println!("\n{}: {}\n", message.role.blue(), message.content.purple());
        } else {
            println!("\n{}: {}\n", message.role.green(), message.content);
        }
    }
    Ok(())
}
