fn get_data() -> Vec<String> {
    include_str!("ch02.txt").lines().map(|line| line.to_string()).collect()
}

#[derive(Debug)]
struct MarbleSet {
    red: i32,
    green: i32,
    blue: i32,
}

impl MarbleSet {
    fn parse(input: &str) -> Result<MarbleSet, String> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for part in input.split(',') {
            let n: i32 = part
                .split_whitespace()
                .nth(0)
                .expect("Malformed MarbleSet")
                .parse::<i32>()
                .expect("Malformed MarbleSet");
            match part.split_whitespace().nth(1) {
                Some("red") => red = n,
                Some("green") => green = n,
                Some("blue") => blue = n,
                _ => return Err(format!("Malformed MarbleSet: {}", input)),
            }
        }
        Ok(MarbleSet { red, green, blue })
    }
    fn contains(&self, other: &MarbleSet) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }
}

struct Game {
    game_id: i32,
    sets: Vec<MarbleSet>,
}

impl Game {
    fn parse(input: &str) -> Result<Game, String> {
        let parts: Vec<&str> = input.splitn(2, ':').collect();
        // parse the substring after "Game "
        let game_id = parts[0]
            .split_whitespace()
            .nth(1)
            .expect("Malformed Game ID")
            .parse::<i32>()
            .expect("Malformed Game ID");
        let sets: Vec<MarbleSet> = parts[1]
            .split(';')
            .map(|s| MarbleSet::parse(s).unwrap())
            .collect();
        Ok(Game { game_id, sets })

    }
}


fn ch02_1() -> i32 {
    let masterset = MarbleSet {
        red: 12,
        green: 13,
        blue: 14,
    };
    get_data()
        .iter()
        .map(|s| Game::parse(s).unwrap())
        .filter(|g| g.sets.iter().all(|s| s.contains(&masterset)))
        .map(|g| g.game_id)
        .sum()
}

fn ch02_2() -> i32 {
    22
}

pub fn ch02() {
    println!("02-1: {}", ch02_1());
    println!("02-2: {}", ch02_2());
}