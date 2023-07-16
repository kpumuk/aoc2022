use std::str::FromStr;
use std::usize;

enum Version {
    CrateMover9000,
    CrateMover9001,
}

fn solution(input: &str, version: &Version) -> String {
    let mut liter = input.lines();
    let stacks: Vec<&str> = liter.by_ref().take_while(|line| !line.is_empty()).collect();
    let mut cranes = Vec::<Vec<char>>::new();
    for line in stacks.iter().rev().skip(1) {
        for (i, cr) in line.chars().skip(1).step_by(4).enumerate() {
            if cranes.len() <= i {
                cranes.push(Vec::<char>::new())
            }
            if cr != ' ' {
                cranes[i].push(cr)
            };
        }
    }
    let instructions: Vec<&str> = liter.collect();
    for instr in instructions.iter() {
        let words: Vec<&str> = instr.split(' ').collect();
        let cnt = usize::from_str(words[1]).unwrap();
        let from = usize::from_str(words[3]).unwrap() - 1;
        let to = usize::from_str(words[5]).unwrap() - 1;

        let mut tmp = Vec::<char>::new();
        for _ in 0..cnt {
            tmp.push(cranes[from].pop().unwrap())
        }
        if let Version::CrateMover9001 = version {
            tmp.reverse();
        }
        for i in 0..cnt {
            cranes[to].push(tmp[i]);
        }
    }
    String::from_iter(cranes.iter().map(|crane| *crane.last().unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        assert_eq!(solution(EXAMPLE, &Version::CrateMover9000), "CMZ");
    }

    #[test]
    fn test_part1_solution() {
        assert_eq!(solution(INPUT, &Version::CrateMover9000), "LBLVVTVLP");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solution(EXAMPLE, &Version::CrateMover9001), "MCD");
    }

    #[test]
    fn test_part2_solution() {
        assert_eq!(solution(INPUT, &Version::CrateMover9001), "TPFFBDRJD");
    }

    const EXAMPLE: &str = include_str!("example.txt");
    const INPUT: &str = include_str!("input.txt");
}
