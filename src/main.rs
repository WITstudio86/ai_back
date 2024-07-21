use anyhow::Result;
use rai::{get_content, read_message};

#[tokio::main]
async fn main() -> Result<()> {
    // 读取曾经的对话内容
    let data = read_message()?;

    let response = get_content(data).await?;
    println!("{:#?}", response);

    Ok(())
}

#[cfg(test)]
mod test {
    use rai::{Data, Message};

    use super::*;
    #[test]
    fn test_read_messagee() {
        let data = read_message().unwrap();
        let target = Data{
            model: "glm-4-flash".to_string(),
            messages: vec![
                Message{
                    role:"system".to_string(),
                    content:"你是一个乐于解答各种问题的助手，你的任务是为用户提供专业、准确、有见地的建议".to_string()
                },
                Message{
                    role:"user".to_string(),
                    content:"你好".to_string()
                }]
        };

        assert_eq!(data, target);
    }
}
