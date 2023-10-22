use std::fs;

fn main() {
    let file = fs::read_to_string("input").unwrap();
    let result = day_10::process_part1(&file);
    println!("{}", result);
}
