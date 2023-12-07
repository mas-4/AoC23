fn get_data() -> Almanac {
    let lines = include_str!("ch05test.txt")
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
    let seeds: Vec<usize> = lines
        .first()
        .expect("Bugged file!")
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().expect("Not an integer!"))
        .collect();

    let mut source = String::new();
    let mut destination = String::new();
    let mut maps = Vec::new();
    for line in lines.iter().skip(2) {
        if line.trim() == "" {
            continue;
        } else if line.chars().next().expect("No string!").is_alphabetic() {
            source = line.split('-').collect::<Vec<&str>>()[0].to_string();
            destination = line
                .split_whitespace()
                .next()
                .unwrap_or("")
                .split('-')
                .collect::<Vec<&str>>()[2]
                .to_string();
        } else {
            maps.push(Map::new(source.clone(), destination.clone(), line));
        }
    }
    Almanac { maps, seeds }
}

struct Almanac {
    maps: Vec<Map>,
    seeds: Vec<usize>,
}

struct Map {
    source: String,
    destination: String,
    source_start: usize,
    destination_start: usize,
    range: usize,
}

impl Map {
    fn new(source: String, destination: String, line: &String) -> Map {
        let nums: Vec<usize> = line
            .split_whitespace()
            .map(|n| n.parse::<usize>().expect("bad integer"))
            .collect();
        println!("{:?}", nums);
        Map {
            source,
            destination,
            source_start: nums[0],
            destination_start: nums[1],
            range: nums[2],
        }
    }
}

fn ch05_1() -> usize {
    let almanacs = get_data();
    0
}

fn ch05_2() -> usize {
    0
}

pub fn ch05() {
    println!("05-1: {}", ch05_1());
    println!("05-2: {}", ch05_2());
}
