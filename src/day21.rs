use std::collections::{HashSet, HashMap};

use regex::Regex;

pub fn day21_1() {
    let s = std::fs::read_to_string("src/day21.txt").unwrap();
    let mut map = Vec::new();
    let mut start = (-1,-1);
    for (y_idx, line) in s.lines().enumerate() {
        map.push(Vec::new());
        for (x_idx ,ch) in line.chars().enumerate() {
            let item = match ch {
                '#' => true,
                '.' => false,
                'S' => {
                    start = (y_idx as isize, x_idx as isize); 
                    false
                }
                _ => panic!()
            };
            map.last_mut().unwrap().push(item);
        }
    }
    println!("map: {:?}, start: {:?}", map, start);

    {
        let mut cur_pos = HashSet::<(isize, isize)>::new();
        let mut next_pos = HashSet::<(isize, isize)>::new();
        cur_pos.insert(start);

        for _ in 0..64 {
            for cur in cur_pos.iter() {
                let mods = [(1,0), (-1,0), (0, 1), (0, -1)];
                for md in mods {
                    let new = (cur.0 + md.0, cur.1 + md.1);
                    if !map[new.0 as usize][new.1 as usize] && !cur_pos.contains(&new) {
                        next_pos.insert(new);
                    }
                }
            }
            cur_pos = next_pos.clone();
            next_pos.clear();
        }
        println!("total: {}", cur_pos.len());
    }

}

pub fn day21_2() {
    let s = std::fs::read_to_string("src/day21.txt").unwrap();
    let mut map = Vec::new();
    let mut start = (-1,-1);
    for (y_idx, line) in s.lines().enumerate() {
        map.push(Vec::new());
        for (x_idx ,ch) in line.chars().enumerate() {
            let item = match ch {
                '#' => true,
                '.' => false,
                'S' => {
                    start = (y_idx as isize, x_idx as isize); 
                    false
                }
                _ => panic!()
            };
            map.last_mut().unwrap().push(item);
        }
    }
    // println!("map: {:?}, start: {:?}", map, start);

    {
        let mut cur_pos = HashSet::<((isize, isize), (i16, i16))>::new();
        let mut next_pos = HashSet::<((isize, isize), (i16, i16))>::new();
        cur_pos.insert(((0,0), (start.0 as i16, start.1 as i16)));

        let mut chunk_dist = 0;

        let mut total = 0;

        // get some values and put them into wolfram alpha, make a quadratic equation and use it at
        // the right number! 

        for steps in 0..1000 {
            // println!("count: {}", cur_pos.iter().filter(|x| x.0.0 == -3 && x.0.1 == 2).count()  );
            
            if (steps - 65 ) % 131 == 0 {
                println!("{} ", cur_pos.len());
            }

            for cur in cur_pos.iter() {
                let mods = [(1,0), (-1,0), (0, 1), (0, -1)];
                for md in mods {
                    let mut new = (cur.1.0 + md.0, cur.1.1 + md.1);
                    let mut chunk = cur.0;
                    if new.0 < 0 {
                        chunk.0 -= 1;
                        new.0 += map.len() as i16;
                    }
                    else if new.0 >= map.len() as i16 {
                        chunk.0 += 1;
                        new.0 -= map.len() as i16;
                    }
                    if new.1 < 0 {
                        chunk.1 -= 1;
                        new.1 += map[0].len() as i16;
                    }
                    else if new.1 >= map[0].len() as i16 {
                        chunk.1 += 1;
                        new.1 -= map[0].len() as i16;
                    }
                    // new = (new.0.rem_euclid(map.len() as isize), new.1.rem_euclid(map[0].len() as isize));
                    // println!("{:?} into {:?}", old_new, new);
                    if !map[new.0 as usize][new.1 as usize] && !cur_pos.contains(&(chunk, new)) {
                        next_pos.insert((chunk, new));
                    }
                }
            }
            cur_pos.clone_from(&next_pos);
            // cur_pos = next_pos.clone();
            next_pos.clear();
        }
        println!("rest: {}, total: {}", cur_pos.len(), cur_pos.len() + total);
    }

}
