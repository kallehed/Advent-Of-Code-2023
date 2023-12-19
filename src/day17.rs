use std::collections::{BinaryHeap, HashMap, HashSet};

use regex::Regex;

pub fn day17_1() {
    let s = std::fs::read_to_string("src/day17.txt").unwrap();
    let mut map = vec![];
    for line in s.lines() {
        map.push(vec![]);
        for ch in line.chars() {
            map.last_mut().unwrap().push(ch as u8 - b'0');
        }
    }
    // println!("map: {:?}", map);

    #[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
    enum Dir {
        Right,
        Down,
        Left,
        Up,
    }

    #[derive(Eq, PartialEq, Debug)]
    struct Crucible {
        y: i16,
        x: i16,
        walked_straight: i16,
        dir: Dir,
        heat_loss: i16,
    }

    impl PartialOrd for Crucible {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            other.heat_loss.partial_cmp(&self.heat_loss)
        }
    }
    impl Ord for Crucible {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.heat_loss.cmp(&self.heat_loss)
        }
    }

    // (y, x, steps_walked, dir), (heat_loss)
    let mut djikstra: HashMap<(i16, i16, i16, Dir), i16> = HashMap::new();

    let mut heap: BinaryHeap<Crucible> = BinaryHeap::new();

    heap.push(Crucible {
        y: 0,
        x: 0,
        walked_straight: 0,
        dir: Dir::Right,
        heat_loss: -(map[0][0] as i16),
    });
    heap.push(Crucible {
        y: 0,
        x: 0,
        walked_straight: 0,
        dir: Dir::Down,
        heat_loss: -(map[0][0] as i16),
    });

    while heap.len() > 0 {
        let e = heap.pop().unwrap();
        // println!("pos: {}, {}", e.y, e.x);
        // println!("heap: {:?}", heap);
        // in bounds
        if !(e.y >= 0
            && e.x >= 0
            && (e.y as isize) < map.len() as isize
            && (e.x as isize) < map[0].len() as isize)
        {
            continue;
        }
        let new_heat_loss = e.heat_loss + map[e.y as usize][e.x as usize] as i16;
        //if new_heat_loss >= 1418 {continue;}

        if let Some(&prev_heat_loss) = djikstra.get(&(e.y, e.x, e.walked_straight, e.dir)) {
            if new_heat_loss >= prev_heat_loss {
                continue;
            }
        };
        djikstra.insert((e.y, e.x, e.walked_straight, e.dir), new_heat_loss);

        // try to escape when at the end
        // if stuff.0 as usize == map.len() - 1 && stuff.1 as usize == map[0].len() - 1 {
        //     return;
        // }
        let y_dir = if e.dir == Dir::Down {
            1
        } else if e.dir == Dir::Up {
            -1
        } else {
            0
        };
        let x_dir = if e.dir == Dir::Right {
            1
        } else if e.dir == Dir::Left {
            -1
        } else {
            0
        };

        if e.walked_straight < 3 {
            // can go forward
            heap.push(Crucible {
                y: e.y + y_dir,
                x: e.x + x_dir,
                walked_straight: e.walked_straight + 1,
                dir: e.dir,
                heat_loss: new_heat_loss,
            });
        }
        // do left and right

        if true {
            //left
            {
                let new_dir = (-x_dir, y_dir);
                let ac_dir = if new_dir.0 == 1 {
                    Dir::Down
                } else if new_dir.0 == -1 {
                    Dir::Up
                } else if new_dir.1 == 1 {
                    Dir::Right
                } else {
                    Dir::Left
                };
                heap.push(Crucible {
                    y: e.y + new_dir.0,
                    x: e.x + new_dir.1,
                    walked_straight: 1,
                    dir: ac_dir,
                    heat_loss: new_heat_loss,
                });
            }

            //right
            {
                let new_dir = (x_dir, -y_dir);
                let ac_dir = if new_dir.0 == 1 {
                    Dir::Down
                } else if new_dir.0 == -1 {
                    Dir::Up
                } else if new_dir.1 == 1 {
                    Dir::Right
                } else {
                    Dir::Left
                };
                heap.push(Crucible {
                    y: e.y + new_dir.0,
                    x: e.x + new_dir.1,
                    walked_straight: 1,
                    dir: ac_dir,
                    heat_loss: new_heat_loss,
                });
            }
        }
    }

    // println!("djikstra: {:?}", djikstra);

    let mut min = 10000;
    for res in djikstra.iter() {
        if res.0 .0 as usize == map.len() - 1 && res.0 .1 as usize == map[0].len() - 1 {
            println!("res: {:?}, steps: {}", res.1, res.0 .2);
            if *res.1 < min {
                min = *res.1;
            }
        }
    }
    println!("min: {min}");
}

