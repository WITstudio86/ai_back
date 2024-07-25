use anyhow::Result;
use reqwest::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    Response,
};

use crate::Data;

use super::response_type::ResponseType;

pub struct Gpt {
    pub api_key: String,
    pub api_url: &'static str,
}

impl Gpt {
    pub fn new(api_key: String, api_url: &'static str) -> Self {
        Self { api_key, api_url }
    }

    async fn send(&self, data: &Data) -> Result<Response> {
        // 创建 header
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", self.api_key).parse().unwrap(),
        );
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());

        // 发送 HTTP 请求
        let client = reqwest::Client::new();
        let response = client
            .post(self.api_url)
            .headers(headers)
            .json(&data)
            .send()
            .await
            .expect("Failed to send request");

        Ok(response)
    }

    pub async fn get_rsp_content(&self, data: &Data) -> Result<String> {
        let response = self.send(data).await?;
        // 判断一下 status 是不是 401
        if response.status().as_u16() == 401 {
            return Ok("Token 已经失效 , 购买时长后继续使用".to_string());
        }
        let text = response.text().await?;
        let content: ResponseType = serde_json::from_str(&text)?;
        Ok(content.choices[0].message.content.clone())
    }
}
