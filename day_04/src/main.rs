use std::ops::RangeInclusive;

fn main() {
    let input = input();
    let part1_output = part1(input.clone());
    println!("Part 1: Found {:?} valid passwords", part1_output);
    let part2_output = part2(input.clone());
    println!("Part 2: Found {:?} valid passwords", part2_output);
}

fn input() -> RangeInclusive<usize> {
    372_304..=847_060
}

fn part1(input: RangeInclusive<usize>) -> usize {
    let valid_passwords = input
        .filter(|x: &usize| is_valid_part1(*x))
        .collect::<Vec<usize>>();

    valid_passwords.len()
}

fn part2(input: RangeInclusive<usize>) -> usize {
    let valid_passwords = input
        .filter(|x: &usize| is_valid_part2(*x))
        .collect::<Vec<usize>>();

    valid_passwords.len()
}

fn is_valid_part1(input: usize) -> bool {
    rule_contains_double(input) && rule_never_decreasing(input)
}

fn is_valid_part2(input: usize) -> bool {
    is_valid_part1(input) && rule_double_group_required(input)
}

fn rule_never_decreasing(input: usize) -> bool {
    let chars = input.to_string().chars().collect::<Vec<char>>();
    let unsorted_chars = chars.clone();
    let mut sorted_chars = chars.clone();
    sorted_chars.sort();

    let n_matching_chars = unsorted_chars.iter()
        .zip(&sorted_chars)
        .filter(|&(a, b)| a == b)
        .count();

    n_matching_chars == chars.len()
}

fn rule_contains_double(input: usize) -> bool {
    let chars = input.to_string().chars().collect::<Vec<char>>();
    let mut unique_chars = chars.clone();
    unique_chars.dedup();

    chars.len() != unique_chars.len()
}

fn rule_double_group_required(input: usize) -> bool {
    let chars = input.to_string().chars().collect::<Vec<char>>();

    type OccurrenceMap = std::collections::HashMap<char, usize>;

    let occurences = chars
        .iter()
        .fold(std::collections::HashMap::new(), |mut acc: OccurrenceMap, &character| {
            acc.entry(character).and_modify(|x| *x += 1).or_insert(1);
            acc
        });

    let n_double_groups = occurences.values().filter(|x| **x == 2).count();
    n_double_groups > 0
}

#[test]
fn test_is_valid_part1() {
    assert_eq!(is_valid_part1(111_111), true);
    assert_eq!(is_valid_part1(223_450), false);
    assert_eq!(is_valid_part1(123_789), false);
}

#[test]
fn test_is_valid_part2() {
    assert_eq!(is_valid_part2(112_233), true);
    assert_eq!(is_valid_part2(123_444), false);
    assert_eq!(is_valid_part2(111_122), true);
}
