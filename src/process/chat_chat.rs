use std::{
    io::{self, Write},
    path::Path,
};

use anyhow::Result;
use colored::Colorize;

use crate::{Config, Data, Figure, Gpt, Message, API_URL};

pub async fn chat_with(figure: Figure) -> Result<()> {
    let config = Config::from_toml("rai.toml");
    let token = config.get_token().to_string();
    // 创建 gpt
    let gpt = Gpt::new(token, API_URL);
    let path = match figure {
        Figure::Chat => Path::new("data/chat.json"),
        Figure::Teacher => Path::new("data/teacher.json"),
        Figure::All => Path::new("data/chat.json"),
    };
    // 读取曾经的对话内容
    // 定义文件路径

    // 从文件创建 path ,如果没有文件会直接新建
    let mut data = Data::new_from_file(path, figure)?;

    // 初始化一下消息存储 , 如果内部消息最后一条是用户提问 就先获取一次回复
    if let "user" = data.messages.last().unwrap().role.as_str() {
        let response = gpt.get_rsp_content(&data).await?;
        data.add_message(Message {
            role: "assistant".to_string(),
            content: response,
        })?;
    }

    // 显示所有记录
    data.show_message().await?;

    // 进入主循环不断询问
    loop {
        // 获取用户新的输入
        print!("{}:", "user".green());
        io::stdout().flush()?; // 确保打印的内容立即显示
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let input = input.trim().to_string();

        // 将新输入写入到 data 中
        data.messages.push(Message {
            role: "user".to_string(),
            content: input,
        });

        // 获取响应的内容
        let response = gpt.get_rsp_content(&data).await?;

        // 输出响应内容
        println!("\n{}:{}\n", "assistant".blue(), response.purple());

        data.add_message(Message {
            role: "assistant".to_string(),
            content: response,
        })?;
    }
}
