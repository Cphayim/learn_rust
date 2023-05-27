use clap::{ColorChoice, Parser};

use crate::http::HTTPMethod;

/// A naive httpie implementation with Rust, can you imagine how easy it is?
#[derive(Parser, Debug)]
#[clap(color = ColorChoice::Always)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub method: HTTPMethod,
}
