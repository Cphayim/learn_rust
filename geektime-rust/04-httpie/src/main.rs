use anyhow::{Ok, Result};

#[tokio::main]
async fn main() -> Result<()> {
    httpie::exec_cli().await?;
    Ok(())
}
