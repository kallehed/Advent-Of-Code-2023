use std::collections::{HashSet, HashMap};

use regex::Regex;

pub fn day22_1() {
    let s = std::fs::read_to_string("src/day22.txt").unwrap();

    // make them fall down, use 3d matrix, fall each one in good order, from smallest z to highest
    // keep a track of their positions. Then for each one look at which ones it has below it, 
    // if it has only one, we can not disintegrate that one, so bad.


    // crashes rust because stack is too small :(
    // let mut world = [[[(false, -1); 400]; 400]; 400];

    let mut world: Vec<Vec<Vec<(bool, i32)>>> = Vec::new();
    for x in 0..400 {
        world.push(vec![]);
        for y in 0..400 {
            world.last_mut().unwrap().push(vec![]);
            for z in 0..400 {
                world.last_mut().unwrap().last_mut().unwrap().push((false, -1));
            }
        }
    }

    let reg = Regex::new(r"(\d+),(\d+),(\d+)\~(\d+),(\d+),(\d+)").unwrap();

    let mut start_positions = Vec::new();
    for line in s.lines() {
        let mut x0 = -69;
        let mut y0 = -69;
        let mut z0 = -69;
        let mut x1 = -69;
        let mut y1 = -69;
        let mut z1 = -69;
        for cap in reg.captures_iter(line) {
           x0 = cap[1].parse::<i32>().unwrap(); 
           y0 = cap[2].parse::<i32>().unwrap(); 
           z0 = cap[3].parse::<i32>().unwrap();
           x1 = cap[4].parse::<i32>().unwrap(); 
           y1 = cap[5].parse::<i32>().unwrap(); 
           z1 = cap[6].parse::<i32>().unwrap();
           break;
        }
        start_positions.push(((x0, y0, z0),(x1,y1,z1)));
    }
    // println!("start pos: {:?}", start_positions);
    start_positions.sort_by(|&x, &y| {
        x.0.2.min(x.1.2).cmp(&y.0.2.min(y.1.2))
    });
    println!("sorted {:?}", start_positions);

    let mut planks = Vec::new();
    {
        for (idx, mut pos) in start_positions.into_iter().enumerate() {
            planks.push(Vec::new());

            'placement_loop: while pos.0.2.min(pos.1.2) > 1 {
                // look if collision
                // if collide, go back one step and place
                // otherwise continue
                let max_dif = (pos.1.0 - pos.0.0).max(pos.1.1 - pos.0.1).max(pos.1.2 - pos.0.2) + 1;
                let x_change = if pos.1.0 - pos.0.0 == 0 {0} else {1};
                let y_change = if pos.1.1 - pos.0.1 == 0 {0} else {1};
                let z_change = if pos.1.2 - pos.0.2 == 0 {0} else {1};
                let mut temp_pos = pos.0;
                for _ in 0..max_dif {
                    if world[temp_pos.0 as usize][temp_pos.1 as usize][(temp_pos.2 - 1) as usize].0 {
                       // failed here 
                        break 'placement_loop;
                    }

                    temp_pos.0 += x_change;
                    temp_pos.1 += y_change;
                    temp_pos.2 += z_change;
                }
                pos.0.2 -= 1;
                pos.1.2 -= 1;
            }

            let max_dif = (pos.1.0 - pos.0.0).max(pos.1.1 - pos.0.1).max(pos.1.2 - pos.0.2) + 1;
            let x_change = if pos.1.0 - pos.0.0 == 0 {0} else {1};
            let y_change = if pos.1.1 - pos.0.1 == 0 {0} else {1};
            let z_change = if pos.1.2 - pos.0.2 == 0 {0} else {1};
            let mut temp_pos = pos.0;
            for _ in 0..max_dif {
                world[temp_pos.0 as usize][temp_pos.1 as usize][temp_pos.2 as usize].0 = true;
                world[temp_pos.0 as usize][temp_pos.1 as usize][temp_pos.2 as usize].1 = idx as i32;
                planks.last_mut().unwrap().push((temp_pos.0, temp_pos.1, temp_pos.2));
                temp_pos.0 += x_change;
                temp_pos.1 += y_change;
                temp_pos.2 += z_change;
            }
        }
    }
    // for each plank, look below it, if it depends on a single plank, add that plank idx to a
    // hashset

    // 681 wrong answer
    // 793 wrong answer
    // 767 too high
    // 535 wrong answer
    // 473

    let mut imp_plank_idxs = HashSet::new();
    'plank_loop: for (idx, plank) in planks.iter().enumerate() {
        let mut single_plank_dependency = -1;
        for pos in plank {
            // look below in world to see if there are any planks
            let res = world[pos.0 as usize][pos.1 as usize][(pos.2 - 1) as usize];
            if res.0 {
                if res.1 == idx as _ || res.1 == single_plank_dependency {
                    continue; // it's us, we found ourselves
                }
                // println!("found plank {} below", res.1);
                if single_plank_dependency == -1 {
                    single_plank_dependency = res.1;
                }
                else {
                    // depends on two planks, everything is fine
                    continue 'plank_loop;
                }
            }
        }
        if single_plank_dependency != -1 { // not ones on the ground
            imp_plank_idxs.insert(single_plank_dependency);
        }
    }
    println!("imp: {}, can be disintegrated: {}", imp_plank_idxs.len(), planks.len() - imp_plank_idxs.len());
}

