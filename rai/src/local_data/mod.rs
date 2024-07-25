mod data;
use std::path::Path;

pub use data::{Data, Message};

mod config;
pub use config::Config;

pub fn chat_init_data(path: &Path) -> Data {
    Data {
        model: "glm-4-flash".to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content:
                    "你是一个乐于解答各种问题的助手，你的任务是为用户提供专业、准确、有见地的建议"
                        .to_string(),
            },
            Message {
                role: "user".to_string(),
                content: "你好".to_string(),
            },
            Message {
                role: "assistant".to_string(),
                content: "你好👋！有什么可以帮助你的吗？我很乐意为你提供帮助。".to_string(),
            },
        ],
        path: path.to_str().unwrap().to_string(),
    }
}

pub fn teacher_init_data(path: &Path) -> Data {
    Data {
        model: "glm-4-flash".to_string(),
        messages: vec![
            Message {
                role: "system".to_string(),
                content:
                    "你是一个乐于解答学生各种问题的老师，你的任务是为专业 , 准确 , 生动的解答学生的问题"
                        .to_string(),
            },
            Message {
                role: "user".to_string(),
                content: "老师好".to_string(),
            },
            Message {
                role: "assistant".to_string(),
                content: "同学, 你好呀！有什么问题我可以帮到你吗？请随时提问，我会尽力为你提供专业、准确、生动的解答。".to_string(),
            }
        ],
        path: path.to_str().unwrap().to_string(),
    }
}
