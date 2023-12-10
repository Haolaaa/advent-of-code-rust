use grid::{grid, Grid};
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
pub struct Part {
    pub location: Point,
    pub part_type: char,
}

pub fn create_grid(input: &str) -> Grid<char> {
    // create a grid and fill it with our input
    let mut grid: Grid<char> = grid![];
    for line in input.lines() {
        grid.push_row(line.chars().collect::<Vec<_>>())
    }

    grid
}

pub fn parse_part(location: (usize, usize), part_type: &char) -> Option<Part> {
    match part_type {
        '0'..='9' | '.' => None,
        _ => Some(Part {
            location: Point {
                x: location.0,
                y: location.1,
            },
            part_type: *part_type,
        }),
    }
}

pub fn find_adjacent_points(point: &Point) -> Vec<Point> {
    // For the part, calculate all it's adjacent coordinates
    let mut adjacent_points: Vec<Point> = vec![];

    // Loop around the point generating a vec
    // There are no parts on the edge of the schematic so we do not worry over/uderflowing
    for x in (point.x - 1)..=point.x + 1 {
        for y in (point.y - 1)..=(point.y + 1) {
            adjacent_points.push(Point { x, y })
        }
    }

    adjacent_points
}

pub fn discover_numbers(part: &Part, grid: &Grid<char>) -> Vec<u32> {
    // For the part, calculate all it's adjacent coordinates
    let adjacent_points = find_adjacent_points(&part.location);

    // Regex matcher for numbers
    let re = Regex::new(r"\d+").unwrap();
    let mut matches: Vec<u32> = vec![];

    for x in (part.location.x - 1)..=part.location.x + 1 {
        // build string from grid
        let row = grid.iter_row(x).collect::<String>();

        // Iterate through the matches and attach them to the part
        for m in re.find_iter(&row) {
            let match_range = m.start()..m.end();

            for y in match_range {
                if adjacent_points.contains(&Point { x, y }) {
                    // Parse the match and push the result into the part
                    matches.push(m.as_str().parse::<u32>().unwrap());
                    // Move onto the next regex match if a gear is touching
                    break;
                }
            }
        }
    }

    matches
}

pub fn get_parts_list(grid: Grid<char>) -> Vec<(Part, Vec<u32>)> {
    grid.indexed_iter()
        .filter_map(|(location, part_type)| parse_part(location, part_type))
        .map(|part| {
            let matches = discover_numbers(&part, &grid);
            (part, matches)
        })
        .collect::<Vec<_>>()
}
