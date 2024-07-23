use std::io::{self, Write};

use anyhow::Result;
use colored::Colorize;
use rai::{get_content, read_message, show_message, write_content_to_file, Message};

#[tokio::main]
async fn main() -> Result<()> {
    // 读取曾经的对话内容
    let mut data = read_message()?;

    // 判断记录中最后一次输入是不是 user , 是的话直接发起请求
    let last_message = &data.messages[&data.messages.len() - 1];
    if last_message.role == "user" {
        let response = get_content(&data).await?;
        // 将返回写入文件
        write_content_to_file(
            &mut data,
            Message {
                role: "assistant".to_string(),
                content: response,
            },
        )?;
    }

    // 显示所有记录
    show_message(&data).await?;
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
        let response = get_content(&data).await?;

        // 输出响应内容
        println!("\n{}:{}\n", "assistant".blue(), response.purple());

        // 将返回写入文件
        write_content_to_file(
            &mut data,
            Message {
                role: "assistant".to_string(),
                content: response,
            },
        )?;
    }
}

// todo 添加角色分类 ,
// todo 并且有清空历史记录的功能,
// todo 运行时验证文件是否存在
// todo 命令行参数
//      todo 选择角色
//      todo 清空历史记录
//      todo 清空指定角色的历史记录
