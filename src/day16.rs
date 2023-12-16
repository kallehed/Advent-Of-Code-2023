use std::collections::HashSet;

use regex::Regex;



// HAS TO BE COMPILED IN RELEASE MODE, OTHERWISE THE INTEGER UNDERFLOWS (WHICH ARE DELIBERATE) WILL
// CRASH

pub fn day16_1() {
    println!("part 1");
    let s = std::fs::read_to_string("src/day16.txt").unwrap();

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    enum Tile {
        Air,
        Vertical,
        Horizontal,
        FwSlash,
        BkSlash,
    }
    let mut map = vec![];
    for line in s.lines() {
        map.push(vec![]);
        for ch in line.chars() {
            let t = match ch {
                '.' => Tile::Air,
                '|' => Tile::Vertical,
                '-' => Tile::Horizontal,
                '/' => Tile::FwSlash,
                '\\' => Tile::BkSlash,
                _ => panic!()
            };
            map.last_mut().unwrap().push(t);
        }
    }
    println!("map: {:?}", map);

    let mut all_beams = HashSet::<(usize, usize, i8, i8)>::new();

    let mut beams = HashSet::<(usize, usize, i8, i8)>::new();
    beams.insert((0,0, 0, 1));

    let mut beams_2 = HashSet::<(usize, usize, i8, i8)>::new();

    let mut tiles_energized = HashSet::<(usize, usize)>::new();

    let mut prev_all_beams_size = 1234;

    while all_beams.len() != prev_all_beams_size {
        prev_all_beams_size = all_beams.len();
        beams_2.clear();
        for beam in beams.iter() {
            if beam.0 < map.len() && beam.1 < map[0].len() {

            }
            else {
                continue;
            }
            let tile = map[beam.0][beam.1];
            tiles_energized.insert((beam.0, beam.1));
            all_beams.insert(*beam);

            match tile {
                Tile::Air => {
                    // continue
                    beams_2.insert(((beam.0 as isize  + beam.2 as isize ) as _, (beam.1 as isize + beam.3 as isize) as _ , beam.2, beam.3));
                },
                Tile::Vertical => {
                    if beam.2 != 0 {
                        // travelling through it, continue
                        beams_2.insert(((beam.0 as isize + beam.2 as isize ) as _, (beam.1 as isize + beam.3 as isize) as _ , beam.2, beam.3));
                    }
                    else {
                        // go down and up from here
                        beams_2.insert((beam.0 + 1, beam.1, 1, 0));
                        beams_2.insert((beam.0 - 1, beam.1, -1, 0));
                    }
                },
                Tile::Horizontal => {
                    if beam.3 != 0 {
                        // travelling through it, continue
                        beams_2.insert(((beam.0 as isize + beam.2 as isize ) as _, (beam.1 as isize + beam.3 as isize) as _ , beam.2, beam.3));
                    }
                    else {
                        // go down and up from here
                        beams_2.insert((beam.0, beam.1 + 1, 0, 1));
                        beams_2.insert((beam.0, beam.1 - 1, 0, -1));
                    }
                },
                Tile::FwSlash => {
                    let dir = (-beam.3, -beam.2);
                    beams_2.insert(((beam.0 as isize + dir.0 as isize) as _, (beam.1 as isize + dir.1 as isize) as _, dir.0, dir.1));
                },
                Tile::BkSlash => {
                    let dir = (beam.3, beam.2);
                    beams_2.insert(((beam.0 as isize + dir.0 as isize) as _, (beam.1 as isize + dir.1 as isize) as _, dir.0, dir.1));
                },
            }
        }
        beams.clone_from(&beams_2);

    }
    println!("total: {}", tiles_energized.len());
}

