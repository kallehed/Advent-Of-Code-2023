use regex::Regex;

pub fn day6_1() {
    // let s = std::fs::read_to_string("src/day6.txt").unwrap();
    // let times = [7isize, 15, 30];
    // let dists = [9isize, 40, 200];
    let times = [60isize, 94, 78, 82];
    let dists = [475isize, 2138, 1015, 1650];

    let mut total = 1;

    for (&race_time, &race_dist) in times.iter().zip(dists.iter()) {
        let mut success = 0;
        for press_time in 1..race_time {
            let got = (race_time - press_time) * press_time;
            if got > race_dist {
                success += 1;
            }
        }
        // success += success - 2 + idx % 2;

        println!("got: {}", success);
        total *= success;
    }
    println!("total: {}", total);
}

pub fn day6_2() {
    // let s = std::fs::read_to_string("src/day6.txt").unwrap();
    // let times = [7isize, 15, 30];
    // let dists = [9isize, 40, 200];
    let times = [60947882isize];
    let dists = [475213810151650isize];

    let mut total = 1;

    for (&race_time, &race_dist) in times.iter().zip(dists.iter()) {
        let mut success = 0;
        for press_time in 1..race_time {
            let got = (race_time - press_time) * press_time;
            if got > race_dist {
                success += 1;
            }
        }
        // success += success - 2 + idx % 2;

        println!("got: {}", success);
        total *= success;
    }
    println!("total: {}", total);
}
