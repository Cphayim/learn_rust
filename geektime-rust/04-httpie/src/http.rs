use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use clap::{Args, Subcommand};
use reqwest::{header, Client};

use crate::{
    parser::{self, KvPair},
    print,
};

// 子命令分别对应不同的 HTTP 方法，目前只支持 get/post
#[derive(Subcommand, Debug)]
pub enum HTTPMethod {
    Get(Get),
    Post(Post),
}

#[async_trait]
pub trait HTTPHandler {
    async fn handle(&self, client: &Client) -> Result<()>;
}

// get 子命令
/// feed get with an url and we will retrieve the response for you
#[derive(Args, Debug)]
pub struct Get {
    /// HTTP 请求的 URL
    #[arg(value_parser = parser::parse_url)]
    pub url: String,
}

#[async_trait]
impl HTTPHandler for Get {
    async fn handle(&self, client: &Client) -> Result<()> {
        let resp = client.get(&self.url).send().await?;
        Ok(print::print_resp(resp).await?)
    }
}

// post 子命令。需要输入一个 URL，和若干个可选的 key=value，用于提供 json body
/// feed post with an url and optional key=value pairs. We will post the data
/// as JSON, and retrieve the response for you
#[derive(Args, Debug)]
pub struct Post {
    /// HTTP 请求的 URL
    #[arg(value_parser = parser::parse_url)]
    pub url: String,
    /// HTTP 请求的 body
    #[arg(value_parser = parser::parse_kv_pair)]
    pub body: Vec<KvPair>,
}

#[async_trait]
impl HTTPHandler for Post {
    async fn handle(&self, client: &Client) -> Result<()> {
        let mut body = HashMap::new();
        for pair in self.body.iter() {
            body.insert(&pair.k, &pair.v);
        }
        let resp = client.post(&self.url).json(&body).send().await?;
        Ok(print::print_resp(resp).await?)
    }
}

/// 创建 http 请求客户端
pub fn create_client() -> Result<Client> {
    // let client = Client::new();
    let headers = create_default_header()?;
    let client = Client::builder().default_headers(headers).build()?;
    Ok(client)
}

/// 创建默认的 http 请求头
fn create_default_header() -> Result<header::HeaderMap> {
    let mut headers = header::HeaderMap::new();
    headers.insert("X-POWERED-BY", "Rust".parse()?);
    headers.insert(header::USER_AGENT, "Rust Httpie".parse()?);
    Ok(headers)
}
