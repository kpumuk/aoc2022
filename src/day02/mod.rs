use std::fmt;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug)]
enum Figure {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Outcome {
    Lost,
    Draw,
    Won,
}

trait Score {
    fn score(&self) -> u64;
}

impl Score for Figure {
    fn score(&self) -> u64 {
        match self {
            Figure::Rock => 1,
            Figure::Paper => 2,
            Figure::Scissors => 3,
        }
    }
}

impl Figure {
    fn outcome(&self, other: &Figure) -> Outcome {
        match (self, other) {
            (Figure::Rock, Figure::Scissors) => Outcome::Won,
            (Figure::Scissors, Figure::Paper) => Outcome::Won,
            (Figure::Paper, Figure::Rock) => Outcome::Won,
            (Figure::Scissors, Figure::Scissors) => Outcome::Draw,
            (Figure::Paper, Figure::Paper) => Outcome::Draw,
            (Figure::Rock, Figure::Rock) => Outcome::Draw,
            _ => Outcome::Lost,
        }
    }

    fn for_their_outcome(&self, outcome: &Outcome) -> Figure {
        match (self, outcome) {
            (Figure::Rock, Outcome::Lost) => Figure::Scissors,
            (Figure::Scissors, Outcome::Lost) => Figure::Paper,
            (Figure::Paper, Outcome::Lost) => Figure::Rock,
            (Figure::Scissors, Outcome::Draw) => Figure::Scissors,
            (Figure::Paper, Outcome::Draw) => Figure::Paper,
            (Figure::Rock, Outcome::Draw) => Figure::Rock,
            (Figure::Rock, Outcome::Won) => Figure::Paper,
            (Figure::Paper, Outcome::Won) => Figure::Scissors,
            (Figure::Scissors, Outcome::Won) => Figure::Rock,
        }
    }
}

impl From<u8> for Figure {
    fn from(value: u8) -> Self {
        match value {
            b'A' | b'X' => Figure::Rock,
            b'B' | b'Y' => Figure::Paper,
            b'C' | b'Z' => Figure::Scissors,
            _ => unimplemented!(),
        }
    }
}

impl From<u8> for Outcome {
    fn from(value: u8) -> Self {
        match value {
            b'X' => Outcome::Lost,
            b'Y' => Outcome::Draw,
            b'Z' => Outcome::Won,
            _ => unimplemented!(),
        }
    }
}

impl Score for Outcome {
    fn score(&self) -> u64 {
        match self {
            Outcome::Lost => 0,
            Outcome::Draw => 3,
            Outcome::Won => 6,
        }
    }
}

struct Rule1 {
    them: Figure,
    me: Figure,
}

impl Score for Rule1 {
    fn score(&self) -> u64 {
        self.me.outcome(&self.them).score() + self.me.score()
    }
}

impl FromStr for Rule1 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let b = s.as_bytes();
        Ok(Rule1 {
            them: Figure::from(b[0]),
            me: Figure::from(b[2]),
        })
    }
}

impl fmt::Display for Rule1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} < {:?} = {:?} ({})",
            self.them,
            self.me,
            self.me.outcome(&self.them),
            self.score()
        )
    }
}

struct Rule2 {
    them: Figure,
    outcome: Outcome,
}

impl Score for Rule2 {
    fn score(&self) -> u64 {
        self.them.for_their_outcome(&self.outcome).score() + self.outcome.score()
    }
}

impl FromStr for Rule2 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let b = s.as_bytes();
        Ok(Rule2 {
            them: Figure::from(b[0]),
            outcome: Outcome::from(b[2]),
        })
    }
}

impl fmt::Display for Rule2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let me = self.them.for_their_outcome(&self.outcome);
        write!(
            f,
            "{:?} < {:?} = {:?} ({})",
            self.them,
            me,
            me.score(),
            self.score()
        )
    }
}

fn solution1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| Rule1::from_str(line).unwrap())
        // .inspect(|rule| println!("{}", rule))
        .map(|rule| rule.score())
        .sum()
}

fn solution2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| Rule2::from_str(line).unwrap())
        // .inspect(|rule| println!("{}", rule))
        .map(|rule| rule.score())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1_example() {
        assert_eq!(solution1(EXAMPLE), 15);
    }

    #[test]
    fn test_solution1_solution() {
        assert_eq!(solution1(INPUT), 11386);
    }

    #[test]
    fn test_solution2_example() {
        assert_eq!(solution2(EXAMPLE), 12);
    }

    #[test]
    fn test_solution2_solution() {
        assert_eq!(solution2(INPUT), 13600);
    }

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");
}
