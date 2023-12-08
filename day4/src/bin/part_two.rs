use std::fs;
use day4::part_two;

fn main() {
    let input = fs::read_to_string("./INPUT.txt").unwrap();
    println!("{}", part_two(&input));
}
