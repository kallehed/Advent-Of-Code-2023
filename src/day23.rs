use std::collections::HashSet;

use regex::Regex;
pub fn day23_1() {
    std::thread::Builder::new().stack_size(1024 * 2048).spawn(|| {
        day23_1_real();
    }).unwrap();
    loop{}
}

fn day23_1_real() {
    let s = std::fs::read_to_string("src/day23.txt").unwrap();
    let mut map = Vec::new();
    #[derive(Debug, Clone, Copy)]
    enum Tile {
        Air,
        Block,
        Force(i8, i8)
    }
    for line in s.lines() {
        map.push(vec![]);
        for ch in line.chars() {
            map.last_mut().unwrap().push( 
                match ch {
                    '#' => Tile::Block,
                    '.' => Tile::Air,
                    '>' => Tile::Force(0, 1),
                    '<' => Tile::Force(0, -1),
                    '^' => Tile::Force(-1, 0),
                    'v' => Tile::Force(1, 0),
                    _ => panic!()
                })
        }
    }
    println!("map: {:?}", map);
    fn recurse(map: &[Vec<Tile>], pos: (i16, i16), set: &mut HashSet<(i16, i16)>) -> i64 {
        let ways = [(1,0),(-1,0),(0,1),(0,-1)]; 

        // valid where we are?

        if pos.0 < 0 || pos.1 < 0 || pos.0 >= map.len() as i16 || pos.1 >= map[0].len() as i16 {
            return 0;
        }
        // check if we won

        // add position to set as of now
        if pos.0 == map.len() as i16 - 1 {
            // reached end
            return set.len() as i64;
        }

        if !set.insert(pos) {
            // was not newly inserted
            return 0;
        }

        let res = match map[pos.0 as usize][pos.1 as usize] {
            Tile::Air => {
                let mut best = 0;
                for way in ways.iter() {
                    let wy = way.0;
                    let wx = way.1;
                    let res = recurse(map, (pos.0 + wy as i16, pos.1 + wx as i16), set);
                    if res > best {
                        best = res;
                    }
                }
                best
            },
            Tile::Block => 0,
            Tile::Force(wy, wx) => {
                recurse(map, (pos.0 + wy as i16, pos.1 + wx as i16), set)
            },
        };
        set.remove(&pos);
        res
    }

    let mut set = HashSet::new();

    let res = recurse(&map, (0, 1), &mut set);
    println!("res: {}", res);
}

pub fn day23_2() {
    let j = std::thread::Builder::new().stack_size(1024 * 2048).spawn(|| {
        day23_2_real();
    }).unwrap();
    j.join().unwrap();
}

pub fn day23_2_real() {
    let s = std::fs::read_to_string("src/day23.txt").unwrap();
    let mut map = Vec::new();
    #[derive(Debug, Clone, Copy, PartialEq)]
    enum Tile {
        Air,
        Block,
    }
    for line in s.lines() {
        map.push(vec![]);
        for ch in line.chars() {
            map.last_mut().unwrap().push( 
                match ch {
                    '#' => Tile::Block,
                    _ => Tile::Air,
                })
        }
    }
    println!("map: {:?}", map);
    const WAYS: [(i32, i32); 4] = [(-1,0),(0,1),(1,0),(0,-1),]; 
    fn recurse(map: &mut [Vec<Tile>], pos: (i16, i16), steps: i64, highest: &mut i64) -> i64 {

        // valid where we are?

        if pos.0 < 0 || pos.1 < 0 || pos.0 >= map.len() as i16 || pos.1 >= map[0].len() as i16 {
            return 0;
        }

        if map[pos.0 as usize][pos.1 as usize] != Tile::Air {
            return 0;
        }

        if pos.0 == map.len() as i16 - 1 {
            // reached end
            if steps > *highest {
                *highest = steps;
                println!("cur highest: {:?}", *highest);
            }
            return steps;
        }

        // add position to set as of now
        map[pos.0 as usize][pos.1 as usize] = Tile::Block;

        let res = {
            let mut best = 0;
            for (idx, way) in WAYS.iter().enumerate() {
                let wy = way.0;
                let wx = way.1;
                let res = recurse(map, (pos.0 + wy as i16, pos.1 + wx as i16), steps + 1, highest);
                if res > best {
                    best = res;
                }
            }
            best
        };
        map[pos.0 as usize][pos.1 as usize] = Tile::Air;
        res
    }

    let mut highest = 0;

    let res = recurse(&mut map, (0, 1), 0, &mut highest);
    println!("res: {} END", res);
}
