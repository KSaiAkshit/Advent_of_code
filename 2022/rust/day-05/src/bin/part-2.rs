use std::fs;

fn main() {
    let file = fs::read_to_string("input").unwrap();
    let result = day_05::process_part2(&file);
    println!("{}", result);
}
