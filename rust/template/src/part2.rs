use crate::Answer;
use anyhow::Result;

pub fn solve(input: &str) -> Result<Answer> {
    todo!("Implement part 2")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EXAMPLE;

    const ANSWER: Answer = 0; // Update value based on problem

    #[test]
    fn test_solve() -> Result<()> {
        assert_eq!(solve(EXAMPLE)?.to_string(), ANSWER.to_string());
        Ok(())
    }
}
