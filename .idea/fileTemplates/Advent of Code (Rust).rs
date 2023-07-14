fn solution(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(solution(EXAMPLE), 0);
    }

    #[test]
    fn test_solution() {
        assert_eq!(solution(INPUT), 0);
    }

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");
}
