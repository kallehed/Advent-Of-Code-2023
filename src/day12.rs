use std::collections::HashMap;

use regex::Regex;

pub fn day12_1() {
    let s = std::fs::read_to_string("src/day12.txt").unwrap().into_bytes();

    fn act_correct(s: &[u8], nums: &[isize]) -> bool {
        // println!("got: {:?}", s);
        let mut nums = nums.to_owned();
        let mut num_idx = 0;
        let mut on_num = false;
        for idx in 0..s.len() {
            if s[idx] == b'?' {
                panic!();
            }
            else if s[idx] == b'#' {
                on_num = true;
                if num_idx >= nums.len() {return false;}
                nums[num_idx] -= 1;
                if nums[num_idx] < 0 {return false;}
            }
            else if s[idx] == b'.' {
                if num_idx < nums.len() && on_num && nums[num_idx] != 0 {
                    return false;
                }
                if num_idx < nums.len() && nums[num_idx] == 0 {
                    num_idx += 1;
                    on_num = false;
                }
            }
        }
        if num_idx == nums.len() || (num_idx == nums.len() - 1 && nums.iter().sum::<isize>() == 0) {
            // println!("bad: {:?}, hm: {:?}", nums, s);
            return true;
        }
        else {
            return false;
        }

    }

    fn check_correct(s: &mut [u8], at: isize, counter: &mut isize, nums: &[isize]) {
        for idx in at..s.len() as isize {
            if s[idx as usize] == b'?' {
                // split universes
                s[idx as usize] = b'.';
                check_correct(s, idx + 1, counter, nums);
                s[idx as usize] = b'#';
                check_correct(s, idx + 1, counter, nums);
                s[idx as usize] = b'?';
                return;
            }
        }
        // check correctness, if so inc counter
        if act_correct(s, nums) {
            *counter += 1;
        }

    }

    let mut total = 0;
    for line in s.split(|x| *x == b'\n') {
        if line.len() == 0 {continue;}
        let mut i = line.split(|x| *x == b' ');
        let springs = i.next().unwrap();
        let mut springs = springs.to_owned();
        let nums = i.next().unwrap();
        let nums: Vec<isize> = nums.split(|x| *x == b',').map(|x| std::str::from_utf8(x).unwrap().parse().unwrap()).collect();

        let mut counter = 0;
        check_correct(&mut springs, 0, &mut counter, &nums);
        total += counter;

        println!("counter: {:?}", counter);
    }
    println!("total: {}", total);
}

pub fn day12_2() {

    let s = std::fs::read_to_string("src/day12.txt").unwrap().into_bytes();

    fn check_correct(mut string: &[u8], lengths: &[isize], cache: &mut HashMap<(isize, isize), isize>) -> isize {
        if lengths.len() == 0 {
            // no nums left, check that there are no # left 
            if string.iter().all(|x| *x != b'#') {return 1} else {return 0}
        }

        match &cache.get(&(string.len() as isize, lengths.len() as isize)) {
            Some(&cached_total) => return cached_total,
            None => (),
        }

        let orig_string_len = string.len();
    
        let my_len = lengths[0];

        let mut total = 0;

        loop {
            if string.len() < my_len as usize {
                // stop
                break;
            }
            let can_fill_here = string.iter().take(my_len as usize).all(|&x| x == b'#' || x == b'?');
            // look at the one after this one, it can't be a #
            let good_after = match string.iter().nth(my_len as usize) {
                Some(&c) => c != b'#',
                None => {
                    // this is the end
                    if can_fill_here && lengths.len() == 1 {
                        total += 1;
                    }
                    break;
                },
            };
            if can_fill_here && good_after {
                total += check_correct(&string[(my_len + 1) as usize..], &lengths[1..], cache);
            }
            match string[0] {
                b'.' => (),
                b'#' => {
                    break; // we can't continue after #
                }
                b'?' => (),
                _ => panic!()
            }
            string = &string[1..];
        }
        
        cache.insert((orig_string_len as isize, lengths.len() as isize), total);

        return total;
    }

    let mut cache = HashMap::new();
    let mut total = 0;
    for line in s.split(|x| *x == b'\n') {
        if line.len() == 0 {continue;}
        let mut i = line.split(|x| *x == b' ');
        let springs = i.next().unwrap();
        let springs = springs.to_owned();
        let mul = 5;
        let springs: Vec<_> = springs.iter().chain(std::iter::once(&b'?')).map(|x| *x).cycle().take((springs.len() + 1) * mul - 1).collect();
        println!("sp: {:?}", springs);
        let nums = i.next().unwrap();
        let nums: Vec<isize> = nums.split(|x| *x == b',').map(|x| std::str::from_utf8(x).unwrap().parse().unwrap()).collect();
        let nums: Vec<isize> = nums.iter().cycle().take(nums.len() * mul).map(|x| *x).collect();

        let mut placings = vec![];
        let mut offset = 0;
        for num in &nums {
            placings.push(offset);
            offset += num + 1;
        }

        cache.clear();

        let counter = check_correct(&springs, &nums, &mut cache);
        total += counter;

        println!("counter: {:?}", counter);
    }
    println!("total: {}", total);
}
