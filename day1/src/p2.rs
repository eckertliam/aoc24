use std::collections::HashMap;
use crate::p1::parse_input;

pub fn solve_p2(input_path: &str) -> i32 {
    // a map of the number of times each number appears on the right
    let mut right_counts = HashMap::new();
    let (left, right) = parse_input(input_path);
    right.iter().for_each(|n| {
        // increment the count for the number if it exists, otherwise set it to 1
        *right_counts.entry(n).or_insert(0) += 1;
    });
    // for each number on the left multiply it by the number of times it appears on the right
    // returning the similarity score
    left.iter().map(|l| l * right_counts.get(l).unwrap_or(&0)).sum()
}