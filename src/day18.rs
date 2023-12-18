use std::collections::HashSet;

use regex::Regex;

pub fn day18_1() {
    let s = std::fs::read_to_string("src/day18.txt").unwrap();
    let mut goes = vec![];
    {
        for line in s.lines() {
            let dir = match line.chars().nth(0).unwrap() {
                'R' => (0,1),
                'L' => (0, -1),
                'U' => (-1,0),
                'D' => (1,0),
                _ => panic!()
            };
            let steps = line.split(' ').nth(1).unwrap().parse::<i32>().unwrap();

            goes.push((dir, steps));
        }
    }
    println!("goes: {:?}", goes);

    let mut map = HashSet::<(i32, i32)>::new();
    {
        let mut y = 0;
        let mut x = 0;
        // map.insert((0, 0));

        for &go in goes.iter() {

            for _ in 0..go.1 {
                y += go.0.0;
                x += go.0.1;
                map.insert((y,x));
            }
        }
        println!("map: {:?}", map);

        for y in (-200)..(200) {
            println!();
            for x in (-20)..(150) {
                let mut ch = match map.contains(&(y,x)) {
                    true => '#',
                    false => '.'
                };
                if y == 0 && x == 0{
                    ch = 'X';
                }
                print!("{}", ch);
            }
        }
    }

    println!("map size: {}", map.len());

    {
        let mut interior = HashSet::new();

        let mut new_places = Vec::new();
        new_places.push((5,5));

        while new_places.len() != 0 {

            let mut new_place = new_places.pop().unwrap();

            if (!map.contains(&new_place)) &&  (!interior.contains(&new_place)) {
                interior.insert(new_place);

                // also add neighbors
                //
                new_place.0 += 1;
                new_places.push(new_place);

                new_place.0 -= 2;
                new_places.push(new_place);

                new_place.0 += 1;
                new_place.1 += 1;
                new_places.push(new_place);

                new_place.1 -= 2;
                new_places.push(new_place);
            }
        }
        println!("interior: {:?}, sum: {}", interior.len(),  interior.len() + map.len());
    }

}

pub fn day18_2() {
    let s = std::fs::read_to_string("src/day18.txt").unwrap();
    let mut goes = vec![];
    {
        for line in s.lines() {
            let dir = match line.split(' ').nth(2).unwrap().chars().nth(7).unwrap() {
                '0' => (0,1),
                '2' => (0, -1),
                '3' => (-1,0),
                '1' => (1,0),
                _ => panic!()
            };
            let hex = &line.split(' ').nth(2).unwrap()[2..7];
            println!("hex: {:?}", hex);

            let mut total = 0;
            let mut base = 1;
            for c in hex.chars().rev() {
                if c.is_ascii_digit() {
                    total += (c as u8 - b'0') as i32 * base;
                }
                else {
                    println!("{}", c);
                    total += (10 + (c as u8 - b'a')) as i32 * base;
                }
                base *= 16;
            }

            let steps = total;
            goes.push((dir, steps));
        }
    }
    println!("goes: {:?}", goes);

    let mut map = Vec::<(i32, i32)>::new();
    {
        let mut y = 0;
        let mut x = 0;
        // map.insert((0, 0));

        for &go in goes.iter() {

            for _ in 0..go.1 {
                y += go.0.0;
                x += go.0.1;
                map.push((y,x));
            }
        }
        // println!("map: {:?}", map);

        // for y in (-200)..(200) {
        //     println!();
        //     for x in (-20)..(150) {
        //         let mut ch = match map.contains(&(y,x)) {
        //             true => '#',
        //             false => '.'
        //         };
        //         if y == 0 && x == 0{
        //             ch = 'X';
        //         }
        //         print!("{}", ch);
        //     }
        // }
    }
    println!("map size: {}", map.len());

    {
        let mut shoe: i64 = 0;
        for wind in map.windows(2) {
            shoe += wind[0].1 as i64 * wind[1].0 as i64 - wind[1].1 as i64 * wind[0].0 as i64;
        }
        shoe += (map.last().unwrap().1 * map.first().unwrap().0 - map.first().unwrap().1 * map.last().unwrap().0) as i64;
        shoe /= 2;

        println!("shoe: {:?}", shoe);

        let area = shoe + map.len() as i64 / 2 + 1;

        println!("area: {:?}", area);


    }

    // let mut min_y = 100000;
    // let mut min_x = 100000;
    // let mut max_y = -100000;
    // let mut max_x = -100000;
    // for &pos in map.iter() {
    //     if pos.0 < min_y {
    //         min_y = pos.0;
    //     } 
    //     if pos.0 > max_y {
    //         max_y = pos.0;
    //     }
    //     if pos.1 < min_x {
    //         min_x = pos.1;
    //     }
    //     if pos.1 > max_x {
    //         max_x = pos.1
    //     }
    // }
    // println!("min_y: {}, max_y: {}, min_x: {}, max_x: {}", min_y, max_y, min_x, max_x);

}