pub fn day17_2() {
    let s = std::fs::read_to_string("src/day17.txt").unwrap();
    let mut map = vec![];
    for line in s.lines() {
        map.push(vec![]);
        for ch in line.chars() {
            map.last_mut().unwrap().push(ch as u8 - b'0');
        }
    }
    // println!("map: {:?}", map);

    #[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
    enum Dir {
        Right,
        Down,
        Left,
        Up,
    }

    #[derive(Eq, PartialEq, Debug)]
    struct Crucible {
        y: i16,
        x: i16,
        walked_straight: i16,
        dir: Dir,
        heat_loss: i16,
    }

    impl PartialOrd for Crucible {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            other.heat_loss.partial_cmp(&self.heat_loss)
        }
    }
    impl Ord for Crucible {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.heat_loss.cmp(&self.heat_loss)
        }
    }

    // (y, x, steps_walked, dir), (heat_loss)
    let mut djikstra: HashMap<(i16, i16, i16, Dir), i16> = HashMap::new();

    let mut heap: BinaryHeap<Crucible> = BinaryHeap::new();

    heap.push(Crucible {
        y: 0,
        x: 0,
        walked_straight: 0,
        dir: Dir::Right,
        heat_loss: -(map[0][0] as i16),
    });
    heap.push(Crucible {
        y: 0,
        x: 0,
        walked_straight: 0,
        dir: Dir::Down,
        heat_loss: -(map[0][0] as i16),
    });

    while heap.len() > 0 {
        let e = heap.pop().unwrap();
        // println!("pos: {}, {}", e.y, e.x);
        // println!("heap: {:?}", heap);
        // in bounds
        if !(e.y >= 0
            && e.x >= 0
            && (e.y as isize) < map.len() as isize
            && (e.x as isize) < map[0].len() as isize)
        {
            continue;
        }
        let new_heat_loss = e.heat_loss + map[e.y as usize][e.x as usize] as i16;
        //if new_heat_loss >= 1418 {continue;}

        if let Some(&prev_heat_loss) = djikstra.get(&(e.y, e.x, e.walked_straight, e.dir)) {
            if new_heat_loss >= prev_heat_loss {
                continue;
            }
        };
        djikstra.insert((e.y, e.x, e.walked_straight, e.dir), new_heat_loss);

        // try to escape when at the end
        // if stuff.0 as usize == map.len() - 1 && stuff.1 as usize == map[0].len() - 1 {
        //     return;
        // }
        let y_dir = if e.dir == Dir::Down {
            1
        } else if e.dir == Dir::Up {
            -1
        } else {
            0
        };
        let x_dir = if e.dir == Dir::Right {
            1
        } else if e.dir == Dir::Left {
            -1
        } else {
            0
        };

        if e.walked_straight < 10 {
            // can go forward
            heap.push(Crucible {
                y: e.y + y_dir,
                x: e.x + x_dir,
                walked_straight: e.walked_straight + 1,
                dir: e.dir,
                heat_loss: new_heat_loss,
            });
        }
        // do left and right

        if e.walked_straight >= 4 {
            //left
            {
                let new_dir = (-x_dir, y_dir);
                let ac_dir = if new_dir.0 == 1 {
                    Dir::Down
                } else if new_dir.0 == -1 {
                    Dir::Up
                } else if new_dir.1 == 1 {
                    Dir::Right
                } else {
                    Dir::Left
                };
                heap.push(Crucible {
                    y: e.y + new_dir.0,
                    x: e.x + new_dir.1,
                    walked_straight: 1,
                    dir: ac_dir,
                    heat_loss: new_heat_loss,
                });
            }

            //right
            {
                let new_dir = (x_dir, -y_dir);
                let ac_dir = if new_dir.0 == 1 {
                    Dir::Down
                } else if new_dir.0 == -1 {
                    Dir::Up
                } else if new_dir.1 == 1 {
                    Dir::Right
                } else {
                    Dir::Left
                };
                heap.push(Crucible {
                    y: e.y + new_dir.0,
                    x: e.x + new_dir.1,
                    walked_straight: 1,
                    dir: ac_dir,
                    heat_loss: new_heat_loss,
                });
            }
        }
    }

    // println!("djikstra: {:?}", djikstra);

    let mut min = 10000;
    for res in djikstra.iter() {
        if res.0 .0 as usize == map.len() - 1
            && res.0 .1 as usize == map[0].len() - 1
            && res.0 .2 >= 4
        {
            println!("res: {:?}, steps: {}", res.1, res.0 .2);
            if *res.1 < min {
                min = *res.1;
            }
        }
    }
    println!("min: {min}");
}
