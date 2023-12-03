fn get_data_01_1() -> Vec<String> {
    include_str!("ch01.txt").lines().map(|line| line.to_string()).collect()
}

fn first_digit(s: &str) -> Option<char> {
    s.chars().find(|c| c.is_numeric())
}

fn last_digit(s: &str) -> Option<char> {
    s.chars().rev().find(|c| c.is_numeric())
}


fn ch01_1() -> i32 {
    get_data_01_1()
        .iter()
        .map(|s| {
            format!(
                "{}{}",
                first_digit(s).unwrap(),
                last_digit(s).unwrap()
            ).parse::<i32>().unwrap()
        }).sum()
}


fn get_list_of_number_words() -> [&'static str; 10]{
    ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
}

fn first_number(s: &str) -> Option<i32> {
    let digit_pos = s.chars().position(|c| c.is_numeric())?;
    let mut word_pos = 1000;
    let mut found_word = "zero";
    for word in get_list_of_number_words() {
        if s.contains(word) && s.find(word)? < word_pos {
            word_pos = s.find(word)?;
            found_word = word;
        }
    }
    if word_pos < digit_pos {
        Some(get_list_of_number_words().iter().position(|&x| x == found_word).unwrap() as i32)
    } else {
        Some(s.chars().nth(digit_pos)?.to_digit(10)? as i32)
    }
}

fn last_number(s: &str) -> Option<i32> {
    let digit_pos = s.len() - s.chars().rev().position(|c| c.is_numeric()).unwrap() - 1;
    let mut word_pos = 0;
    let mut found_word = "zero";
    for word in get_list_of_number_words() {
        if s.contains(word) && s.rfind(word)? > word_pos {
            word_pos = s.rfind(word)?;
            found_word = word;
        }
    }
    if word_pos > digit_pos {
        Some(get_list_of_number_words().iter().position(|&x| x == found_word).unwrap() as i32)
    } else {
        Some(s.chars().nth(digit_pos)?.to_digit(10)? as i32)
    }
}

fn ch01_2() -> i32 {
    get_data_01_1()
        .iter()
        .map(|s| {
            let first = first_number(s).unwrap();
            let last = last_number(s).unwrap();
            format!(
                "{}{}",
                first,
                last
            ).parse::<i32>().unwrap()
        }).sum()
}

pub fn ch01() {
    println!("01-1: {}", ch01_1());
    println!("01-2: {}", ch01_2());
}
