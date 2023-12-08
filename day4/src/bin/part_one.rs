use std::fs;
use day4::part_one;

fn main() {
    let input = fs::read_to_string("./INPUT.txt").unwrap();
    println!("{}", part_one(&input));
}
