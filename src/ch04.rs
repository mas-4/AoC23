fn get_data() -> Vec<String> {
    include_str!("ch04.txt")
        .lines()
        .map(|l| l.to_string())
        .collect()
}

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<i32>,
    your_numbers: Vec<i32>,
}

impl Card {
    fn new(line: &str) -> Card {
        let parts: Vec<&str> = line.split(':').collect();
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
            winning_numbers,
            your_numbers,
        }
    }

    fn count_winning_numbers(&self) -> usize {
        let mut count = 0;
        for n in &self.your_numbers {
            if self.winning_numbers.contains(n) {
                count += 1;
            }
        }
        count
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
    let cards = get_data()
        .iter()
        .map(|l| Card::new(l.as_str()))
        .collect::<Vec<Card>>();
    let mut card_counts = vec![1; cards.len()];
    for (i, c) in cards.iter().enumerate() {
        let winning_count = c.count_winning_numbers();
        if winning_count == 0 {
            continue;
        }
        for j in 1..(winning_count+1) {
            card_counts[i + j] += 1*card_counts[i];
        }
    }
    card_counts.iter().sum()
}

pub fn ch04() {
    println!("04-1: {}", ch04_1());
    println!("04-2: {}", ch04_2());
}