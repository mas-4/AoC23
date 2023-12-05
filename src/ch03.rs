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

    fn touches_closure(&self, schematic: &Vec<String>, closure: fn(c: char) -> bool) -> bool {
        self.check_row(self.row as i32 - 1, schematic, closure)
            || self.check_row(self.row as i32 + 1, schematic, closure)
            || self.check_row(self.row as i32, schematic, closure)
    }

    fn get_num(schematic: &[String], row: i32, col: i32) -> i32 {
        let c = schematic[row as usize].chars().nth(col as usize).expect("Bad char");
        // number can spread either left or right
        if c.is_numeric() {
            let mut i = col;
            while i >= 0 { // go left
                let c = schematic[row as usize].chars().nth(i as usize).expect("Bad char left");
                if !c.is_numeric() || c == '.' {
                    i += 1; // traced too far
                    break;
                }
                i -= 1;
            }
            i = i.max(0);
            let mut j = col;
            while j < schematic[0].len() as i32 { // go right
                let c = schematic[row as usize].chars().nth(j as usize).expect("Bad char right");
                if !c.is_numeric() || c == '.' {
                    j -= 1; // traced too far
                    break;
                }
                j += 1;
            }
            let num_str = schematic[row as usize].chars().skip(i as usize).take((j - i + 1) as usize).collect::<String>();
            let msg = format!("bad number num_str: {}, row: {}, col: {}\n{}", num_str, row, col, schematic[row as usize]);
            let msg = msg.as_str();
            num_str.parse().expect(msg)
        }
        else {
            0
        }
    }

    fn gear_ratio(&self, schematic: &Vec<String>) -> i32 {
        let mut nums: Vec<i32> = Vec::new();
        let i0 = (self.row as i32 - 1).max(0);
        let i1 = (self.row as i32 + 1).min(schematic.len() as i32 - 1);
        let j0 = (self.start as i32 - 1).max(0);
        let j1 = (self.end as i32 + 1).min(schematic[0].len() as i32 - 1);

        for i in i0..i1 {
            for j in j0..j1 {
                let c = schematic[i as usize].chars().nth(j as usize).expect("Bad char");
                if c.is_numeric() {
                    let num = Span::get_num(schematic, i, j);
                    if nums.contains(&num)
                    {
                        continue
                    }
                    nums.push(num);
                }
                if nums.len() == 2 {
                    println!("nums: {:?}", nums);
                    return nums[0] * nums[1]
                }
            }
        }
        0
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
            let closure = |c: char| !c.is_alphanumeric() && c != '.';
            total += if n.touches_closure(&data, closure) {
                n.contents.parse().expect("Bad number!")
            } else {
                0
            };
        }
    }
    total
}

fn ch03_2() -> i32 {
    let data = get_data();
    let mut total = 0;
    for (i, row) in data.iter().enumerate() {
        let mut j = 0;
        while let Some(pos) = row[j..].chars().position(|c| c == '*') {
            j += pos;
            let n = Span {
                row: i,
                start: j,
                end: j + 1,
                contents: "*".to_string(),
            };
            j += 1;
            let closure = |c: char| c.is_numeric();
            total += if n.touches_closure(&data, closure) {
                n.gear_ratio(&data)
            } else {
                0
            }
        }
    }
    total
}

pub fn ch03() {
    println!("03-1: {}", ch03_1());
    println!("03-2: {}", ch03_2());
}
