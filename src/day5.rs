use regex::Regex;

pub fn day5_1() {
    let s = std::fs::read_to_string("src/day5.txt").unwrap();

    let mut map: Vec<Vec<(isize, isize, isize)>> = Vec::new();

    let mut nums = Vec::<isize>::new();

    let reg = Regex::new(r"\d+").unwrap();

    let (first_line, rest) = s.split_once('\n').unwrap();

    for cap in reg.captures_iter(first_line) {
        let find = &cap[0];
        nums.push(find.parse().unwrap());
    }
    println!("nums: {:?}", nums);

    for line in rest.split('\n') {
        if line.ends_with(':') {
            map.push(Vec::new());
            continue;
        }
        if line.len() == 0 {
            continue;
        }
        let mut imports = [0; 3];
        for (idx, cap) in reg.captures_iter(line).enumerate() {
            imports[idx] = cap[0].parse().unwrap();
        }
        // destination,  source, and length
        map.last_mut()
            .unwrap()
            .push((imports[1], imports[0], imports[2]));
    }
    println!("end: {:?}", map);

    for ma in map {
        // map nums through ma
        nums = nums
            .iter()
            .map(|num| {
                for m in ma.iter() {
                    if (m.0..(m.0 + m.2)).contains(num) {
                        return (*num - m.0) + m.1;
                    }
                }
                return *num;
            })
            .collect();
    }
    nums.sort();
    println!("nums: {:?}", nums[0]);
}

pub fn day5_2() {
    let s = std::fs::read_to_string("src/day5.txt").unwrap();

    let mut map: Vec<Vec<(isize, isize, isize)>> = Vec::new();

    let mut bad_nums = Vec::<isize>::new();

    let reg = Regex::new(r"\d+").unwrap();

    let (first_line, rest) = s.split_once('\n').unwrap();

    for cap in reg.captures_iter(first_line) {
        let find = &cap[0];
        bad_nums.push(find.parse().unwrap());
    }
    // ranges
    let mut nums = Vec::<(isize, isize)>::new();

    for ns in bad_nums.chunks(2) {
        nums.push((ns[0], ns[1] + ns[0]));
    }

    println!("nums: ");
    for line in rest.split('\n') {
        if line.ends_with(':') {
            map.push(Vec::new());
            continue;
        }
        if line.len() == 0 {
            continue;
        }
        let mut imports = [0; 3];
        for (idx, cap) in reg.captures_iter(line).enumerate() {
            imports[idx] = cap[0].parse().unwrap();
        }
        // destination,  source, and length
        map.last_mut()
            .unwrap()
            .push((imports[1], imports[0], imports[2]));
    }
    println!("end: ");

    let mut new_nums = Vec::<(isize, isize)>::new();
    for ma in map {
        println!("nums: {:?}", nums);
        new_nums.clear();
        // map nums through ma

        for num in nums {
            let mut number = 0;
            for m in ma.iter() {
                let range = (num.0 - m.0 + m.1, num.1 - m.0 + m.1);
                let new_r = (range.0.max(m.1), range.1.min(m.1 + m.2));
                if new_r.1 - new_r.0 < 0 {
                    continue;
                }
                new_nums.push(new_r);
                number += 1;
            }
            if number < 1 {
                new_nums.push(num);
            }
        }
        new_nums.dedup();

        nums = new_nums.clone();
    }
    println!("nums: {:?}", nums);
    nums.sort();
    println!("nums: {:?}", nums[0]);
}
