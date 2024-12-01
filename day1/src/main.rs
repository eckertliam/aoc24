mod p1;
mod p2;

use p1::solve_p1;
use p2::solve_p2;

fn main() {
    let total_diff = solve_p1("p1_input.txt");
    println!("total diff: {}", total_diff);
    let similarity_score = solve_p2("p2_input.txt");
    println!("similarity score: {}", similarity_score);
}
