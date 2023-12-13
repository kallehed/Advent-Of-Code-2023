use regex::Regex;

pub fn day13_1() {
    let s = std::fs::read_to_string("src/day13.txt").unwrap().into_bytes();
    let s: Vec<_> = s.split(|x| *x == b'\n').collect();

    #[derive(Debug, PartialEq)]
    enum Cell {
        Ash, Rocks,
    }

    fn find_mirrors_horizontally(map: &[Vec<Cell>]) -> usize {
        let width = map[0].len();
        let height = map.len();

        // row idx is the upper row
        'test_pairs: for row_idx in 0..(height - 1) {
            
            let mut upper = row_idx;
            let mut lower = row_idx + 1;
            while lower < height {

                // check equality
                if *map[upper] == *map[lower] {
                    if upper == 0 {
                        break;
                    }
                    upper -= 1;
                    lower += 1;
                }
                else {
                    continue 'test_pairs;
                }
            }
            // those two pairs succeded
            return 100 * (row_idx + 1);
        }
        return 0;
    }

    fn find_mirrors_vertically(map: &[Vec<Cell>]) -> usize {
        let width = map[0].len();
        let height = map.len();

        'test_pairs: for column_idx in 0..(width - 1) {
            
            let mut left = column_idx;
            let mut right = column_idx + 1;
            while right < width {
                // check equality
                if map.iter().all(|row| row[left] == row[right]) {
                    if left == 0 {
                        break;
                    }
                    left -= 1;
                    right += 1;
                }
                else {
                    continue 'test_pairs;
                }
            }
            // these two succeded
            return column_idx + 1;
        }
        return 0;
    }

    println!("split by asd: {:?}", s);
    let mut map: Vec<Vec<Cell>> = Vec::new();

    let mut total = 0;

    for line in s {
        if line.len() == 0 {
            // now calc the mirror stuff
            println!("map: {:?}", map);
            
            // find mirrors
            let count = find_mirrors_horizontally(&map);
            let count2 = find_mirrors_vertically(&map);
            total += count + count2;
            println!("got count: {}", count + count2);
            assert!(!(count != 0 && count2 != 0));
            // this line here separates two places, reset stuff
            map.clear();
        }
        else {
            map.push(vec![]);
            for ch in line {
                map.last_mut().unwrap().push(
                    match *ch {
                        b'#' => Cell::Rocks,
                        b'.' => Cell::Ash,
                        other => panic!("got wrong: {}", other),
                    }
                )
            }
        }
    }
    println!("total: {total}");
}

pub fn day13_2() {
    let s = std::fs::read_to_string("src/day13.txt").unwrap().into_bytes();
    let s: Vec<_> = s.split(|x| *x == b'\n').collect();

    #[derive(Debug, PartialEq)]
    enum Cell {
        Ash, Rocks,
    }

    // now we need to have a ONE of difference between the reflections for it to be valid

    fn find_mirrors_horizontally(map: &[Vec<Cell>]) -> usize {
        let width = map[0].len();
        let height = map.len();

        // row idx is the upper row
        for row_idx in 0..(height - 1) {
            
            let mut upper = row_idx;
            let mut lower = row_idx + 1;
            let mut errors = 0;
            while lower < height && errors <= 1 {

                // check equality
                errors += map[upper].iter().zip(map[lower].iter()).filter(|&x| x.0 != x.1).count();
                if upper == 0 {
                    break;
                }
                upper -= 1;
                lower += 1;
            }
            if errors != 1{continue;}
            // those two pairs succeded
            return 100 * (row_idx + 1);
        }
        return 0;
    }

    fn find_mirrors_vertically(map: &[Vec<Cell>]) -> usize {
        let width = map[0].len();
        let height = map.len();

        for column_idx in 0..(width - 1) {
            
            let mut left = column_idx;
            let mut right = column_idx + 1;
            let mut errors = 0;
            while right < width {
                // check equality
                errors += map.iter().filter(|row| row[left] != row[right]).count();
                if left == 0 {
                    break;
                }
                left -= 1;
                right += 1;
            }
            if errors != 1 {continue;}
            // these two succeded
            return column_idx + 1;
        }
        return 0;
    }

    println!("split by asd: {:?}", s);
    let mut map: Vec<Vec<Cell>> = Vec::new();

    let mut total = 0;

    for line in s {
        if line.len() == 0 {
            // now calc the mirror stuff
            println!("map: {:?}", map);
            
            // find mirrors
            let count = find_mirrors_horizontally(&map);
            let count2 = find_mirrors_vertically(&map);
            total += count + count2;
            println!("got count: {}", count + count2);
            assert!(!(count != 0 && count2 != 0));

            // this line here separates two places, reset stuff
            map.clear();
        }
        else {
            map.push(vec![]);
            for ch in line {
                map.last_mut().unwrap().push(
                    match *ch {
                        b'#' => Cell::Rocks,
                        b'.' => Cell::Ash,
                        other => panic!("got wrong: {}", other),
                    }
                )
            }
        }
    }
    println!("total: {total}");
}
