use crate::util::parse_line;

pub fn process(input: &str) -> u32 {
    input.lines().map(parse_line).fold(0, |acc, count| {
        if count != 0 {
            return acc + 2u32.pow(count - 1);
        }
        acc
    })
}
