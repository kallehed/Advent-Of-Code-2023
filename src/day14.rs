use std::collections::{HashMap, HashSet};

use regex::Regex;

pub fn day14_1() {
    println!("part 1;");
    let s = std::fs::read_to_string("src/day14.txt").unwrap();
    #[derive(Clone, Copy, Debug, PartialEq)]
    enum Tile {
        Fast,
        Stone,
        Air,
    }
    let mut map: Vec<Vec<Tile>> = Vec::new();
    for line in s.lines() {
        map.push(
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Air,
                    '#' => Tile::Fast,
                    'O' => Tile::Stone,
                    _ => panic!(),
                })
                .collect(),
        )
    }
    println!("map: {:?}", map);

    fn up_fall(map: &mut [Vec<Tile>]) {
        // loop until all are down
        loop {
            let mut changed = false;
            for y in 1..map.len() {
                for x in 0..map[0].len() {
                    if map[y][x] == Tile::Stone && map[y - 1][x] == Tile::Air {
                        map[y - 1][x] = Tile::Stone;
                        map[y][x] = Tile::Air;
                        changed = true;
                    }
                }
            }

            if !changed {
                break;
            }
        }
    }

    fn score_map(map: &[Vec<Tile>]) -> usize {
        let mut score = 0;
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if map[y][x] == Tile::Stone {
                    score += map.len() - y;
                }
            }
        }
        return score;
    }

    up_fall(&mut map);

    let total = score_map(&map);
    println!("total: {}", total);
}

pub fn day14_2() {
    println!("part 2;");
    let s = std::fs::read_to_string("src/day14.txt").unwrap();
    #[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
    enum Tile {
        Fast,
        Stone,
        Air,
    }
    let mut map: Vec<Vec<Tile>> = Vec::new();
    for line in s.lines() {
        map.push(
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Air,
                    '#' => Tile::Fast,
                    'O' => Tile::Stone,
                    _ => panic!(),
                })
                .collect(),
        )
    }
    println!("map: {:?}", map);

    // here I do intentionally slow simulation, because I guessed there would be a repeating
    // pattern, because the rocks can't really fall in super interesting ways
    // so I simulate it for like a thousand iterations, and then look for a repeat
    // then calculate how many iterations are left to get the same result as
    // 1000000000 would give me.

    fn up_fall(map: &mut [Vec<Tile>]) {
        // loop until all are down
        loop {
            let mut changed = false;
            for y in 1..map.len() {
                for x in 0..map[0].len() {
                    if map[y][x] == Tile::Stone && map[y - 1][x] == Tile::Air {
                        map[y - 1][x] = Tile::Stone;
                        map[y][x] = Tile::Air;
                        changed = true;
                    }
                }
            }

            if !changed {
                break;
            }
        }
    }

    fn left_fall(map: &mut [Vec<Tile>]) {
        // loop until all are down
        loop {
            let mut changed = false;
            for y in 0..map.len() {
                for x in 1..map[0].len() {
                    if map[y][x] == Tile::Stone && map[y][x - 1] == Tile::Air {
                        map[y][x - 1] = Tile::Stone;
                        map[y][x] = Tile::Air;
                        changed = true;
                    }
                }
            }

            if !changed {
                break;
            }
        }
    }
    fn down_fall(map: &mut [Vec<Tile>]) {
        loop {
            let mut changed = false;
            for y in 0..(map.len() - 1) {
                for x in 0..map[0].len() {
                    if map[y][x] == Tile::Stone && map[y + 1][x] == Tile::Air {
                        map[y + 1][x] = Tile::Stone;
                        map[y][x] = Tile::Air;
                        changed = true;
                    }
                }
            }

            if !changed {
                break;
            }
        }
    }
    fn right_fall(map: &mut [Vec<Tile>]) {
        loop {
            let mut changed = false;
            for y in 0..map.len() {
                for x in 0..(map[0].len() - 1) {
                    if map[y][x] == Tile::Stone && map[y][x + 1] == Tile::Air {
                        map[y][x + 1] = Tile::Stone;
                        map[y][x] = Tile::Air;
                        changed = true;
                    }
                }
            }

            if !changed {
                break;
            }
        }
    }

    fn score_map(map: &[Vec<Tile>]) -> usize {
        let mut score = 0;
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if map[y][x] == Tile::Stone {
                    score += map.len() - y;
                }
            }
        }
        return score;
    }
    let total_iters: usize = 1000000000;
    let start_iterations: usize = 1048; // put a big number here so that it reaches it's repetition state
    for i in 0..start_iterations {
        up_fall(&mut map);
        left_fall(&mut map);
        down_fall(&mut map);
        right_fall(&mut map);
    }
    let after_start_iters = map.clone();

    // wait for repeat
    let repeat_period = {
        let mut i = 0;
        loop {
            up_fall(&mut map);
            left_fall(&mut map);
            down_fall(&mut map);
            right_fall(&mut map);
            i += 1;
            if map == after_start_iters {
                // found repeat
                break i;
            }
        }
    };
    println!("repeat period: {}", repeat_period);

    // how many more iters to get to the 1000000000 map
    let rest_needed = (total_iters - start_iterations) % repeat_period;
    println!("more iters: {}", rest_needed);
    for i in 0..rest_needed {
        up_fall(&mut map);
        left_fall(&mut map);
        down_fall(&mut map);
        right_fall(&mut map);
    }

    // println!("map: {:?}", map);
    let total = score_map(&map);
    println!("total: {}", total);
}
