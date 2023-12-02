pub mod part1;
pub mod part2;

fn main() {

    let input = include_str!("./input.txt");
    
    let p1 = part1::process(input).unwrap();
    let p2 = part2::process(input).unwrap();

    println!("part one -> {p1}");
    println!("part two -> {p2}");
}
