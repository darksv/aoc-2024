use std::collections::HashMap;
use std::iter;
use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let mut list1 = Vec::with_capacity(1000);
    let mut list2 = Vec::with_capacity(1000);
    for [a, b] in input.split_ascii_whitespace().array_chunks() {
        let a = a.parse::<u32>().unwrap();
        let b = b.parse::<u32>().unwrap();

        list1.push(a);
        list2.push(b);
    }

    list1.sort_unstable();
    list2.sort_unstable();

    iter::zip(&list1, &list2).map(|(a, b)| a.abs_diff(*b)).sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let mut list1 = Vec::with_capacity(1000);
    let mut counts = HashMap::new();

    for [a, b] in input.split_ascii_whitespace().array_chunks() {
        let a = a.parse::<u32>().unwrap();
        let b = b.parse::<u32>().unwrap();

        list1.push(a);
        counts.entry(b).and_modify(|x| *x += 1).or_insert(1);
    }

    list1.iter().map(|x| *x * counts.get(x).copied().unwrap_or(0)).sum()
}