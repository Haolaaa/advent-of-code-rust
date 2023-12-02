use std::io::Error;

pub fn process(input: &str) -> Result<u32, Error> {
    let output = input.lines().map(|line| {
        let mut it = line.chars();

        let first = it.find_map(|char| {
            char.to_digit(10)
        }).unwrap();

        let last = it.rfind(|char| char.is_digit(10)).map(|char| {
            char.to_digit(10).unwrap()
        }).unwrap_or(first);

        first * 10 + last
    }).sum::<u32>();

    Ok(output)
}
