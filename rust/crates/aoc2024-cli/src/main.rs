mod client;
mod workspace;

use anyhow::{Context, Result};
use aoc2024_macros::env_vars;
use clap::Parser;
use std::path::PathBuf;

use client::AocClient;
use workspace::{
    create_day_crate, extract_problem_description, find_next_day, find_workspace_root,
};

const AOC_BASE_URL: &str = "https://adventofcode.com/2024/day/";

#[env_vars]
enum EnvVars {
    AOC_SESSION_COOKIE,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Session cookie for Advent of Code
    #[arg(short, long, env = EnvVars::AOC_SESSION_COOKIE, value_name = "COOKIE")]
    session: String,
    /// Workspace root directory (defaults to detecting from current directory)
    #[arg(long, value_name = "DIR")]
    workspace_root: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let workspace_root = cli
        .workspace_root
        .unwrap_or_else(|| find_workspace_root().expect("Failed to detect workspace root"));

    let next_day = find_next_day(&workspace_root)?;
    println!("Setting up day {}...", next_day);

    let client = AocClient::new(cli.session);
    let problem_html = client
        .get_problem(next_day)
        .await
        .context("Failed to download problem description")?;
    let problem_md = extract_problem_description(&problem_html)
        .context("Failed to extract problem description")?;
    let input = client
        .get_input(next_day)
        .await
        .context("Failed to download input")?;

    create_day_crate(&workspace_root, next_day, &problem_md, &input)
        .context("Failed to create day crate")?;

    println!("Successfully created aoc2024-day{}", next_day);
    Ok(())
}
