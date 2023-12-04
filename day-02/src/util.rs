
#[derive(Debug, Default)]
pub struct Turn {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

impl Turn {
    pub fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}


pub fn parse(input: &str) -> Vec<Vec<Turn>> {
    let lines = input.lines();
    let mut games: Vec<Vec<Turn>> = Vec::new();

    for line in lines {
        let (_, turns) = line.split_once(": ").unwrap();
        let turns = turns.split("; ").collect::<Vec<_>>();
        let mut turn_list = Vec::new();

        for t in turns {
            let cubes = t.split(", ").collect::<Vec<_>>();
            let mut turn = Turn::default();

            for cube in cubes {
                let (amount, color) = cube.split_once(' ').unwrap();
                let amount: usize = amount.parse().unwrap();

                match &color[0..1] {
                    "r" => turn.red = amount,
                    "g" => turn.green = amount,
                    "b" => turn.blue = amount,
                    _ => panic!("bug in your code"),
                }
            }

            turn_list.push(turn);
        }
        games.push(turn_list);
    }

    games
}
