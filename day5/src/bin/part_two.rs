use day5::part_two;
use std::fs;

fn main() {
    let input = fs::read_to_string("./INPUT.txt").unwrap();
    println!("{}", part_two(&input));
}
