use day8::part_two;
use std::fs;

fn main() {
    let input = fs::read_to_string("./INPUT.txt").unwrap();
    let s = Box::leak(input.into_boxed_str());
    println!("{}", part_two(s));
}
