#[cfg(test)]
mod tests {
    use std::str::FromStr;

    struct Range {
        from: u64,
        to: u64,
    }

    impl Range {
        fn fully_contains(&self, other: &Self) -> bool {
            self.from <= other.from && self.to >= other.to
        }

        fn overlaps(&self, other: &Self) -> bool {
            self.to >= other.from && other.to >= self.from
        }
    }

    impl FromStr for Range {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let parts: Vec<&str> = s.split('-').collect();
            if parts.len() != 2 {
                return Err(());
            }
            let from = u64::from_str(parts[0]).map_err(|_| ())?;
            let to = u64::from_str(parts[1]).map_err(|_| ())?;
            Ok(Range { from, to })
        }
    }

    struct Pair(Range, Range);

    impl Pair {
        fn fully_contains(&self) -> bool {
            self.0.fully_contains(&self.1) || self.1.fully_contains(&self.0)
        }
        fn overlaps(&self) -> bool {
            self.0.overlaps(&self.1)
        }
    }

    impl FromStr for Pair {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let parts: Vec<&str> = s.split(',').collect();
            if parts.len() != 2 {
                return Err(());
            }
            let p1 = Range::from_str(parts[0]).map_err(|_| ())?;
            let p2 = Range::from_str(parts[1]).map_err(|_| ())?;
            Ok(Pair(p2, p1))
        }
    }

    fn solution_part1(input: &str) -> usize {
        let rules: Result<Vec<Pair>, <Pair as FromStr>::Err> =
            input.lines().map(Pair::from_str).collect();
        match rules {
            Ok(rows) => rows.iter().filter(|pair| pair.fully_contains()).count(),
            Err(()) => panic!("Error occurred"),
        }
    }

    fn solution_part2(input: &str) -> usize {
        let rules: Result<Vec<Pair>, <Pair as FromStr>::Err> =
            input.lines().map(Pair::from_str).collect();
        match rules {
            Ok(rows) => rows.iter().filter(|pair| pair.overlaps()).count(),
            Err(()) => panic!("Error occurred"),
        }
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(solution_part1(EXAMPLE), 2);
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(solution_part1(INPUT), 496);
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution_part2(EXAMPLE), 4);
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(solution_part2(INPUT), 847);
    }

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");
}
