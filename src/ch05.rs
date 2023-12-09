use std::collections::HashMap;

fn get_data() -> Almanac {
    let lines = include_str!("ch05.txt")
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
    let mut maps = HashMap::new();
    let mut map_order = Vec::new();
    for line in lines.iter().skip(2) {
        if line.trim() == "" {
            continue;
        } else if line.chars().next().expect("No string!").is_alphabetic() {
            source = line.split('-').collect::<Vec<&str>>()[0].to_string();
            maps.insert(source.clone(), Vec::new());
            map_order.push(source.clone());
        } else {
            // Insert a new map into the vector
            maps.get_mut(&source).expect("No source!").push(Map::new(line));
        }
    }
    Almanac {
        maps,
        map_order,
        seeds,
    }
}

struct Almanac {
    maps: HashMap<String, Vec<Map>>,
    map_order: Vec<String>,
    seeds: Vec<usize>,
}

impl Almanac {
    fn feed_forward(&self, seed: usize) -> usize {
        let mut result = seed;
        for map in self.map_order.iter() {
            for m in self.maps.get(map).expect("No map!") {
                if m.contains(result) {
                    result = m.translate(result);
                    break;
                }
            }
        }
        result
    }
}

struct Map {
    source_start: usize,
    destination_start: usize,
    range: usize,
}

impl Map {
    fn new(line: &String) -> Map {
        let nums: Vec<usize> = line
            .split_whitespace()
            .map(|n| n.parse::<usize>().expect("bad integer"))
            .collect();
        Map {
            source_start: nums[1],
            destination_start: nums[0],
            range: nums[2],
        }
    }

    fn contains(&self, n: usize) -> bool {
        n >= self.source_start && n < self.source_start + self.range
    }
    fn translate(&self, n: usize) -> usize {
        n - self.source_start + self.destination_start
    }


}

fn ch05_1() -> usize {
    let almanac = get_data();
    almanac
        .seeds
        .iter()
        .map(|seed| almanac.feed_forward(*seed))
        .min()
        .expect("No minimum!")
}

fn ch05_2() -> usize {
    0
}

pub fn ch05() {
    println!("05-1: {}", ch05_1());
    println!("05-2: {}", ch05_2());
}
