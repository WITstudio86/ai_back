use anyhow::Result;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs::write;
use std::path::Path;

use crate::Figure;

use super::{chat_init_data, teacher_init_data};

// 定义一个结构体Data，用于存储模型和消息
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    pub model: String,
    pub messages: Vec<Message>,
    pub path: String,
}

// 定义一个结构体Message，用于存储角色和内容
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl Data {
    // 从文件中读取Data结构体并返回一个 Data
    pub fn new_from_file(path: &Path, figure: Figure) -> Result<Self> {
        // 验证文件
        Self::verify_file(path, figure)?;

        // 从文件中读取JSON字符串
        let content = std::fs::read_to_string(path)?;
        // 将JSON字符串转换为Data结构体
        let data: Data = serde_json::from_str(&content)?;
        Ok(data)
    }
    // 验证文件是否存在，如果不存在则创建一个默认的Data对象并保存到文件中
    fn verify_file(path: &Path, figure: Figure) -> Result<()> {
        // 先确定目标路径所在文件夹是否存在
        let parent = path.parent().unwrap();
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
        if !path.exists() {
            let data = match figure {
                Figure::Chat => chat_init_data(path),
                Figure::Teacher => teacher_init_data(path),
                Figure::All => chat_init_data(path),
            };
            data.save_to_file()?;
        }
        Ok(())
    }

    // 将Data结构体保存到文件中
    pub fn save_to_file(&self) -> Result<()> {
        // 将Data结构体转换为JSON字符串
        let content = serde_json::to_string(&self).unwrap();
        // 定义文件路径
        let path = Path::new(&self.path);
        // 将JSON字符串写入文件
        write(path, content)?;
        Ok(())
    }

    // 展示消息记录到终端作为历史记录
    pub async fn show_message(&self) -> Result<()> {
        for message in &self.messages[2..] {
            if message.role == "assistant" {
                println!("\n{}: {}\n", message.role.blue(), message.content.purple());
            } else if message.role == "user" {
                println!("\n{}: {}\n", message.role.green(), message.content);
            }
        }
        Ok(())
    }

    pub fn add_message(&mut self, message: Message) -> Result<()> {
        self.messages.push(message);
        self.save_to_file()?;
        Ok(())
    }
}
