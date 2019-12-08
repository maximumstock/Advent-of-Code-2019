use std::ops::RangeInclusive;

fn main() {
    let input = input();
    let part1_output = part1(input);
    println!("Found {:?} valid passwords", part1_output);
}

fn input() -> RangeInclusive<usize> {
    372_304..=847_060
}

fn part1(input: RangeInclusive<usize>) -> usize {
    let valid_passwords = input
        .filter(|x: &usize| is_valid(*x))
        .collect::<Vec<usize>>();

    valid_passwords.len()
}

fn is_valid(input: usize) -> bool {
    rule_contains_double(input) && rule_never_decreasing(input)
}

fn rule_never_decreasing(input: usize) -> bool {

    let chars = input.to_string().chars().collect::<Vec<char>>();
    let unsorted_chars = chars.clone();
    let mut sorted_chars = chars.clone();
    sorted_chars.sort();

    let n_matching_chars = unsorted_chars.iter()
        .zip(&sorted_chars)
        .filter(|&(a, b)| a == b )
        .count();

    n_matching_chars == chars.len()

}

fn rule_contains_double(input: usize) -> bool {

    let chars = input.to_string().chars().collect::<Vec<char>>();
    let mut unique_chars = chars.clone();
    unique_chars.dedup();

    chars.len() != unique_chars.len()

}

#[test]
fn test_is_valid() {
    assert_eq!(is_valid(111_111), true);
    assert_eq!(is_valid(223_450), false);
    assert_eq!(is_valid(123_789), false);
}