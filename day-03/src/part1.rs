use std::io::Error;

use crate::util::{Part, create_grid, get_parts_list};

fn part_a(parts: &Vec<(Part, Vec<u32>)>) -> u32 {
    parts
        .iter()
        .map(|(_, matches)| matches.iter().sum::<u32>())
        .sum()
}

pub fn process(input: &str) -> Result<u32, Error> {
    let grid = create_grid(input);

    let parts = get_parts_list(grid);

    let part_1 = part_a(&parts);

    Ok(part_1)
}
