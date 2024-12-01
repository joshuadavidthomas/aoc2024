use crate::location::Locations;
use crate::Answer;
use anyhow::Result;

pub fn solve(input: &str) -> Result<Answer> {
    let locations = Locations::from_input(input);
    let distance = locations.calculate_distance();
    Ok(distance)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EXAMPLE;

    const ANSWER: Answer = 11; // Update value based on problem

    #[test]
    fn test_solve() -> Result<()> {
        assert_eq!(solve(EXAMPLE)?.to_string(), ANSWER.to_string());
        Ok(())
    }
}
