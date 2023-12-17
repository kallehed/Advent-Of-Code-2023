use std::collections::{HashSet, HashMap};

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
    println!("map: {:?}", map);

    #[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
    enum Dir {
        Right,
        Down,
        Left,
        Up,
    }

    // y, x, y_dir, x_dir, steps_walked, heat_loss
    let mut walkers: HashSet<(i16, i16, i8, i8, i16, i32)> = HashSet::new();

    // (y, x, steps_walked), (heat_loss)
    let mut djikstra: HashMap<(i16, i16, i16, Dir), i32> = HashMap::new();

    walkers.insert((0, 0, 1, 0, 0, 0));

    fn dive(stuff: (i16, i16, i16), y_dir: i8, x_dir: i8, heat_loss: i32, djikstra: &mut HashMap<(i16, i16, i16, Dir), i32>, map: &Vec<Vec<u8>>) {
        // in bounds
        if !(stuff.0 >= 0 && stuff.1 >= 0 && (stuff.0 as isize) < map.len() as isize && (stuff.1 as isize)  < map[0].len() as isize) {
            return;
        }
        let new_heat_loss = heat_loss + map[stuff.0 as usize][stuff.1 as usize] as i32;
        if new_heat_loss >= 1261 {return;}

        let dir = if y_dir == 1 {Dir::Down} else if y_dir == -1 {Dir::Up} else if x_dir == 1 {Dir::Right} else {Dir::Left};

        let o_stuff = (stuff.0, stuff.1, stuff.2, dir);

        match djikstra.get(&o_stuff) {
            Some(&prev_heat_loss) => {
                if new_heat_loss >= prev_heat_loss {
                    return;
                } 
            },
            None => {

            },
        }
        djikstra.insert(o_stuff, new_heat_loss);

       if stuff.2 < 3 {
           // can go forward
           let new_stuff = (stuff.0 + y_dir as i16, stuff.1 + x_dir as i16 , stuff.2 + 1);
           dive(new_stuff, y_dir, x_dir, new_heat_loss, djikstra, map);
       } 
       // do left and right
       //left
        
       {
           let new_dir = (-x_dir, y_dir);
           dive((stuff.0 + new_dir.0 as i16, stuff.1 + new_dir.1 as i16, 1), new_dir.0, new_dir.1, new_heat_loss, djikstra, map);
       }

       //right
       {
           let new_dir = (x_dir, -y_dir);
           dive((stuff.0 + new_dir.0 as i16, stuff.1 + new_dir.1 as i16, 1), new_dir.0, new_dir.1, new_heat_loss, djikstra, map);
       }
    }

    dive((0,0,0), 1, 0, -(map[0][0] as i32), &mut djikstra, &map);

    println!("djikstra: {:?}", djikstra);

    for res in djikstra.iter() {

        if res.0.0 as usize == map.len() - 1 && res.0.1 as usize == map[0].len() - 1 {

            println!("res: {:?}", res.1);
        }
    }
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
    println!("map: {:?}", map);

    #[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
    enum Dir {
        Right,
        Down,
        Left,
        Up,
    }

    // (y, x, steps_walked), (heat_loss)
    let mut djikstra: HashMap<(i16, i16, i16, Dir), i32> = HashMap::new();


    fn dive(stuff: (i16, i16, i16), y_dir: i8, x_dir: i8, heat_loss: i32, djikstra: &mut HashMap<(i16, i16, i16, Dir), i32>, map: &Vec<Vec<u8>>) {
        // in bounds
        if !(stuff.0 >= 0 && stuff.1 >= 0 && (stuff.0 as isize) < map.len() as isize && (stuff.1 as isize)  < map[0].len() as isize) {
            return;
        }
        let new_heat_loss = heat_loss + map[stuff.0 as usize][stuff.1 as usize] as i32;
        if new_heat_loss >= 1418 {return;}

        let dir = if y_dir == 1 {Dir::Down} else if y_dir == -1 {Dir::Up} else if x_dir == 1 {Dir::Right} else {Dir::Left};

        let o_stuff = (stuff.0, stuff.1, stuff.2, dir);

        match djikstra.get(&o_stuff) {
            Some(&prev_heat_loss) => {
                if new_heat_loss >= prev_heat_loss {
                    return;
                } 
            },
            None => {

            },
        }
        djikstra.insert(o_stuff, new_heat_loss);

        if stuff.0 as usize == map.len() - 1 && stuff.1 as usize == map[0].len() - 1 {
            return;
        }

       if stuff.2 < 10 {
           // can go forward
           let new_stuff = (stuff.0 + y_dir as i16, stuff.1 + x_dir as i16 , stuff.2 + 1);
           dive(new_stuff, y_dir, x_dir, new_heat_loss, djikstra, map);
       } 
       // do left and right
        
       if stuff.2 >= 4 {
           //left
           {
               let new_dir = (-x_dir, y_dir);
               dive((stuff.0 + new_dir.0 as i16, stuff.1 + new_dir.1 as i16, 1), new_dir.0, new_dir.1, new_heat_loss, djikstra, map);
           }

           //right
           {
               let new_dir = (x_dir, -y_dir);
               dive((stuff.0 + new_dir.0 as i16, stuff.1 + new_dir.1 as i16, 1), new_dir.0, new_dir.1, new_heat_loss, djikstra, map);
           }
       }
    }

    dive((0,0,0), 1, 0, -(map[0][0] as i32), &mut djikstra, &map);
    dive((0,0,0), 0, 1, -(map[0][0] as i32), &mut djikstra, &map);

    println!("djikstra: {:?}", djikstra);

    let mut min = 1000000;
    for res in djikstra.iter() {

        if res.0.0 as usize == map.len() - 1 && res.0.1 as usize == map[0].len() - 1 {

            println!("res: {:?}, steps: {}", res.1, res.0.2);
            if *res.1 < min {
                min = *res.1;
            }
        }
    }
    println!("min: {min}");

}
