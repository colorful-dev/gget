use anyhow::Result;
use clap::Parser;
use reqwest::Url;

/// a git file download command tool
#[derive(Debug, Parser)]
#[command(author = "color-dev", name = "gget")]
pub struct GGet {
    /// download file url
    #[arg(value_parser = parse_url)]
    pub url: String,

    /// download file name
    pub filename: Option<String>,

    /// set download file path
    #[arg(short = 'P', long)]
    pub path: Option<String>,

    /// use clashx proxy
    #[arg(long)]
    pub proxy: Option<bool>,
}

fn parse_url(s: &str) -> Result<String> {
    let url: Url = s.parse()?;
    Ok(url.into())
}

impl GGet {
    /// return after clap parse arg.
    pub fn run() -> Result<GGet> {
        Ok(GGet::parse())
    }
}
