use regex::Regex;

pub fn day2_1() {
    let s = std::fs::read_to_string("src/day2.txt").unwrap();

    let reg_cols = Regex::new(r"blue|red|green").unwrap();
    let reg_nums = Regex::new(r"\d+").unwrap();

    let mut total = 0;

    let red = 12;
    let green = 13;
    let blue = 14;
    for (game_id, line) in s.lines().enumerate() {
        let (_, data) = line.split_once(':').unwrap();
        let chunks: Vec<&str> = data.split(';').collect();
        let mut any_chunk_failed = false;
        for chunk in chunks {
            let mut c_red = 0;
            let mut c_green = 0;
            let mut c_blue = 0;
            let cols = reg_cols.captures_iter(chunk);
            let nums = reg_nums.captures_iter(chunk);
            for (col, num) in cols.zip(nums) {
                println!("{}", &num[0]);
                let real_num: u64 = num[0].parse().unwrap();
                match &col[0] {
                    "red" => c_red += real_num,
                    "green" => c_green += real_num,
                    "blue" => c_blue += real_num,
                    it => panic!("got {}", it),
                }
            }
            println!("cols: {}, {}, {}", c_red, c_green, c_blue);
            if c_red > red || c_blue > blue || c_green > green {
                any_chunk_failed = true;
            }
        }
        if !any_chunk_failed {
            total += game_id + 1;
        }
    }
    println!("total: {}", total);
}

pub fn day2_2() {
    let s = std::fs::read_to_string("src/day2.txt").unwrap();

    let reg = Regex::new(r"(\d+) (blue|red|green)").unwrap();

    let mut total = 0;

    for (game_id, line) in s.lines().enumerate() {
        let (_, data) = line.split_once(':').unwrap();
        let chunks: Vec<&str> = data.split(';').collect();
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for chunk in chunks {
            let mut c_red = 0;
            let mut c_green = 0;
            let mut c_blue = 0;
            for m in reg.captures_iter(chunk) {
                let real_num: u64 = m[1].parse().unwrap();
                match &m[2] {
                    "red" => c_red += real_num,
                    "green" => c_green += real_num,
                    "blue" => c_blue += real_num,
                    it => panic!("got {}", it),
                }
            }
            if c_red > min_red {
                min_red = c_red;
            }
            if c_green > min_green {
                min_green = c_green;
            }
            if c_blue > min_blue {
                min_blue = c_blue;
            }
        }
        total += min_red * min_green * min_blue;
    }
    println!("total: {}", total);
}
