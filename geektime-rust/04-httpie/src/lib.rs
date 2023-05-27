// 定义 HTTPie 的 CLI 的主入口，它包含若干个子命令
// 下面 /// 的注释是文档，clap 会将其作为 CLI 的帮助

mod cli;
mod http;
mod parser;
mod print;

use anyhow::{Ok, Result};
use clap::Parser;
use http::{HTTPHandler, HTTPMethod};

use cli::Cli;

pub async fn exec_cli() -> Result<()> {
    let cli = Cli::parse();

    // 生成一个 HTTP 客户端
    let client = http::create_client()?;

    let result = match cli.method {
        HTTPMethod::Get(ref get) => get.handle(&client).await?,
        HTTPMethod::Post(ref post) => post.handle(&client).await?,
    };

    Ok(result)
}