pub fn day22_2() {
    let s = std::fs::read_to_string("src/day22.txt").unwrap();

    let mut world: Vec<Vec<Vec<(bool, i32)>>> = Vec::new();
    for x in 0..400 {
        world.push(vec![]);
        for y in 0..400 {
            world.last_mut().unwrap().push(vec![]);
            for z in 0..400 {
                world.last_mut().unwrap().last_mut().unwrap().push((false, -1));
            }
        }
    }

    let reg = Regex::new(r"(\d+),(\d+),(\d+)\~(\d+),(\d+),(\d+)").unwrap();

    let mut start_positions = Vec::new();
    for line in s.lines() {
        let mut x0 = -69;
        let mut y0 = -69;
        let mut z0 = -69;
        let mut x1 = -69;
        let mut y1 = -69;
        let mut z1 = -69;
        for cap in reg.captures_iter(line) {
           x0 = cap[1].parse::<i32>().unwrap(); 
           y0 = cap[2].parse::<i32>().unwrap(); 
           z0 = cap[3].parse::<i32>().unwrap();
           x1 = cap[4].parse::<i32>().unwrap(); 
           y1 = cap[5].parse::<i32>().unwrap(); 
           z1 = cap[6].parse::<i32>().unwrap();
           break;
        }
        start_positions.push(((x0, y0, z0),(x1,y1,z1)));
    }
    // println!("start pos: {:?}", start_positions);
    start_positions.sort_by(|&x, &y| {
        x.0.2.min(x.1.2).cmp(&y.0.2.min(y.1.2))
    });
    println!("sorted {:?}", start_positions);

    let mut planks = Vec::new();
    {
        for (idx, mut pos) in start_positions.into_iter().enumerate() {
            planks.push(Vec::new());

            'placement_loop: while pos.0.2.min(pos.1.2) > 1 {
                // look if collision
                // if collide, go back one step and place
                // otherwise continue
                let max_dif = (pos.1.0 - pos.0.0).max(pos.1.1 - pos.0.1).max(pos.1.2 - pos.0.2) + 1;
                let x_change = if pos.1.0 - pos.0.0 == 0 {0} else {1};
                let y_change = if pos.1.1 - pos.0.1 == 0 {0} else {1};
                let z_change = if pos.1.2 - pos.0.2 == 0 {0} else {1};
                let mut temp_pos = pos.0;
                for _ in 0..max_dif {
                    if world[temp_pos.0 as usize][temp_pos.1 as usize][(temp_pos.2 - 1) as usize].0 {
                       // failed here 
                        break 'placement_loop;
                    }

                    temp_pos.0 += x_change;
                    temp_pos.1 += y_change;
                    temp_pos.2 += z_change;
                }
                pos.0.2 -= 1;
                pos.1.2 -= 1;
            }

            let max_dif = (pos.1.0 - pos.0.0).max(pos.1.1 - pos.0.1).max(pos.1.2 - pos.0.2) + 1;
            let x_change = if pos.1.0 - pos.0.0 == 0 {0} else {1};
            let y_change = if pos.1.1 - pos.0.1 == 0 {0} else {1};
            let z_change = if pos.1.2 - pos.0.2 == 0 {0} else {1};
            let mut temp_pos = pos.0;
            for _ in 0..max_dif {
                world[temp_pos.0 as usize][temp_pos.1 as usize][temp_pos.2 as usize].0 = true;
                world[temp_pos.0 as usize][temp_pos.1 as usize][temp_pos.2 as usize].1 = idx as i32;
                planks.last_mut().unwrap().push((temp_pos.0, temp_pos.1, temp_pos.2));
                temp_pos.0 += x_change;
                temp_pos.1 += y_change;
                temp_pos.2 += z_change;
            }
        }
    }

    // find dependencies 

    let mut plank_depends_on = Vec::new();
    for (idx, plank) in planks.iter().enumerate() {
        let mut depends_on = HashSet::new();
        for pos in plank {
            // look below in world to see if there are any planks
            let res = world[pos.0 as usize][pos.1 as usize][(pos.2 - 1) as usize];
            if res.0 {
                if res.1 == idx as _ {
                    continue; // it's us, we found ourselves
                }
                // println!("found plank {} below", res.1);
                depends_on.insert(res.1);
            }
        }
        plank_depends_on.push(depends_on);
    }
    println!("{:?}", plank_depends_on);

    let mut total = 0;
    for plank_idx in 0..planks.len() {
        // what if we destroy this one, do a loop that adds things that will fall
        let mut fell_off = HashSet::new();
        fell_off.insert(plank_idx as i32);
        let mut changed_fell_off = true;
        while changed_fell_off {
            changed_fell_off = false;
            for (other_plank_idx, what_it_depends_on) in plank_depends_on.iter().enumerate() {
                if !fell_off.contains(&(other_plank_idx as i32)) {
                    // got a fresh plank here, wonder if it depends on things that have fell off
                    if what_it_depends_on.len() != 0 && what_it_depends_on.is_subset(&fell_off) {
                        changed_fell_off = true;
                        fell_off.insert(other_plank_idx as i32);
                    }
                }
            }
        }
        // nobody is falling anymore, count how many fell
        println!("len: {}", fell_off.len());
        total += fell_off.len() - 1;
    }
    println!("total: {total}");

}
