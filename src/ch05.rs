use std::time::Instant;
use std::sync::Arc;

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


#[derive(Clone)]
struct Almanac {
    maps: Vec<Map>,
    seeds: Vec<usize>,
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

#[allow(dead_code)]
#[derive(Clone)]
struct Map {
    source: String,
    destination: String,
    translations: Vec<Translation>,
}

impl Map {
    fn new(lines: &Vec<&String>) -> Map {
        let (source, destination) = parse_map_title(lines[0]);
        let mut translations: Vec<Translation> = lines
            .iter()
            .skip(1)
            .map(|line| Translation::new(line))
            .collect::<Vec<Translation>>();
        translations.sort_by(|a, b| a.destination_start.cmp(&b.destination_start));
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

#[derive(Clone)]
struct Translation {
    source_start: usize,
    destination_start: usize,
    range: usize,
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
    almanac
        .seeds
        .iter()
        .map(|seed| almanac.feed_forward(*seed))
        .min()
        .expect("No minimum!")
}

#[derive(Clone)]
struct SeedSet {
    start: usize,
    range: usize,
}

fn find_lowest_location(seeds: &SeedSet, almanac: &Almanac) -> usize {
    let mut result = usize::MAX;
    for seed in seeds.start..=seeds.start + seeds.range {
        let location = almanac.feed_forward(seed);
        if location < result {
            result = location;
        }
    }
    result
}

fn ch05_2() -> usize {
    let start_time = Instant::now();
    let almanac = Arc::new(get_data());
    let seeds: Vec<SeedSet> = almanac
        .seeds
        .chunks(2)
        .map(|chunk| SeedSet { start: chunk[0], range: chunk[1], })
        .collect();

    let mut threads = Vec::new();
    for seedset in seeds.iter() {
        let almanac = Arc::clone(&almanac);
        let seedset = seedset.clone();
        // spawn a thread for each seedset
        threads.push(std::thread::spawn(move || {
            find_lowest_location(&seedset, &almanac)
        }));
    }
    // wait for thread completion
    let mut results = Vec::new();
    for thread in threads {
        results.push(thread.join().unwrap());
    }
    let elapsed = start_time.elapsed();
    println!("Elapsed: {:?}", elapsed);
    *results.iter().min().unwrap()
}

pub fn ch05() {
    println!("05-1: {}", ch05_1());
//    println!("05-2: {}", ch05_2());
    println!("05-2: disabled due to time constraints");
}
