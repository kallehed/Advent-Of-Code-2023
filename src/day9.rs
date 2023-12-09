use regex::Regex;

pub fn day9_1() {
    let s = std::fs::read_to_string("src/day9.txt").unwrap();

    fn algo(nums: &[i64]) -> i64 {
        let mut vec = Vec::new();
        for pair in nums.windows(2) {
            vec.push(pair[1] - pair[0]);
        }
        if vec.iter().all(|x|*x == vec[0]) {
            return vec[0];
        }
        else {
            let val = algo(&vec) + vec.last().unwrap();
            println!("passing on: {val}");
            return val ;
        }
    }
    
    let reg = Regex::new(r"-?\d+").unwrap();
    let mut vec = Vec::new();
    let mut total = 0;
    for line in s.lines() {
        vec.clear();
        for cap in reg.captures_iter(line) {
            let num: i64 = cap[0].parse().unwrap();
            println!("num: {}", num);
            vec.push(num);
        }
        let got = algo(&vec) + vec.last().unwrap();
        println!("got: {got}");
        total += got;
    }
    println!("total: {}", total);

}

pub fn day9_2() {

    let s = std::fs::read_to_string("src/day9.txt").unwrap();

    fn algo(nums: &[i64]) -> i64 {
        let mut vec = Vec::new();
        for pair in nums.windows(2) {
            vec.push(pair[1] - pair[0]);
        }
        if vec.iter().all(|x|*x == vec[0]) {
            return vec[0];
        }
        else {
            let val = -algo(&vec) + vec.first().unwrap();
            println!("passing on: {val}");
            return val ;
        }
    }
    
    let reg = Regex::new(r"-?\d+").unwrap();
    let mut vec = Vec::new();
    let mut total = 0;
    for line in s.lines() {
        vec.clear();
        for cap in reg.captures_iter(line) {
            let num: i64 = cap[0].parse().unwrap();
            println!("num: {}", num);
            vec.push(num);
        }
        let got = -algo(&vec) + vec.first().unwrap();
        println!("got: {got}");
        total += got;
    }
    println!("total: {}", total);
}
