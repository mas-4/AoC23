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

    let mut parse_lines = Vec::new();
    let mut maps: Vec<Map> = Vec::new();
    for line in lines.iter().skip(2) {
        if line.trim().is_empty() {
            maps.push(Map::new(&parse_lines));
            parse_lines.clear();
            continue;
        } else {
            parse_lines.push(line);
        }
    }
    maps.push(Map::new(&parse_lines));

    Almanac {
        maps,
        seeds,
    }
}

fn parse_map_title(line: &String) ->  (String, String) {
    // Parses the title of a map, which is of the form
    // "source-to-destination map:"
    let parts: Vec<&str> = line.split("-to-").collect();
    let mut destination = parts[1].trim();
    if let Some(index) = destination.find(" map") {
        destination = &destination[..index];
    }
    (parts[0].trim().to_string(), destination.to_string())
}


#[derive(Debug)]
struct Almanac {
    maps: Vec<Map>,
    seeds: Vec<usize>,
}
#[derive(Debug)]
struct Map {
    source: String,
    destination: String,
    translations: Vec<Translation>,
}
#[derive(Debug)]
struct Translation {
    source_start: usize,
    destination_start: usize,
    range: usize,
}


impl Almanac {
    fn feed_forward(&self, seed: usize) -> usize {
        let mut result = seed;
        for map in self.maps.iter() {
            result = map.translate(result);
        }
        result
    }
}

impl Map {
    fn new(lines: &Vec<&String>) -> Map {
        let (source, destination) = parse_map_title(lines[0]);
        let translations = lines
            .iter()
            .skip(1)
            .map(|line| Translation::new(line))
            .collect();
        Map {
            source,
            destination,
            translations,
        }
    }

    fn translate(&self, n: usize) -> usize {
        for translation in self.translations.iter() {
            if translation.contains(n) {
                return translation.translate(n);
            }
        }
        n
    }
}

impl Translation {
    fn new(line: &String) -> Translation {
        let nums: Vec<usize> = line
            .split_whitespace()
            .map(|n| n.parse::<usize>().expect("bad integer"))
            .collect();
        Translation {
            source_start: nums[1],
            destination_start: nums[0],
            range: nums[2],
        }
    }

    fn contains(&self, n: usize) -> bool {
        n >= self.source_start && n <= self.source_start + self.range
    }
    fn translate(&self, n: usize) -> usize {
        n - self.source_start + self.destination_start
    }
}

fn ch05_1() -> usize {
    let almanac = get_data();
    for map in almanac.maps.iter() {
        println!("{:?}", map.source);
    }
    almanac
        .seeds
        .iter()
        .map(|seed| almanac.feed_forward(*seed))
        .min()
        .expect("No minimum!")
}

fn ch05_2() -> usize {
    let almanac = get_data();
    let seed_pairs = almanac
        .seeds
        .chunks(2);
    3


}

pub fn ch05() {
    println!("05-1: {}", ch05_1());
    println!("05-2: {}", ch05_2());
}
