use std::collections::HashSet;

use regex::Regex;

pub fn day10_1() {
    let s = std::fs::read_to_string("src/day10.txt").unwrap();

    #[derive(Copy, Clone, Debug)]
    enum Place {
        Dot,
        Vertical,
        Horizontal,
        NorthEast,
        NorthWest,
        SouthWest,
        SouthEast,
        StartPos,
    }
    let mut grid = Vec::new();
    let mut start: (isize, isize) = (-1,-1);
    for (lineidx, line) in s.as_bytes().split(|&x| x == b'\n').enumerate() {
        grid.push(Vec::new());
        for (idx, c) in line.iter().enumerate() {
            grid.last_mut().unwrap().push(
            match c {
                b'|' => Place::Vertical,
                b'-' => Place::Horizontal,
                b'L' => Place::NorthEast,
                b'J' => Place::NorthWest,
                b'7' => Place::SouthWest,
                b'F' => Place::SouthEast,
                b'S' => {
                    start = (lineidx as _, idx as _);
                    Place::StartPos
                },
                b'.' => Place::Dot,
                _ => panic!(),
            }
            );
        } 
    }
    println!("startpos: {:?}", start);

    use Place::*;
    let possible = [Vertical, Horizontal, NorthEast, NorthWest, SouthWest, SouthEast];

    'leloop: for try_this_one in possible.iter() {
        grid[start.0 as usize][start.1 as usize] = *try_this_one;

        // simulate flow through pipes
        let highest_tries = grid.len() * grid[0].len();
        let mut place = start;
        let mut prev_move = (0,0);
        for tries in 0.. {
            // println!("at: {:?}", place);
            // println!("prevmove: {:?}", prev_move);
            if tries > highest_tries || place.0 < 0 || place.1 < 0 || place.0 >= grid.len() as _ || place.1 >= grid[0].len() as _{
                continue 'leloop;
            }
            prev_move = match grid[place.0 as usize][place.1 as usize] {
                Dot => continue 'leloop, // bad to land here
                Vertical => {
                    if prev_move.0 > 0 {
                        (1,0)
                    }
                    else {
                        (-1,0)
                    }
                },
                Horizontal => {
                    if prev_move.1 > 0 {
                        (0,1)
                    }
                    else {(0,-1)}
                },
                NorthEast => {
                    if prev_move.0 > 0 {
                        (0,1)
                    }
                    else {
                        (-1,0)
                    }
                },
                NorthWest => {
                    if prev_move.0 > 0 {
                        (0, -1)
                    }
                    else {
                        (-1, 0)
                    }
                } 
                SouthWest => {
                    if prev_move.0 < 0 {
                        (0, -1)
                    }
                    else {
                        (1, 0)
                    }
                } 
                SouthEast => {
                    if prev_move.0 < 0 {
                        (0,1)
                    }
                    else {
                        (1,0)
                    }
                }
                StartPos => panic!(), 
            };
            place.0 += prev_move.0;
            place.1 += prev_move.1;
            if place == start {
                // got back, we have loop
                println!("got loop: len {}", (tries + 1)/2);
                continue 'leloop;
            }
        }
    }
    println!("TAKE THE HIGHEST NUMBER OF THE LOOPS ABOVE!");
}

