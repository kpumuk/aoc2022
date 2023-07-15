#[cfg(test)]
fn solution(input: &str, days: usize) -> u64 {
    let mut elves = vec![0];
    for calories in input.lines().map(|v| v.parse().unwrap_or(0)) {
        match calories {
            0 => elves.push(0),
            _ => {
                elves.last_mut().map(|elf| *elf += calories);
            }
        }
    }
    elves.sort();
    elves[elves.len() - days..].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_single_elve() {
        assert_eq!(solution("5", 1), 5);
    }

    #[test]
    fn part1_first_elve_wins() {
        assert_eq!(solution("5\n\n4", 1), 5);
    }

    #[test]
    fn part1_example() {
        assert_eq!(solution(EXAMPLE, 1), 24000);
    }

    #[test]
    fn part1_question() {
        assert_eq!(solution(INPUT, 1), 75501);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solution(EXAMPLE, 3), 45000);
    }

    #[test]
    fn part2_question() {
        assert_eq!(solution(INPUT, 3), 215594);
    }

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");
}
