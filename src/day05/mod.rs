use crate::day05::ParseInstructionError::{InvalidCount, InvalidFormat, InvalidFrom, InvalidTo};
use std::str::FromStr;
use std::usize;

#[derive(Clone, PartialEq)]
struct Crate(char);

impl Crate {
    const NONE: Crate = Crate(' ');
}

impl From<char> for Crate {
    fn from(value: char) -> Self {
        Crate(value)
    }
}

impl<'a> FromIterator<&'a Crate> for String {
    fn from_iter<T: IntoIterator<Item = &'a Crate>>(iter: T) -> Self {
        let mut s = String::new();
        for cr in iter {
            s.push(cr.0);
        }

        s
    }
}

struct Ship {
    stacks: Vec<Vec<Crate>>,
}

impl Ship {
    fn new() -> Self {
        Ship { stacks: Vec::new() }
    }

    fn pick_crate(&mut self, from: usize) -> Option<Crate> {
        self.stacks[from - 1].pop()
    }

    fn put_crate(&mut self, to: usize, cr: Crate) {
        while self.stacks.len() < to {
            self.stacks.push(Vec::new());
        }

        if cr != Crate::NONE {
            self.stacks[to - 1].push(cr);
        }
    }

    fn pick_crates(&mut self, from: usize, count: usize) -> Option<Vec<Crate>> {
        if self.stacks[from - 1].len() < count {
            return None;
        }

        let first = self.stacks[from - 1].len() - count;
        let crates: Vec<Crate> = self.stacks[from - 1].iter().skip(first).cloned().collect();
        self.stacks[from - 1].truncate(first);

        Some(crates)
    }

    fn put_crates(&mut self, to: usize, crates: &mut Vec<Crate>) {
        self.stacks[to - 1].append(crates);
    }

    fn top_view(&self) -> String {
        String::from_iter(
            self.stacks
                .iter()
                .map(|stack| stack.last().unwrap_or(&Crate::NONE)),
        )
    }

    fn load(&mut self, layout: Vec<&str>) {
        for line in layout.into_iter().rev().skip(1) {
            for (i, cr) in line.chars().skip(1).step_by(4).enumerate() {
                self.put_crate(i + 1, Crate::from(cr));
            }
        }
    }
}

trait CrateMover {
    fn move_crates(ship: &mut Ship, from: usize, to: usize, count: usize);
}

struct CrateMover9000;

impl CrateMover for CrateMover9000 {
    fn move_crates(ship: &mut Ship, from: usize, to: usize, count: usize) {
        for _ in 0..count {
            let cr = ship.pick_crate(from).unwrap();
            ship.put_crate(to, cr);
        }
    }
}

struct CrateMover9001;

impl CrateMover for CrateMover9001 {
    fn move_crates(ship: &mut Ship, from: usize, to: usize, count: usize) {
        let mut crates = ship.pick_crates(from, count).unwrap();
        ship.put_crates(to, &mut crates);
    }
}

#[derive(Debug)]
struct Instruction {
    from: usize,
    to: usize,
    count: usize,
}

#[derive(Debug)]
enum ParseInstructionError {
    InvalidFormat(String),
    InvalidCount(String),
    InvalidFrom(String),
    InvalidTo(String),
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split(' ').collect();
        if words.len() != 6 || words[0] != "move" || words[2] != "from" || words[4] != "to" {
            return Err(InvalidFormat(format!("Invalid instruction format {:?}", s)));
        }
        let count = usize::from_str(words[1])
            .map_err(|_| InvalidCount(format!("Invalid crates count in {:?} instruction", s)))?;
        let from = usize::from_str(words[3]).map_err(|_| {
            InvalidFrom(format!(
                "Invalid crate starting position in {:?} instruction",
                s
            ))
        })?;
        let to = usize::from_str(words[5]).map_err(|_| {
            InvalidTo(format!(
                "Invalid crate target position in {:?} instruction",
                s
            ))
        })?;

        Ok(Instruction { count, from, to })
    }
}

fn solution<T>(input: &str) -> String
where
    T: CrateMover,
{
    let mut liter = input.lines();
    let mut ship = Ship::new();

    ship.load(liter.by_ref().take_while(|line| !line.is_empty()).collect());

    for instruction in liter {
        let instruction = Instruction::from_str(instruction).unwrap();

        T::move_crates(
            &mut ship,
            instruction.from,
            instruction.to,
            instruction.count,
        );
    }
    ship.top_view()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ship_empty_stack() {
        let mut ship = Ship::new();
        ship.put_crate(2, Crate('A'));
        assert_eq!(ship.top_view(), " A");
    }

    #[test]
    fn test_ship_put_empty_crate() {
        let mut ship = Ship::new();
        ship.put_crate(2, Crate('A'));
        ship.put_crate(3, Crate(' '));
        assert_eq!(ship.top_view(), " A ");
        ship.put_crate(3, Crate('B'));
        assert_eq!(ship.top_view(), " AB");
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solution::<CrateMover9000>(EXAMPLE), "CMZ");
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(solution::<CrateMover9000>(INPUT), "LBLVVTVLP");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution::<CrateMover9001>(EXAMPLE), "MCD");
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(solution::<CrateMover9001>(INPUT), "TPFFBDRJD");
    }

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");
}
