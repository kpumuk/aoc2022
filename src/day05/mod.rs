use std::str::FromStr;
use std::usize;

type Crate = char;

struct Ship {
    stacks: Vec<Vec<Crate>>,
}

impl Ship {
    pub fn new() -> Self {
        Ship { stacks: Vec::new() }
    }

    pub fn pick_crate(&mut self, from: usize) -> Option<Crate> {
        self.stacks[from - 1].pop()
    }

    pub fn put_crate(&mut self, to: usize, cr: Crate) {
        while self.stacks.len() < to {
            self.stacks.push(Vec::new());
        }
        self.stacks[to - 1].push(cr);
    }

    pub fn pick_crates(&mut self, from: usize, count: usize) -> Option<Vec<Crate>> {
        if self.stacks[from - 1].len() < count {
            return None;
        }

        let first = self.stacks[from - 1].len() - count;
        let crates: Vec<Crate> = (&self.stacks[from - 1])
            .iter()
            .skip(first)
            .cloned()
            .collect();
        self.stacks[from - 1].truncate(first);

        Some(crates)
    }

    pub fn put_crates(&mut self, to: usize, crates: &mut Vec<Crate>) {
        self.stacks[to - 1].append(crates);
    }

    pub fn top_view(&self) -> String {
        String::from_iter(self.stacks.iter().map(|stack| *stack.last().unwrap()))
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

fn solution<T>(input: &str) -> String
where
    T: CrateMover,
{
    let mut liter = input.lines();
    let mut ship = Ship::new();

    let stacks: Vec<&str> = liter.by_ref().take_while(|line| !line.is_empty()).collect();
    for line in stacks.iter().rev().skip(1) {
        for (i, cr) in line.chars().skip(1).step_by(4).enumerate() {
            if cr != ' ' {
                ship.put_crate(i + 1, cr as Crate);
            };
        }
    }
    let instructions: Vec<&str> = liter.collect();
    for instr in instructions.iter() {
        let words: Vec<&str> = instr.split(' ').collect();
        let count = usize::from_str(words[1]).unwrap();
        let from = usize::from_str(words[3]).unwrap();
        let to = usize::from_str(words[5]).unwrap();

        T::move_crates(&mut ship, from, to, count);
    }
    ship.top_view()
}

#[cfg(test)]
mod tests {
    use super::*;

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
