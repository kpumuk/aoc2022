use std::collections::HashSet;

struct Item(u8);

impl Item {
    fn priority(&self) -> u64 {
        match self {
            Item(b'a'..=b'z') => u64::from(self.0 - b'a') + 1,
            Item(b'A'..=b'Z') => u64::from(self.0 - b'A') + 27,
            _ => unimplemented!(),
        }
    }
}

impl From<u8> for Item {
    fn from(value: u8) -> Self {
        Item(value)
    }
}

#[cfg(test)]
fn solution_part1(input: &str) -> u64 {
    let mut total = 0;
    for line in input.lines() {
        let (left, right) = line.split_at(line.len() / 2);
        let left: HashSet<u8> = HashSet::from_iter(left.bytes());
        total += right
            .bytes()
            .find(|b| left.contains(&b))
            .map(|b| Item::from(b).priority())
            .unwrap_or(0);
    }

    total
}

#[cfg(test)]
fn solution_part2(input: &str) -> u64 {
    let mut total = 0;
    let mut first = HashSet::new();
    let mut second = HashSet::new();

    for line in input.lines() {
        if first.is_empty() {
            first = HashSet::from_iter(line.bytes());
        } else if second.is_empty() {
            second = HashSet::from_iter(line.bytes());
        } else {
            total += line
                .bytes()
                .find(|b| first.contains(&b) && second.contains(&b))
                .map(|b| Item::from(b).priority())
                .unwrap_or(0);
            first.clear();
            second.clear();
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE), 157);
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(solution_part1(INPUT), 8105);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution_part2(EXAMPLE), 70);
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(solution_part2(INPUT), 2363);
    }

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");
}
