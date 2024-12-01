use std::fs;

pub fn parse_input(input_path: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = vec![];
    let mut right = vec![];
    // read the input file line by line
    fs::read_to_string(input_path).unwrap()
        .lines()
        .for_each(|line| {
            // for each line split the line, parse the numbers, and push them to the respective vec
            let nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            left.push(nums[0]);
            right.push(nums[1]);
        });
    // sort the vecs least to greatest
    left.sort();
    right.sort();
    (left, right)
}

fn get_total_diff(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    left.iter().zip(right.iter()).map(|(l, r)| (l - r).abs()).sum()
}

pub fn solve_p1(input_path: &str) -> i32 {
    let (left, right) = parse_input(input_path);
    get_total_diff(&left, &right)
}