#[cfg(test)]
mod tests {
    fn solution(input: &str, days: usize) -> u64 {
        let mut elves = vec![0];
        for calories in input.lines().map(|v| v.parse().unwrap_or(0)) {
            match calories {
                0 => elves.push(0),
                _ => {
                    if let Some(elf) = elves.last_mut() {
                        *elf += calories
                    }
                }
            }
        }
        elves.sort();
        elves[elves.len() - days..].iter().sum()
    }

    #[test]
    fn part1_single_elf() {
        assert_eq!(solution("5", 1), 5);
    }

    #[test]
    fn part1_first_elf_wins() {
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
