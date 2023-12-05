fn get_data() -> Vec<String> {
    include_str!("ch04.txt")
        .lines()
        .map(|l| l.to_string())
        .collect()
}

#[derive(Debug)]
struct Card {
    card_no: i32,
    winning_numbers: Vec<i32>,
    your_numbers: Vec<i32>,
}

impl Card {
    fn new(line: &str) -> Card {
        let parts: Vec<&str> = line.split(':').collect();
        let card_no = parts[0]
            .split_whitespace()
            .nth(1)
            .expect("Bad card no")
            .parse::<i32>()
            .expect("Bad card no");
        let numbers: Vec<&str> = parts[1]
            .split('|')
            .collect();
        let winning_numbers = numbers[0]
            .split_whitespace()
            .map(|n| n.trim().parse::<i32>().expect("Bad winning number"))
            .collect();
        let your_numbers = numbers[1]
            .split_whitespace()
            .map(|n| n.trim().parse::<i32>().expect("Bad your number"))
            .collect();
        Card {
            card_no,
            winning_numbers,
            your_numbers,
        }
    }

    fn score(&self) -> i32 {
        let mut score = 0;
        for n in &self.your_numbers {
            if !self.winning_numbers.contains(n) {
                continue;
            }
            if score == 0 {
                score = 1;
            } else {
                score *= 2;
            }
        }
        score
    }
}

fn ch04_1() -> i32 {
    get_data()
        .iter()
        .map(|l| Card::new(l.as_str()).score())
        .sum()
}

fn ch04_2() -> i32 {
    0
}

pub fn ch04() {
    println!("04-1: {}", ch04_1());
    println!("04-2: {}", ch04_2());
}