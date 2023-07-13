use std::str::FromStr;

fn day1(input: &str, days: usize) -> u64 {
    let mut elves = Vec::<u64>::new();
    for i in input.lines() {
        match i {
            "" => elves.push(0),
            _ => {
                let last_elve = elves.pop().unwrap_or(0);
                let current_value = u64::from_str(i).unwrap_or(0);
                elves.push(last_elve + current_value)
            }
        }
    }
    elves.sort();
    elves.reverse();
    elves[..days].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(day1(EXAMPLE, 1), 24000);
    }

    #[test]
    fn part1_question() {
        assert_eq!(day1(INPUT, 1), 75501);
    }

    #[test]
    fn part2_example() {
        assert_eq!(day1(EXAMPLE, 3), 45000);
    }

    #[test]
    fn part2_question() {
        assert_eq!(day1(INPUT, 3), 215594);
    }

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");
}
