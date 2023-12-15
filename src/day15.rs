use regex::Regex;

pub fn day15_1() {
    println!("part 1");
    let s = std::fs::read_to_string("src/day15.txt").unwrap().into_bytes();
    let s: Vec<_> = s.iter().map(|x| *x).filter(|&x| x != b'\n').collect();
    let mut total: isize = 0;
    for part in s.split(|&x| x == b',') {
        let mut cur_val = 0;

        for ch in part.iter() {
            let ch = *ch;
            cur_val += ch;
            cur_val *= 17;
        }
        println!("got {cur_val}");
        total += cur_val as isize;
    }
    println!("total {total}");
}

pub fn day15_2() {
    println!("part 2");

    let mut boxes: [Vec<(&[u8], u8)>;256] = [0;256].map(|_| vec![]);

    let s = std::fs::read_to_string("src/day15.txt").unwrap().into_bytes();
    let s: Vec<_> = s.iter().map(|x| *x).filter(|&x| x != b'\n').collect();
    let mut total: isize = 0;
    for mut part in s.split(|&x| x == b',') {
        let way;
        let mut foc = 0;
        if part[part.len() - 2] == b'=' {
            foc = part[part.len() - 1] - b'0';
            part = &part[..(part.len() - 2)];
            way = 0;
        }
        else if part[part.len() - 1] == b'-' {
            part = &part[..(part.len() - 1)];
            way = 1;
        }
        else {
            panic!();
        }
        let mut cur_val = 0;

        for ch in part.iter() {
            let ch = *ch;
            cur_val += ch;
            cur_val *= 17;
        }

        let box_idx = cur_val;

        if way == 0 { // =
            match boxes[box_idx as usize].iter().position(|&x| x.0 == part) {
                Some(pos) => {
                    boxes[box_idx as usize][pos].1 = foc;
                }
                None => {
                    boxes[box_idx as usize].push((part, foc));
                }
            }
        }
        else { // -
            // find label in box
            boxes[box_idx as usize].retain_mut(|x| {
                x.0 != part
            });
        }
    }
    println!("boxes: {:?}", boxes);
    
    for box_idx in 0..256 {
        for lens_idx in 0..boxes[box_idx].len() {
            let hm = (1 + box_idx as isize) * (1 + lens_idx as isize) * (boxes[box_idx][lens_idx].1 as isize);
            total += hm;
        }
    }
    println!("total {total}");
}
