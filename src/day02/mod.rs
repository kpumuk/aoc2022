use std::fmt;
use std::fmt::{Debug, Display};
use std::str::FromStr;

#[derive(Debug)]
enum ParseError {
    InvalidRule(String),
    UnexpectedFigureToken(u8, String),
    UnexpectedOutcomeToken(u8, String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidRule(rule) => {
                write!(f, "The rule {:?} is invalid. Expected a letter (A-C) followed by a letter (A-C or X-Z), separated with space", rule)
            }
            ParseError::UnexpectedFigureToken(b, rule) => {
                write!(
                    f,
                    "Unexpected figure token {:?} in rule {:?}, expected one of A-C or X-Z",
                    *b as char, rule
                )
            }
            ParseError::UnexpectedOutcomeToken(b, rule) => {
                write!(
                    f,
                    "Unexpected outcome token {:?} in rule {:?}, expected one of X-Z",
                    *b as char, rule
                )
            }
        }
    }
}

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

impl TryFrom<u8> for Figure {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'A' | b'X' => Ok(Figure::Rock),
            b'B' | b'Y' => Ok(Figure::Paper),
            b'C' | b'Z' => Ok(Figure::Scissors),
            _ => Err(()),
        }
    }
}

impl TryFrom<u8> for Outcome {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'X' => Ok(Outcome::Lost),
            b'Y' => Ok(Outcome::Draw),
            b'Z' => Ok(Outcome::Won),
            _ => Err(()),
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
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let b = s.as_bytes();
        if b.len() != 3 || b[1] != b' ' {
            return Err(ParseError::InvalidRule(s.to_owned()));
        }
        let (them, me) = (b[0], b[2]);
        Ok(Rule1 {
            them: Figure::try_from(them)
                .map_err(|_| ParseError::UnexpectedFigureToken(them, s.to_owned()))?,
            me: Figure::try_from(me)
                .map_err(|_| ParseError::UnexpectedFigureToken(me, s.to_owned()))?,
        })
    }
}

impl Display for Rule1 {
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
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let b = s.as_bytes();
        if b.len() != 3 || b[1] != b' ' {
            return Err(ParseError::InvalidRule(s.to_owned()));
        }
        Ok(Rule2 {
            them: Figure::try_from(b[0])
                .map_err(|_| ParseError::UnexpectedFigureToken(b[0], s.to_owned()))?,
            outcome: Outcome::try_from(b[2])
                .map_err(|_| ParseError::UnexpectedOutcomeToken(b[2], s.to_owned()))?,
        })
    }
}

impl Display for Rule2 {
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

#[cfg(test)]
fn solution<T>(input: &str) -> u64
where
    T: Score + FromStr,
    <T as FromStr>::Err: Display,
{
    let rules: Result<Vec<T>, <T as FromStr>::Err> =
        input.lines().map(|line| T::from_str(line)).collect();
    match rules {
        Ok(rows) => rows.iter().map(Score::score).sum(),
        Err(err) => {
            panic!("Error occurred: {}", err);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1_example() {
        assert_eq!(solution::<Rule1>(EXAMPLE), 15);
    }

    #[test]
    fn test_solution1_solution() {
        assert_eq!(solution::<Rule1>(INPUT), 11386);
    }

    #[test]
    fn test_solution2_example() {
        assert_eq!(solution::<Rule2>(EXAMPLE), 12);
    }

    #[test]
    fn test_solution2_solution() {
        assert_eq!(solution::<Rule2>(INPUT), 13600);
    }

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");
}
