use regex::Regex;

pub fn day4_1() {
    let s = std::fs::read_to_string("src/day4.txt").unwrap();
    let reg = Regex::new(r"\d+").unwrap();
    let mut total = 0;
    for line in s.lines() {
        let mut winning = std::collections::HashSet::new();
        let (_, line) = line.split_once(":").unwrap();
        let (first, second) = line.split_once('|').unwrap();

        for cap in reg.captures_iter(first) {
            winning.insert(cap[0].parse::<i64>().unwrap());
        }

        let mut score = 0;
        for cap in reg.captures_iter(second) {
            if winning.contains(&cap[0].parse::<i64>().unwrap()) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }
        total += score;
    }
    println!("total: {}", total);
}

pub fn day4_2() {
    let s = std::fs::read_to_string("src/day4.txt").unwrap();
    let reg = Regex::new(r"\d+").unwrap();
    let mut cards = [1; 200];
    let mut my_lines = 0;
    for (card_at, line) in s.split('\n').enumerate() {
        let mut winning = std::collections::HashSet::new();
        my_lines += 1;
        let pos = line.chars().position(|x| x == ':').unwrap();
        let line = &line[(pos + 1)..];
        for (idx, part) in line.split('|').enumerate() {
            if idx == 0 {
                for cap in reg.captures_iter(part) {
                    winning.insert(cap[0].parse::<i64>().unwrap());
                }
            } else {
                let mut score = 0;
                for cap in reg.captures_iter(part) {
                    if winning.contains(&cap[0].parse::<i64>().unwrap()) {
                        score += 1;
                    }
                }
                for forward in (card_at + 1)..(card_at + 1 + score) {
                    cards[forward] += 1 * cards[card_at];
                }
            }
        }
    }
    let sum: i64 = cards.iter().take(my_lines).sum();
    println!("sum: {}", sum);
}
