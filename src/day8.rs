use std::collections::HashMap;

use regex::Regex;

pub fn day8_1() {
    let s = std::fs::read_to_string("src/day8.txt").unwrap();

    let (goes, rest) = s.split_once('\n').unwrap();

    let mut go_instrs = Vec::new();

    for go in goes.chars() {
        go_instrs.push(
            match go {
                'L' => 0,
                'R' => 1,
                a => panic!("wierd chard: {a}")
        });
    }
    println!("go insts: {:?}", go_instrs);

    let (_, rest) = rest.split_once('\n').unwrap();

    let reg = Regex::new(r"(...) = \((...), (...)\)").unwrap();
    let mut inc_num = 0;
    let mut map: HashMap<isize, (isize, isize)> = HashMap::new();

    let mut place_to_num: std::collections::HashMap<String, isize> = std::collections::HashMap::new();
    for line in rest.lines() {
        println!("{}", line);
        for cap in reg.captures_iter(line) {
            println!("a:");
            let from = &cap[1]; 
            let left = &cap[2]; 
            let right = &cap[3]; 
            if !place_to_num.contains_key(from) {
                place_to_num.insert(from.to_string(), inc_num);
                inc_num += 1;
            }
            if !place_to_num.contains_key(left) {
                place_to_num.insert(left.to_string(), inc_num);
                inc_num += 1;
            }
            if !place_to_num.contains_key(right) {
                place_to_num.insert(right.to_string(), inc_num);
                inc_num += 1;
            }
            map.insert(*place_to_num.get(from).unwrap(), (*place_to_num.get(left).unwrap(), *place_to_num.get(right).unwrap()));
        }

    }
    println!("map: {:?}", map);

    let start = *place_to_num.get("AAA").unwrap();
    let end = *place_to_num.get("ZZZ").unwrap();

    let mut num_steps = 0;

    let mut at = start;
    'leloop: loop {
        for go in go_instrs.iter() {
            let next = map.get(&at).unwrap();
            num_steps += 1;
            let next_num = match go {
                0 => next.0,
                1 => next.1,
                _ => panic!()
            };
            at = next_num;
            if at == end {
                break 'leloop;
            }
        }
    }
    println!("took: {}", num_steps);
}

pub fn day8_2() {
    let s = std::fs::read_to_string("src/day8.txt").unwrap();

    let (goes, rest) = s.split_once('\n').unwrap();

    let mut go_instrs = Vec::new();

    for go in goes.chars() {
        go_instrs.push(
            match go {
                'L' => 0,
                'R' => 1,
                a => panic!("wierd chard: {a}")
        });
    }
    println!("go insts: {:?}", go_instrs);

    let (_, rest) = rest.split_once('\n').unwrap();

    let reg = Regex::new(r"(...) = \((...), (...)\)").unwrap();
    let mut inc_num = 0;
    let mut map: HashMap<isize, (isize, isize)> = HashMap::new();

    use std::collections::HashSet;
    let mut ends_with_z: HashSet<isize> = HashSet::new();
    let mut ends_with_a: HashSet<isize> = HashSet::new();

    let mut place_to_num: std::collections::HashMap<String, isize> = std::collections::HashMap::new();
    for line in rest.lines() {
        println!("{}", line);
        for cap in reg.captures_iter(line) {
            println!("a:");
            let from = &cap[1]; 
            let left = &cap[2]; 
            let right = &cap[3]; 
            if !place_to_num.contains_key(from) {
                place_to_num.insert(from.to_string(), inc_num);
                inc_num += 1;
            }
            if !place_to_num.contains_key(left) {
                place_to_num.insert(left.to_string(), inc_num);
                inc_num += 1;
            }
            if !place_to_num.contains_key(right) {
                place_to_num.insert(right.to_string(), inc_num);
                inc_num += 1;
            }
            if from.ends_with("Z") {
                ends_with_z.insert(*place_to_num.get(from).unwrap());
            }
            if from.ends_with("A") {
                ends_with_a.insert(*place_to_num.get(from).unwrap());
            }
            if left.ends_with("Z") {
                ends_with_z.insert(*place_to_num.get(left).unwrap());
            }
            if right.ends_with("Z") {
                ends_with_z.insert(*place_to_num.get(right).unwrap());
            }
            map.insert(*place_to_num.get(from).unwrap(), (*place_to_num.get(left).unwrap(), *place_to_num.get(right).unwrap()));
        }

    }
    println!("map: {:?}", map);
    println!("ends with z: {:?}", ends_with_z);
    println!("ends with a: {:?}", ends_with_a);


    let start_at: i64 = (go_instrs.len() as i64 * map.len() as i64) * 20;
    let mut vecs = Vec::new();
    for one in ends_with_a {
        let mut one = one;
        let mut start_at_place: isize = -1;
        let mut num_steps: isize = 0;
        let mut go_idx_at_start: isize = -1;
        let mut on_good_vec = Vec::new();
        'leloop: loop {
            for (go_idx, go) in go_instrs.iter().enumerate() {
                let next = map.get(&one).unwrap(); 
                    let next_num = match go {
                        0 => next.0,
                        1 => next.1,
                        _ => panic!()
                    };
                    one = next_num;
                    num_steps += 1;
                    if one == start_at_place && go_idx_at_start == go_idx as _ {
                        println!("took {} to get back to {}", num_steps - start_at as isize, start_at_place);
                        break 'leloop;
                    }
                    if num_steps == start_at as isize {
                        start_at_place = one;
                        go_idx_at_start = go_idx as _;
                    }
                    if start_at_place != -1 {
                        on_good_vec.push(ends_with_z.contains(&one));
                    }
            }
        }
        vecs.push(on_good_vec);
        continue;
    }
    for vec in vecs.iter() {
        println!("size: {}", vec.len());
    }
    // println!("{:?}", vecs[0]);

    // assume each one only contains one true value
   
    let mut good_places = Vec::new();
    let mut sizes: Vec<i64> = Vec::new();
    for (vecidx, vec) in vecs.iter().enumerate() {
        sizes.push(vec.len() as _);
        for (idx, p) in vec.iter().enumerate() {
            if *p {
                good_places.push(idx);
            }
        }
    }
    println!("good places: {:?}", good_places);
    assert_eq!(good_places.len(), vecs.len());
    println!("sizes: {:?}", sizes);

    println!("started with: {}", start_at);

    println!(" put this into wolframalpha and you will get the answer ");


    // println!("took: {}", num_steps);
}
