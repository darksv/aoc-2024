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

#[derive(Clone, Copy)]
enum Op {
    Do,
    Dont,
    Mul,
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    let mut sum = 0;
    let mut rest = input;
    let mut enabled = true;
    loop {
        if let Some(idx) = memchr::memchr2(b'd', b'm', rest.as_bytes()) {
            rest = &rest[idx..];
        } else {
            break;
        }

        let (op, idx) = if !enabled {
            // fast skip to mul enable op
            if let Some(idx) = rest.find("do()") {
                (Op::Do, idx)
            } else {
                break;
            }
        } else {
            let mul_idx = rest.find("mul(");
            if mul_idx.is_none() {
                // no more muls
                break;
            }

            let dont_idx = rest.find("don't()");
            let do_idx = rest.find("do()");

            let op = match (mul_idx, do_idx, dont_idx) {
                (Some(0), _, _) => Op::Mul,
                (_, Some(0), _) => Op::Do,
                (_, _, Some(0)) => Op::Dont,
                (Some(mul_idx), Some(do_idx), Some(dont_idx)) => {
                    if mul_idx < do_idx && mul_idx < dont_idx {
                        Op::Mul
                    } else if do_idx < mul_idx && do_idx < dont_idx {
                        Op::Do
                    } else {
                        Op::Dont
                    }
                }
                (Some(mul_idx), None, Some(dont_idx)) => {
                    if mul_idx < dont_idx {
                        Op::Mul
                    } else {
                        Op::Dont
                    }
                }
                (Some(mul_idx), Some(do_idx), None) => {
                    if mul_idx < do_idx {
                        Op::Mul
                    } else {
                        Op::Do
                    }
                }
                _ => unreachable!(),
            };

            (op, match op {
                Op::Do => do_idx,
                Op::Dont => dont_idx,
                Op::Mul => mul_idx
            }.unwrap())
        };

        match op {
            Op::Do => {
                rest = &rest[idx + 4..];
                enabled = true;
            }
            Op::Dont => {
                rest = &rest[idx + 6..];
                enabled = false;
            }
            Op::Mul => {
                let (mul, r) = parse_mul(rest, idx);
                if let Some(mul) = mul {
                    if enabled {
                        sum += mul;
                    }
                }
                rest = r;
            }
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
