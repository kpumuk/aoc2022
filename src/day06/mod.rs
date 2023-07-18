#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    fn solution(input: &str, marker_size: usize) -> usize {
        input
            .chars()
            .collect::<Vec<char>>()
            .windows(marker_size)
            .enumerate()
            .find(|(_, group)| HashSet::<&char>::from_iter(*group).len() == marker_size)
            .unwrap()
            .0
            + marker_size
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solution(EXAMPLE, 4), 7);
    }

    #[test]
    fn test_part1_example2() {
        assert_eq!(solution("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
    }

    #[test]
    fn test_part1_example3() {
        assert_eq!(solution("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
    }

    #[test]
    fn test_part1_example4() {
        assert_eq!(solution("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
    }

    #[test]
    fn test_part1_example5() {
        assert_eq!(solution("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(solution(INPUT, 4), 1702);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution(EXAMPLE, 14), 19);
    }

    #[test]
    fn test_part2_example2() {
        assert_eq!(solution("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
    }

    #[test]
    fn test_part2_example3() {
        assert_eq!(solution("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
    }

    #[test]
    fn test_part2_example4() {
        assert_eq!(solution("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
    }

    #[test]
    fn test_part2_example5() {
        assert_eq!(solution("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(solution(INPUT, 14), 3559);
    }

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");
}
