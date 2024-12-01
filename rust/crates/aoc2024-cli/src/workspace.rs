use anyhow::Result;
use chrono::{Datelike, Local, TimeZone};
use html2md::parse_html;
use scraper::{Html, Selector};
use std::fs;
use std::path::{Path, PathBuf};

pub fn find_workspace_root() -> Result<PathBuf> {
    let mut current_dir = std::env::current_dir()?;

    loop {
        // Check for Cargo.toml
        let cargo_toml = current_dir.join("Cargo.toml");
        if cargo_toml.exists() {
            // Verify it's a workspace by checking its contents
            let contents = std::fs::read_to_string(cargo_toml)?;
            if contents.contains("[workspace]") {
                return Ok(current_dir);
            }
        }

        // Go up one directory
        if !current_dir.pop() {
            anyhow::bail!("Could not find workspace root. Please specify with --workspace-root");
        }
    }
}

pub fn find_next_day(workspace_root: &Path) -> Result<u32> {
    let mut max_day = 0;

    for entry in std::fs::read_dir(workspace_root.join("crates"))? {
        let entry = entry?;
        let file_name = entry.file_name().to_string_lossy().to_string();
        if file_name.starts_with("aoc2024-day") {
            if let Ok(day) = file_name["aoc2024-day".len()..].parse::<u32>() {
                max_day = max_day.max(day);
            }
        }
    }

    // Check if the next day is available (AoC releases at midnight EST)
    let now = Local::now();
    let est_now = now.with_timezone(&chrono_tz::America::New_York);
    let december_first = chrono_tz::America::New_York
        .with_ymd_and_hms(2024, 12, 1, 0, 0, 0)
        .unwrap();

    if est_now < december_first {
        anyhow::bail!("Advent of Code 2024 hasn't started yet!");
    }

    let current_day = est_now.day();
    let next_day = max_day + 1;

    if next_day > current_day {
        anyhow::bail!("Next day ({}) is not available yet!", next_day);
    }

    Ok(next_day)
}

pub fn extract_problem_description(html: &str) -> Result<String> {
    let document = Html::parse_document(html);
    let selector = Selector::parse("article.day-desc").unwrap();

    let articles: Vec<_> = document.select(&selector).collect();
    if articles.is_empty() {
        anyhow::bail!("Could not find problem description");
    }

    let markdown = articles
        .iter()
        .map(|article| parse_html(&article.inner_html()))
        .collect::<Vec<_>>()
        .join("\n\n---\n\n");

    Ok(markdown)
}

pub fn create_day_crate(
    workspace_root: &Path,
    day: u32,
    description: &str,
    input: &str,
) -> Result<()> {
    let crate_name = format!("aoc2024-day{:02}", day);
    let template_path = workspace_root.join("template");
    let crate_path = workspace_root.join("crates").join(&crate_name);

    if template_path.exists() {
        copy_dir_all(&template_path, &crate_path)?;
    } else {
        anyhow::bail!(
            "Template directory not found at {}. Please create a template first.",
            template_path.display()
        );
    }

    let cargo_toml_path = crate_path.join("Cargo.toml");
    let cargo_toml = fs::read_to_string(&cargo_toml_path)?.replace("{{crate_name}}", &crate_name);
    fs::write(cargo_toml_path, cargo_toml)?;

    fs::write(crate_path.join("PROBLEM.md"), description)?;

    fs::write(crate_path.join("input.txt"), input)?;

    println!("Created new crate at {}", crate_path.display());
    Ok(())
}

fn copy_dir_all(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if ty.is_dir() {
            copy_dir_all(&src_path, &dst_path)?;
        } else {
            fs::copy(src_path, dst_path)?;
        }
    }
    Ok(())
}
