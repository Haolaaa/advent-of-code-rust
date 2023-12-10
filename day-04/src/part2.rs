use crate::util::parse_line;

pub fn process(input: &str) -> u32 {
    let mut res = vec![1; input.lines().count()];

    for (index, line) in input.lines().enumerate() {
        for x in 1..=parse_line(line) {
            res[index + x as usize] += res[index];
        }
    }

    res.iter().sum()
}
