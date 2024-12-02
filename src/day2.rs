use aoc_runner_derive::aoc;

#[derive(PartialOrd, PartialEq, Copy, Clone, Debug)]
enum Order {
    Ascending,
    Descending,
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    let mut safe_count = 0;
    for line in input.lines() {
        let numbers = line
            .split_ascii_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .map_windows(|&[x, y]| x.checked_signed_diff(y).unwrap());

        if first_invalid(numbers).is_none() {
            safe_count += 1;
        }
    }

    safe_count
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
    let mut safe_count = 0;
    'main: for line in input.lines() {
        let original_numbers: arrayvec::ArrayVec<_, 10> = line
            .split_ascii_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .map_windows(|&[x, y]| x.checked_signed_diff(y).unwrap())
            .collect();

        if let Some(_) = first_invalid(original_numbers.iter().copied()) {
            for offset in 0..original_numbers.len() {
                let mut numbers = original_numbers.clone();
                numbers.remove(offset);
                if first_invalid(numbers.iter().copied()).is_none() {
                    safe_count += 1;
                    continue 'main;
                }
            }
        } else {
            safe_count += 1;
            continue;
        }
    }

    safe_count
}


fn first_invalid(diffs: impl Iterator<Item=i32>) -> Option<usize> {
    let mut dir = None;
    for (idx, diff) in diffs.enumerate() {
        match diff.abs() {
            1..=3 => (),
            _ => return Some(idx),
        }
        let order = if diff > 0 {
            Order::Ascending
        } else {
            Order::Descending
        };

        if let Some(d) = dir {
            if d != order {
                return Some(idx);
            }
        } else {
            dir = Some(order);
        }
    }

    None
}