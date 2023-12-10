mod part1;
mod part2;
mod util;

fn main() {
    let input = include_str!("./input.txt");
    let p1 = part1::process(input);
    let p2 = part2::process(input);

    // println!("{p1}")
    println!("{p2}")
}
