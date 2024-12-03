use aoc_runner_derive::aoc;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    let mut sum = 0;

    let mut rest = input;
    while let Some(idx) = rest.find("mul(") {
        let (mul, r) = parse_mul(rest, idx);
        if let Some(mul) = mul {
            sum += mul;
        }
        rest = r;
    }
    sum
}

fn parse_mul(input: &str, offset: usize) -> (Option<u32>, &str) {
    let mut rest = &input[offset + 4..];
    let (a, len) = atoi_simd::parse_any_pos(rest.as_bytes()).unwrap_or((0, 0));
    if matches!(len, 1..=3) {
        if rest.as_bytes()[len] == b',' {
            rest = &rest[len + 1..];
            let (b, len) = atoi_simd::parse_any_pos(rest.as_bytes()).unwrap_or((0, 0));
            if matches!(len, 1..=3) {
                if rest.as_bytes()[len] == b')' {
                    rest = &rest[len + 1..];
                    return (Some(a * b), rest);
                }
            }
        } else {
            return (None, rest);
        }
    } else if len == 0 {
        rest = &rest[1..];
    } else {
        rest = &rest[len..];
    }

    return (None, rest);
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    let mut sum = 0;
    let mut rest = input;
    let mut enabled = true;
    loop {
        let mul_idx = rest.find("mul(");
        let do_idx = rest.find("do()");
        let dont_idx = rest.find("don't()");

        match (mul_idx, do_idx, dont_idx) {
            (Some(mul_idx), Some(do_idx), Some(dont_idx)) => {
                if mul_idx < do_idx && mul_idx < dont_idx {
                    let (mul, r) = parse_mul(rest, mul_idx);
                    if let Some(mul) = mul {
                        if enabled {
                            sum += mul;
                        }
                    }
                    rest = r;
                } else if do_idx < mul_idx && do_idx < dont_idx {
                    rest = &rest[do_idx + 4..];
                    enabled = true;
                    continue;
                } else if dont_idx < mul_idx && dont_idx < do_idx {
                    rest = &rest[dont_idx + 6..];
                    enabled = false;
                    continue;
                }
            }
            (None, Some(do_idx), Some(dont_idx)) => {
                if do_idx < dont_idx {
                    rest = &rest[do_idx + 4..];
                    enabled = true;
                    continue;
                } else {
                    rest = &rest[dont_idx + 6..];
                    enabled = false;
                    continue;
                }
            }
            (Some(mul_idx), None, Some(dont_idx)) => {
                if mul_idx < dont_idx {
                    let (mul, r) = parse_mul(rest, mul_idx);
                    if let Some(mul) = mul {
                        if enabled {
                            sum += mul;
                        }
                    }
                    rest = r;
                } else {
                    rest = &rest[mul_idx + 6..];
                    enabled = false;
                    continue;
                }
            }
            (Some(mul_idx), Some(do_idx), None) => {
                if mul_idx < do_idx {
                    let (mul, r) = parse_mul(rest, mul_idx);
                    if let Some(mul) = mul {
                        if enabled {
                            sum += mul;
                        }
                    }
                    rest = r;
                } else {
                    rest = &rest[do_idx + 4..];
                    enabled = true;
                    continue;
                }
            }
            (Some(_), None, None) if !enabled => break,
            _ => break,
        }
    }

    sum
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
        assert_eq!(part2(include_str!("../input/2024/day3.txt")), 63013756);
    }
}
