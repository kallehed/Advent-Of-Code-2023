use std::collections::HashSet;

use regex::Regex;

pub fn day24_1() {
    let s = std::fs::read_to_string("src/day24.txt").unwrap();
    let reg = Regex::new(r"(-?\d+)").unwrap();
    let mut stones = vec![];
    for line in s.lines() {
        stones.push([0.0,0.0,0.0,0.0,0.0,0.0]);
        for (idx, cap) in reg.captures_iter(line).enumerate() {
            stones.last_mut().unwrap()[idx] = cap[0].parse::<i64>().unwrap() as f64;
        }   
    }
    println!("stones: {:?}", stones);
    // for each pair do linear intersection
    let stones = stones;

    // 17384 not right
    // 129 not right

    let mut total = 0;
    for i in 0..(stones.len() - 1) {
        for j in (i + 1)..stones.len() {
            // if set.contains(&i) {continue};
            // if set.contains(&j) {continue};
            let s0 = stones[i];
            let s1 = stones[j];
            let y_div_x = s0[4] / s0[3];
            let z_div_w = s0[4] / s0[3];
            let v = (s0[1] - s0[0] * y_div_x - s1[1] + s1[0] * y_div_x) / (s1[4] - s1[3] * y_div_x);
            let t = (s1[0] - s0[0] + v * s1[3]) / s0[3];
            if v < 0.0 || t < 0.0 {
                continue;
            }
            let x_pos = s1[0] + v * s1[3];
            let y_pos = s1[1] + v * s1[4];
            println!("poses: {}, {}", x_pos, y_pos);

            if x_pos >= 200000000000000.0 && y_pos >= 200000000000000.0 && x_pos <= 400000000000000.0 && y_pos <= 400000000000000.0 {
                total += 1;
                println!("correct");
            }
            // if x_pos >= 7.0 && y_pos >= 7.0 && x_pos <= 24.0 && y_pos <= 24.0 {
            //     total += 1;
            // }
        }
    }
    println!("total real total: {}", total);
}
pub fn day24_2() {
    let s = std::fs::read_to_string("src/day24.txt").unwrap();
    let reg = Regex::new(r"(-?\d+)").unwrap();
    let mut stones = vec![];
    for line in s.lines() {
        stones.push([0,0,0,0,0,0]);
        for (idx, cap) in reg.captures_iter(line).enumerate() {
            stones.last_mut().unwrap()[idx] = cap[0].parse::<i64>().unwrap();
        }   
    }
    println!("stones: {:?}", stones);
    // for each pair do linear intersection
    let stones = stones;

    // 17384 not right
    // 129 not right

    let start_time = 1;

    // print it out to be used with python and z3, look in day24.py for rest of solution
    for stone in stones.iter() {
        println!("a + t*x == {} + t * ({})", stone[0], stone[3]); 
        println!("b + t*y == {} + t * ({})", stone[1], stone[4]); 
        println!("c + t*z == {} + t * ({})", stone[2], stone[5]); 
    }

    // my manual brute force solution did not really work at all, 
    // it tried to look at a pair, and then see if those were the starting 
    // ones that were hit, if the rest were hit as well
    // took too long to compute, so did python instead

    // for start_time in 1..10 {
    //     for time_to_second_rock in 9..10 {
    //         for i in 0..(stones.len()) {
    //             for j in (0)..stones.len() {
    //                 if i == j {continue;}
    //                 let mut s0 = stones[i];
    //                 let mut s1 = stones[j];
    //                 for _ in 0..start_time {
    //                     s0[0] += s0[3];
    //                     s0[1] += s0[4];
    //                     s0[2] += s0[5];
    //                     s1[0] += s1[3];
    //                     s1[1] += s1[4];
    //                     s1[2] += s1[5];
    //                 }
    //                 for _ in 0..time_to_second_rock {
    //                     s1[0] += s1[3];
    //                     s1[1] += s1[4];
    //                     s1[2] += s1[5];
    //                 }
    //                 let mut dif = [s1[0] - s0[0], s1[1] - s0[1], s1[2] - s0[2]];
    //                 // println!("dif: {:?}", dif);
    //                 if (dif[0] % time_to_second_rock == 0) && (dif[1] % time_to_second_rock == 0) && (dif[2] % time_to_second_rock == 0) {
    //                     // we can divide up this time so we get a integer movement
    //                     dif.iter_mut().for_each(|x| *x /= time_to_second_rock);
    //                     // println!("dif: {:?}", dif);
    //                     if dif == [-3, 1, 2] {
    //                         // println!("passed, pos: {:?}, start_time: {}, time for second: {}", s0, start_time, time_to_second_rock);
    //                     }
    //
    //                     // now do simulation
    //                     let start_pos = [s0[0] - dif[0] * start_time, s0[1] - dif[1] * start_time, s0[2] - dif[2] * start_time];
    //                     // println!("start pos: {:?}", start_pos);
    //                     let mut pos = start_pos;
    //                     let mut collided_with = 0;
    //                     for time in 0..1000 {
    //                         
    //                         if stones.iter().any(|&stone| [stone[0] + stone[3] * time, stone[1] + stone[4] * time, stone[2] + stone[5] * time] == pos) {
    //                             // found a rock
    //                             collided_with += 1;
    //                             // println!("collied with: {}", collided_with);
    //                             if collided_with == stones.len() {
    //                                 // FOUND ANSWER
    //                                 println!("start_pos: {:?}, result: {}", start_pos, start_pos.iter().sum::<i64>());
    //                             }
    //                         }
    //
    //                         pos[0] += dif[0];
    //                         pos[1] += dif[1];
    //                         pos[2] += dif[2];
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }

}
