#[cfg(test)]
mod tests {
    fn solution(input: &str) -> usize {
        input.len()
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solution(EXAMPLE), 0);
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(solution(INPUT), 0);
    }


    #[test]
    fn test_part2_example() {
        assert_eq!(solution(EXAMPLE), 0);
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(solution(INPUT), 0);
    }

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");
}
