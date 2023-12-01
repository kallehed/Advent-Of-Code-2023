use regex::Regex;

pub fn day1_1() {
    let s = std::fs::read_to_string("src/day1.txt").unwrap();
    let mut sum: u64 = 0;
    for line in s.lines() {
        let mut digs = Vec::new();
        for c in line.chars() {
            if c.is_numeric() {
                digs.push((c as u8 - b'0') as u64);
            } 
        }
        sum += digs[0] * 10 +  digs[digs.len() - 1];
    }
    println!("{}", sum);
}

pub fn day1_2() {
    let s = std::fs::read_to_string("src/day1.txt").unwrap();
    let mut sum: u64 = 0;
    for line in s.lines() {
        let mut digs = Vec::new();
        for (idx, c) in line.chars().enumerate() {
            if c.is_numeric() {
                digs.push((c as u8 - b'0') as u64);
            } 
            else if line[idx..].starts_with("one") {
                digs.push(1);
            }
            else if line[idx..].starts_with("two") {
                digs.push(2);
            }
            else if line[idx..].starts_with("three") {
                digs.push(3);
            }
            else if line[idx..].starts_with("four") {
                digs.push(4);
            }
            else if line[idx..].starts_with("five") {
                digs.push(5);
            }
            else if line[idx..].starts_with("six") {
                digs.push(6);
            }
            else if line[idx..].starts_with("seven") {
                digs.push(7);
            }
            else if line[idx..].starts_with("eight") {
                digs.push(8);
            }
            else if line[idx..].starts_with("nine") {
                digs.push(9);
            }
        }
        let add = digs[0] * 10 + digs.last().unwrap();
        println!("{:?} = {}", digs, add);

        sum += add;
    }
    println!("{}", sum);
}
