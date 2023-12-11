use regex::Regex;

pub fn day11_1() {
    let s = std::fs::read_to_string("src/day11.txt").unwrap().into_bytes();

    let mut grid: Vec<&[u8]> = s.split(|&x| x == b'\n').collect();
    grid.remove(grid.len()-1);

    println!("grid: {:?}", grid);

    // y and x positions for all galaxies
    let mut galax: Vec<(isize, isize)> = Vec::new();

    let mut y_pos = 0;
    for &line in grid.iter() {
        // all items this row are empty
        if line.iter().all(|x| *x == b'.') {
            y_pos += 1; // double this rows length
        }
        else {
            // else, look at the row
            let mut x_pos: isize = 0;
            for (idx, c) in line.into_iter().enumerate() {
                // if vertical line here is empty, increase x
                if grid.iter().all(|x| x[idx as usize] == b'.' ) {
                    x_pos += 1; // double 
                }
                else if *c == b'#' {
                    galax.push((y_pos, x_pos));
                }
                x_pos += 1;
            }
        }
        y_pos += 1;
    }
    println!("galaxies: {:?}", galax);

    // your normal all pairs loop
    let mut total = 0;
    for i in 0..(galax.len() - 1) {
        for j in (i + 1)..galax.len() {
            total += (galax[i].0 - galax[j].0).abs() + (galax[i].1 - galax[j].1).abs();
        }
    }
    println!("total: {:?}", total);
}

pub fn day11_2() {
    let s = std::fs::read_to_string("src/day11.txt").unwrap().into_bytes();

    let mut grid: Vec<&[u8]> = s.split(|&x| x == b'\n').collect();
    grid.remove(grid.len()-1);

    println!("grid: {:?}", grid);

    let mut galax: Vec<(isize, isize)> = Vec::new();

    let mut y_pos = 0;
    for &line in grid.iter() {
        if line.iter().all(|x| *x == b'.') {
            // all in this row are empty
            y_pos += 999999; // one million - 1
        }
        else {
            let mut x_pos: isize = 0;
            for (idx, c) in line.into_iter().enumerate() {
                // if vertical line is empty, inc x
                if grid.iter().all(|x| x[idx as usize] == b'.' ) {
                    x_pos += 999999;
                }
                else if *c == b'#' {
                    galax.push((y_pos, x_pos));
                }
                x_pos += 1;
            }
        }
        y_pos += 1;
    }
    println!("galaxies: {:?}", galax);

    let mut total = 0;
    for i in 0..(galax.len() - 1) {
        for j in (i + 1)..galax.len() {
            total += (galax[i].0 - galax[j].0).abs() + (galax[i].1 - galax[j].1).abs();
        }
    }
    println!("total: {:?}", total);
}
