use std::io::Error;

use crate::util::parse;

pub fn process(input: &str) -> Result<u32, Error> {
    let mut power_sum = 0;
    let games = parse(input);

    for game in games {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for turn in game {
            red = red.max(turn.red);
            green = green.max(turn.green);
            blue = blue.max(turn.blue);
        }

        power_sum += red * green * blue;
    }

    Ok(power_sum as u32)
}
