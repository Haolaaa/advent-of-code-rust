pub fn parse_line(input: &str) -> u32 {
    let (_, input) = input.split_once(":").unwrap();
    let (left, right) = input.split_once("|").unwrap();

    let winning_numbers = left
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    right
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .fold(0_u32, |acc, num| {
            if winning_numbers.contains(&num) {
                return acc + 1;
            }

            acc
        })
}
