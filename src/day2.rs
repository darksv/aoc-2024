use aoc_runner_derive::aoc;

#[derive(PartialOrd, PartialEq, Copy, Clone, Debug)]
enum Order {
    Ascending,
    Descending,
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    let mut safe_count = 0;
    for line in input.split_terminator('\n') {
        let numbers: arrayvec::ArrayVec<_, 8> = line
            .split_ascii_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        if first_invalid(&numbers).is_none() {
            safe_count += 1;
        }
    }

    safe_count
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
    let mut safe_count = 0;
    'main: for line in input.split_terminator('\n') {
        let original_numbers: arrayvec::ArrayVec<_, 8> = line
            .split_ascii_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        if let Some(_) = first_invalid(&original_numbers) {
            for offset in 0..original_numbers.len() {
                let mut numbers = original_numbers.clone();
                numbers.remove(offset);
                if first_invalid(&numbers).is_none() {
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


fn first_invalid(numbers: &[u32]) -> Option<usize> {
    let mut dir = None;
    for (idx, &[a, b]) in numbers.array_windows().enumerate() {
        let diff = a.checked_signed_diff(b).unwrap();
        match diff.abs() {
            1..4 => (),
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