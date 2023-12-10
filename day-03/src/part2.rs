use std::io::Error;

use crate::util::{create_grid, get_parts_list, Part};

pub fn process(input: &str) -> Result<u32, Error> {
    let grid = create_grid(input);

    let parts = get_parts_list(grid);
    let res = part_b(parts);

    Ok(res)
}

fn part_b(parts: Vec<(Part, Vec<u32>)>) -> u32 {
    parts
        .iter()
        .filter_map(|(part, matches)| match part.part_type {
            '*' => {
                if matches.len() == 2 {
                    Some(matches.iter().product::<u32>())
                } else {
                    None
                }
            }
            _ => None,
        })
        .sum()
}
