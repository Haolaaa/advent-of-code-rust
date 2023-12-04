use std::io::Error;

use crate::util::parse;

pub fn process(input: &str) -> Result<u32, Error> {
    let games = parse(input);
    let mut valid_games_total = 0;

    'next_game: for (index, game) in games.iter().enumerate() {
        for turn in game {
            if !turn.is_valid() {
                continue 'next_game;
            }
        }

        valid_games_total += index + 1;
    }

    Ok(valid_games_total as u32)
}
