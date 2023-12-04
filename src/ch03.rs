fn get_data() -> Vec<String> {
    include_str!("ch03.txt")
        .lines()
        .map(|l| l.to_string())
        .collect()
}

struct Span {
    row: usize,
    start: usize,
    end: usize,
    contents: String,
}

impl Span {
    fn check_row(&self, row: i32, schematic: &Vec<String>, validator: fn(c: char) -> bool) -> bool {
        // Checks that a given row based on our span has a character that qualifies as valid for the callback
        if row < 0 || row as usize >= schematic.len() {
            return false;
        }
        let start = (self.start as i32 - 1).max(0);
        let end = (self.end as i32 + 1).min(schematic[0].len() as i32 - 1);
        for i in start..end {
            let msg = format!(
                "start: {}, line: {}, num: {}",
                start, &schematic[row as usize], self.contents
            );
            let c = schematic[row as usize].chars().nth(i as usize).expect(&msg);
            if validator(c) {
                return true;
            }
        }
        false
    }

    fn touches_symbol(&self, schematic: &Vec<String>) -> bool {
        let closure = |c: char| !c.is_alphanumeric() && c != '.';
        self.check_row(self.row as i32 - 1, schematic, closure)
            || self.check_row(self.row as i32 + 1, schematic, closure)
            || self.check_row(self.row as i32, schematic, closure)
    }
}

fn ch03_1() -> i32 {
    let data = get_data();
    let mut total = 0;
    for (i, row) in data.iter().enumerate() {
        let mut j = 0;
        while let Some(pos) = row[j..].chars().position(|c| c.is_numeric()) {
            j += pos;
            let num_str = row[j..]
                .chars()
                .take_while(|c| c.is_numeric())
                .collect::<String>();
            let n = Span {
                row: i,
                start: j,
                end: j + num_str.len(),
                contents: num_str,
            };
            j = n.end;
            total += if n.touches_symbol(&data) {
                n.contents.parse().expect("Bad number!")
            } else {
                0
            };
        }
    }
    total
}

fn ch03_2() -> i32 {
    0
}

pub fn ch03() {
    println!("03-1: {}", ch03_1());
    println!("03-2: {}", ch03_2());
}
