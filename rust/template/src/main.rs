mod part1;
mod part2;

use anyhow::Result;

pub const EXAMPLE: &str = ""; // Add example from problem description
pub type Answer = i32; // Update type based on problem

fn main() -> Result<()> {
    let input = include_str!("../input.txt");

    println!("Part 1: {}", part1::solve(input)?);
    println!("Part 2: {}", part2::solve(input)?);

    Ok(())
}