// takes 20 seconds to run on my computer, thank you rust!
pub fn day16_2() {
    println!("part 2");
    let s = std::fs::read_to_string("src/day16.txt").unwrap();

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    enum Tile {
        Air,
        Vertical,
        Horizontal,
        FwSlash,
        BkSlash,
    }
    let mut map = vec![];
    for line in s.lines() {
        map.push(vec![]);
        for ch in line.chars() {
            let t = match ch {
                '.' => Tile::Air,
                '|' => Tile::Vertical,
                '-' => Tile::Horizontal,
                '/' => Tile::FwSlash,
                '\\' => Tile::BkSlash,
                _ => panic!()
            };
            map.last_mut().unwrap().push(t);
        }
    }
    println!("map: {:?}", map);

    fn energy(map: &Vec<Vec<Tile>>, start: (usize, usize, i8, i8)) -> usize {
        let mut all_beams = HashSet::<(usize, usize, i8, i8)>::new();

        let mut beams = HashSet::<(usize, usize, i8, i8)>::new();
        beams.insert(start);

        let mut beams_2 = HashSet::<(usize, usize, i8, i8)>::new();

        let mut tiles_energized = HashSet::<(usize, usize)>::new();

        let mut prev_all_beams_size = 1234;

        while all_beams.len() != prev_all_beams_size {
            prev_all_beams_size = all_beams.len();
            beams_2.clear();
            for beam in beams.iter() {
                if beam.0 < map.len() && beam.1 < map[0].len() {

                }
                else {
                    continue;
                }
                let tile = map[beam.0][beam.1];
                tiles_energized.insert((beam.0, beam.1));
                all_beams.insert(*beam);

                match tile {
                    Tile::Air => {
                        // continue
                        beams_2.insert(((beam.0 as isize  + beam.2 as isize ) as _, (beam.1 as isize + beam.3 as isize) as _ , beam.2, beam.3));
                    },
                    Tile::Vertical => {
                        if beam.2 != 0 {
                            // travelling through it, continue
                            beams_2.insert(((beam.0 as isize + beam.2 as isize ) as _, (beam.1 as isize + beam.3 as isize) as _ , beam.2, beam.3));
                        }
                        else {
                            // go down and up from here
                            beams_2.insert((beam.0 + 1, beam.1, 1, 0));
                            beams_2.insert((beam.0 - 1, beam.1, -1, 0));
                        }
                    },
                    Tile::Horizontal => {
                        if beam.3 != 0 {
                            // travelling through it, continue
                            beams_2.insert(((beam.0 as isize + beam.2 as isize ) as _, (beam.1 as isize + beam.3 as isize) as _ , beam.2, beam.3));
                        }
                        else {
                            // go down and up from here
                            beams_2.insert((beam.0, beam.1 + 1, 0, 1));
                            beams_2.insert((beam.0, beam.1 - 1, 0, -1));
                        }
                    },
                    Tile::FwSlash => {
                        let dir = (-beam.3, -beam.2);
                        beams_2.insert(((beam.0 as isize + dir.0 as isize) as _, (beam.1 as isize + dir.1 as isize) as _, dir.0, dir.1));
                    },
                    Tile::BkSlash => {
                        let dir = (beam.3, beam.2);
                        beams_2.insert(((beam.0 as isize + dir.0 as isize) as _, (beam.1 as isize + dir.1 as isize) as _, dir.0, dir.1));
                    },
                }
            }
            beams.clone_from(&beams_2);

        }
        return tiles_energized.len();
    }

    // try out all the directions from all the angles and places and such

    let mut highest = 0;

    for y in 0..map.len() {
        let e = energy(&map, (y, 0, 0, 1));
        if e > highest {
            highest = e;
        }
    }
    for y in 0..map.len() {
        let e = energy(&map, (y, map[0].len() - 1, 0, -1));
        if e > highest {
            highest = e;
        }
    }
    for x in 0..map[0].len() {
        let e = energy(&map, (0, x, 1, 0));
        if e > highest {
            highest = e;
        }
    }
    for x in 0..map[0].len() {
        let e = energy(&map, (map.len() - 1, x, -1, 0));
        if e > highest {
            highest = e;
        }
    }

    println!("highest: {}", highest);

}
