use anyhow::Result;
use reqwest::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    Response,
};
use response_type::ResponseType;
mod response_type;

use crate::{Data, Message, API_KEY, API_URL};

async fn send(data: Data) -> Result<Response> {
    // 创建 header
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", API_KEY).parse().unwrap(),
    );
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

    // 发送 HTTP 请求
    let client = reqwest::Client::new();
    let response = client
        .post(API_URL)
        .headers(headers)
        .json(&data)
        .send()
        .await
        .expect("Failed to send request");
    Ok(response)
}

pub async fn get_content(data: &Data) -> Result<String> {
    let data = data.clone();
    let response = send(data).await?;
    let text = response.text().await?;
    let content: ResponseType = serde_json::from_str(&text)?;
    Ok(content.choices[0].message.content.clone())
}

pub fn write_content_to_file(data: &mut Data, content: Message) -> Result<()> {
    data.messages.push(content);
    let text = serde_json::to_string_pretty(data)?;
    std::fs::write("data.json", text)?;
    Ok(())
}