pub fn day10_2() {
    let s = std::fs::read_to_string("src/day10.txt").unwrap();

    #[derive(Copy, Clone, Debug)]
    enum Place {
        Dot,
        Vertical,
        Horizontal,
        NorthEast,
        NorthWest,
        SouthWest,
        SouthEast,
        StartPos,
    }
    let mut grid = Vec::new();
    let mut start: (isize, isize) = (-1,-1);
    for (lineidx, line) in s.as_bytes().split(|&x| x == b'\n').enumerate() {
        grid.push(Vec::new());
        for (idx, c) in line.iter().enumerate() {
            grid.last_mut().unwrap().push(
            match c {
                b'|' => Place::Vertical,
                b'-' => Place::Horizontal,
                b'L' => Place::NorthEast,
                b'J' => Place::NorthWest,
                b'7' => Place::SouthWest,
                b'F' => Place::SouthEast,
                b'S' => {
                    start = (lineidx as _, idx as _);
                    Place::StartPos
                },
                b'.' => Place::Dot,
                _ => panic!(),
            }
            );
        } 
    }
    println!("startpos: {:?}", start);

    use Place::*;
    let possible = [Vertical, Horizontal, NorthEast, NorthWest, SouthWest, SouthEast];

    let mut best_one_len = 0;
    let mut best_start_tile = Place::Dot;

    'leloop: for try_this_one in possible.iter() {
        grid[start.0 as usize][start.1 as usize] = *try_this_one;

        // simulate flow through pipes
        let highest_tries = grid.len() * grid[0].len();
        let mut place = start;
        let mut prev_move = (0,0);
        for tries in 0.. {
            if tries > highest_tries || place.0 < 0 || place.1 < 0 || place.0 >= grid.len() as _ || place.1 >= grid[0].len() as _{
                continue 'leloop;
            }
            prev_move = match grid[place.0 as usize][place.1 as usize] {
                Dot => continue 'leloop, // bad to land here
                Vertical => {
                    if prev_move.0 > 0 {
                        (1,0)
                    }
                    else {
                        (-1,0)
                    }
                },
                Horizontal => {
                    if prev_move.1 > 0 {
                        (0,1)
                    }
                    else {(0,-1)}
                },
                NorthEast => {
                    if prev_move.0 > 0 {
                        (0,1)
                    }
                    else {
                        (-1,0)
                    }
                },
                NorthWest => {
                    if prev_move.0 > 0 {
                        (0, -1)
                    }
                    else {
                        (-1, 0)
                    }
                } 
                SouthWest => {
                    if prev_move.0 < 0 {
                        (0, -1)
                    }
                    else {
                        (1, 0)
                    }
                } 
                SouthEast => {
                    if prev_move.0 < 0 {
                        (0,1)
                    }
                    else {
                        (1,0)
                    }
                }
                StartPos => panic!(), 
            };
            place.0 += prev_move.0;
            place.1 += prev_move.1;
            if place == start {
                // got back, we have loop
                let we_got = (tries + 1)/2;
                println!("got loop: len {}", we_got);
                if we_got > best_one_len {
                    best_one_len = we_got;
                    best_start_tile = *try_this_one;

                }
                continue 'leloop;
            }
        }
    }
    println!("best took: {}, tile: {:?}", best_one_len, best_start_tile);

    grid[start.0 as usize][start.1 as usize] = best_start_tile;

    // for all the possible start normals. It's fast so it's fine
    let normals = [(1,0), (-1,0), (0,1), (0,-1)];
    for start_normal in normals {
        // simulate flow through pipes
        let mut place = start;
        let mut prev_move = (0,0);
        let mut places = std::collections::HashSet::new();
        let mut inside_places = std::collections::HashSet::new();
        let mut normal = start_normal;
        for tries in 0.. {
            prev_move = match grid[place.0 as usize][place.1 as usize] {
                Dot => panic!(), // bad to land here
                Vertical => {
                    if prev_move.0 > 0 {
                        (1,0)
                    }
                    else {
                        (-1,0)
                    }
                },
                Horizontal => {
                    if prev_move.1 > 0 {
                        (0,1)
                    }
                    else {(0,-1)}
                },
                NorthEast => {
                    normal = (-normal.1, -normal.0);
                    if prev_move.0 > 0 {
                        (0,1)
                    }
                    else {
                        (-1,0)
                    }
                },
                NorthWest => {
                    normal = (normal.1, normal.0);
                    if prev_move.0 > 0 {
                        (0, -1)
                    }
                    else {
                        (-1, 0)
                    }
                } 
                SouthWest => {
                    normal = (-normal.1, -normal.0);
                    if prev_move.0 < 0 {
                        (0, -1)
                    }
                    else {
                        (1, 0)
                    }
                } 
                SouthEast => {
                    normal = (normal.1, normal.0);
                    if prev_move.0 < 0 {
                        (0,1)
                    }
                    else {
                        (1,0)
                    }
                }
                StartPos => panic!(), 
            };
            // don't forget to include the previous position offset by the new normal!
            inside_places.insert((place.0 + normal.0, place.1 + normal.1)); 
            place.0 += prev_move.0;
            place.1 += prev_move.1;
            places.insert(place);
            inside_places.insert((place.0 + normal.0, place.1 + normal.1));
            if place == start {
                // got back, we have loop
                println!("how many dif: {:?}", inside_places.difference(&places).count());
                let mut wow: HashSet<_> = inside_places.difference(&places).map(|x| *x).collect();
                // do rounds trying to increase the set outwards, stop if we don't increase anymore
                // also stop if we reach the borders, that means we failed
                loop {
                    let mut new_p = HashSet::new();
                    for &w in wow.iter() {
                        for y in -1..=1 {
                            for x in -1..=1 {
                                let v = (w.0 + y, w.1 + x);
                                new_p.insert(v);
                            }
                        }
                    }
                    let better_new_p: HashSet<_> = new_p.difference(&places).map(|x| *x).collect();
                    if better_new_p.iter().any(|&x| x.0 < 0 || x.1 < 0 || x.0 >= grid.len() as isize || x.1 >= grid[0].len() as isize) {
                        // bad one
                        break;
                    }
                    if better_new_p.len() == wow.len() {
                        println!("RESULT: {}!", wow.len());
                        break;
                    }
                    wow = better_new_p;
                } 
                break;
            }
        }
    }
}
