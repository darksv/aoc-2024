use aoc_runner_derive::aoc;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    let mut sum = 0;

    let mut rest = input;
    while let Some(idx) = rest.find("mul(") {
        rest = &rest[idx+4..];
        let (a, len) = atoi_simd::parse_any_pos(rest.as_bytes()).unwrap_or((0, 0));
        if matches!(len, 1..=3) {
            if rest.as_bytes()[len] == b',' {
                rest = &rest[len+1..];
                let (b, len) = atoi_simd::parse_any_pos(rest.as_bytes()).unwrap_or((0, 0));
                if matches!(len, 1..=3) {
                    if rest.as_bytes()[len] == b')' {
                        rest = &rest[len+1..];

                        sum += a * b;
                    }
                }
            }
        } else if len == 0 {
            rest = &rest[1..];
        } else {
            rest = &rest[len..];
        }
    }
    sum
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::day3::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("../input/2024/day3.txt")), 189527826);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("../input/2024/day3.txt")), 0);
    }
}