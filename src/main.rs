use std::io::{self, Write};

use anyhow::Result;
use clap::Parser;
use rai::{chat_with, Arg, Command, Figure};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Arg::try_parse()?;
    println!("args: {:?}", &args);
    // 获取指令
    match args.cmd {
        Command::Clear(target) => {
            // 清空历史记录
            print!("⚠️ clear {}'s history?(y/n)", target);
            io::stdout().flush()?; // 确保打印的内容立即显示
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            let input = input.trim().to_string().to_lowercase();
            if input == "y" {
                // 清空历史记录
                match target.target {
                    Figure::Chat => {
                        // 删除对应文件
                        std::fs::remove_file("data/chat.json")?;
                    }
                    Figure::Teacher => {
                        // 删除对应文件
                        std::fs::remove_file("data/teacher.json")?;
                    }
                    Figure::All => {
                        // 删除对应路径
                        std::fs::remove_dir_all("data")?;
                    }
                }
            } else {
                println!("clear history cancel");
            }
        }
        Command::Chat(target) => {
            // 匹配人物
            match target.target {
                Figure::Chat => {
                    chat_with(Figure::Chat).await?;
                }
                Figure::Teacher => {
                    chat_with(Figure::Teacher).await?;
                }
                Figure::All => {
                    chat_with(Figure::Chat).await?;
                }
            }
        }
    }
    return Ok(());
}

// todo 命令行参数
//      todo 鉴权码
//      todo gui
//      todo 更新系统
//      todo 优化同步请求为流请求 , 优化用户体验
//      todo 跨平台
