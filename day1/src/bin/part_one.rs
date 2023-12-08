use day1::part_one;
use std::fs;

fn main() {
    let input = fs::read_to_string("./INPUT.txt").unwrap();
    println!("{}", part_one(&input));
}
