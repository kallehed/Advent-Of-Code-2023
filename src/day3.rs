use regex::Regex;

// go through all characters, find numeric ones and check for symbols near them
pub fn day3_1() {
    let s = std::fs::read_to_string("src/day3.txt").unwrap().into_bytes();

    let len = s.iter().position(|&x| x == b'\n').unwrap() + 1;
    println!("len: {}", len);

    let mut total = 0;

    let mut start_numeric: i64 = -1;
    let mut was_good_num = false;
    for (i, c) in s.iter().enumerate() {

        if c.is_ascii_digit() {
            if start_numeric == -1 {
                start_numeric = i as _;
            }
            // check neighbors for weird symbols
            let i = i as i64;
            let len = len as i64;
            let neighs = [i - 1, i + 1, i + len, i + len -1, i + len + 1, i - len - 1, i - len, i -len + 1];
            for neigh in neighs {
                if neigh < 0 || neigh >= s.len() as i64 {continue;}
                let co = s[neigh as usize];
                if !co.is_ascii_digit() && co != b'.' && co != b'\n' {
                    was_good_num = true;
                }
            } 

        }
        else if start_numeric != -1 {
            // found whole number 
            let num_str = &s[(start_numeric as usize)..i];
            let num: usize = std::str::from_utf8(num_str).unwrap().parse().unwrap();
            if was_good_num {
                total += num;
            }
            start_numeric = -1;
            was_good_num = false;

        }

    }
    println!("total: {}", total);
}

// look for numbers, and if they have an asterisk beside them, then put yourself in a hashmap with
// the asterisks position - if another number finds itself with the same asterisk it will find the
// previous number in the hashmap and add their product to the total
pub fn day3_2() {
    let s = std::fs::read_to_string("src/day3.txt").unwrap().into_bytes();


    let len = s.iter().position(|&x| x == b'\n').unwrap() + 1;
    println!("len: {}", len);

    let mut total = 0;

    use std::collections::HashMap;

    let mut set: HashMap<i64, i64> = HashMap::new();

    let mut start_numeric: i64 = -1;
    let mut was_good_num = false;
    let mut ast_pos = 0;
    for (i, c) in s.iter().enumerate() {
        if c.is_ascii_digit() {
            if start_numeric == -1 {
                start_numeric = i as _;
            }
            // check neighbors for weird symbols
            let i = i as i64;
            let len = len as i64;
            let neighs = [i - 1, i + 1, i + len, i + len -1, i + len + 1, i - len - 1, i - len, i -len + 1];
            for neigh in neighs {
                if neigh < 0 || neigh >= s.len() as i64 {continue;}
                let co = s[neigh as usize];
                if co == b'*' {
                    ast_pos = neigh;
                    was_good_num = true;
                }
            } 
        }
        else if start_numeric != -1 {
            // found whole number 
            let num_str = &s[(start_numeric as usize)..i];
            let num: i64 = std::str::from_utf8(num_str).unwrap().parse().unwrap();
            if was_good_num {
                if set.contains_key(&ast_pos) {
                    total += *set.get(&ast_pos).unwrap() * num as i64;
                }
                else {
                    set.insert(ast_pos as i64, num as i64);
                }
            }
            start_numeric = -1;
            was_good_num = false;
        }
    }
    println!("total: {}", total);
}
