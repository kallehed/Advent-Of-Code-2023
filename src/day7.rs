use std::cmp::Ordering;

use regex::Regex;

pub fn day7_1() {
    let s = std::fs::read_to_string("src/day7.txt").unwrap();

    let mut bids: Vec<i32> = Vec::new();
    let mut cards: Vec<Vec<i32>> = Vec::new();

    let mut orig_cards = Vec::new();

    for line in s.lines() {
        let (c_cards, bid) = line.split_once(' ').unwrap();

        let mut b_cards: Vec<i32> = c_cards.chars().map(|c| {
            match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                other => {(c as u8 - b'0') as i32}
            } 
        }).collect();
        orig_cards.push(b_cards.clone());
        b_cards.sort();
        b_cards.reverse();
        cards.push(b_cards);
        bids.push(bid.parse().unwrap());
    }
    println!("cards: {:?}, bids: {:?}", cards, bids);

    let mut types: Vec<i32> = Vec::new();

    for card in cards.iter() {
        if card.iter().all(|&c| card[0] == c) {
           types.push(6);
        }
        else if card.iter().skip(1).all(|&c| card[1] == c) || card.iter().take(4).all(|&c| card[0] == c) {
            types.push(5);
        }
        else if (card[0] == card[1] && card[1] == card[2] && card[3] == card[4]) || (card[0] == card[1] && card[2] == card[3] && card[3] == card[4]) {
            types.push(4);
        }
        else if (card[0] == card[1] && card[1] == card[2]) || (card[4] == card[3] && card[3] == card[2]) || (card[1] == card[2] && card[2] == card[3]) {
            types.push(3);
        }
        else if {
            let mut count = 0;
            count += (card[0] == card[1]) as i32;
            count += (card[1] == card[2]) as i32;
            count += (card[2] == card[3]) as i32;
            count += (card[3] == card[4]) as i32;
            if count >= 2 {true} else {false}
        } {
            types.push(2);
        }
        else if card.windows(2).any(|c| c[0] == c[1]) {
            types.push(1);
        }
        else {
            types.push(0);
        }
    }
    println!("types: {:?}", types);
    let mut total = 0;
    let mut real: Vec<(i32, i32, Vec<i32>)> = vec![];
    let length = cards.len();
    for i in 0..length {
        real.push((bids[i], types[i], orig_cards[i].clone()));
    }

    real.sort_by(|x, y| {
        if x.1 < y.1 {
            Ordering::Less
        }
        else if x.1 > y.1 {
            Ordering::Greater
        }
        else {
            for i in 0..x.2.len() {
                if x.2[i] < y.2[i] {
                    return Ordering::Less;
                }
                else if x.2[i] > y.2[i] {
                    return Ordering::Greater;
                }
            }
            println!("cards wrong: {:?}, {:?}", x, y);
            panic!();
        }
    });
    println!("reals: {:?}", real);
    for r in real.iter().enumerate() {
        total += (r.0 + 1) as i32 * r.1.0;
    }
    println!("total {total}");
}

pub fn day7_2() {
    let s = std::fs::read_to_string("src/day7.txt").unwrap();

    let mut bids: Vec<i32> = Vec::new();
    let mut cards: Vec<Vec<i32>> = Vec::new();

    let mut orig_cards = Vec::new();

    for line in s.lines() {
        let (c_cards, bid) = line.split_once(' ').unwrap();

        let mut b_cards: Vec<i32> = c_cards.chars().map(|c| {
            match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 0,
                'T' => 10,
                other => {(c as u8 - b'0') as i32}
            } 
        }).collect();
        orig_cards.push(b_cards.clone());
        b_cards.sort();
        b_cards.reverse();
        cards.push(b_cards);
        bids.push(bid.parse().unwrap());
    }
    println!("cards: {:?}, bids: {:?}", cards, bids);

    // turn 0os into better number temporarily

    cards.iter_mut().for_each(|card| {
        let mut appearances = [0; 15];
        for c in card.iter() {
            if *c == 0 {continue;}
            appearances[*c as usize] += 1;
        }
        let mut appear_most_num = 14;
        let mut how_many_times = 0;
        for a in appearances.iter().enumerate() {
            if *a.1 >= how_many_times {
                how_many_times = *a.1;
                appear_most_num = a.0;
            }
        }
        println!("appear most: {appear_most_num}");
        for c in card.iter_mut() {
            if *c == 0 {
                *c = appear_most_num as i32;
            }
        }
        card.sort();
        card.reverse();
    });

    let mut types: Vec<i32> = Vec::new();

    for card in cards.iter() {
        if card.iter().all(|&c| card[0] == c) {
           types.push(6);
        }
        else if card.iter().skip(1).all(|&c| card[1] == c) || card.iter().take(4).all(|&c| card[0] == c) {
            types.push(5);
        }
        else if (card[0] == card[1] && card[1] == card[2] && card[3] == card[4]) || (card[0] == card[1] && card[2] == card[3] && card[3] == card[4]) {
            types.push(4);
        }
        else if (card[0] == card[1] && card[1] == card[2]) || (card[4] == card[3] && card[3] == card[2]) || (card[1] == card[2] && card[2] == card[3]) {
            types.push(3);
        }
        else if {
            let mut count = 0;
            count += (card[0] == card[1]) as i32;
            count += (card[1] == card[2]) as i32;
            count += (card[2] == card[3]) as i32;
            count += (card[3] == card[4]) as i32;
            if count >= 2 {true} else {false}
        } {
            types.push(2);
        }
        else if card.windows(2).any(|c| c[0] == c[1]) {
            types.push(1);
        }
        else {
            types.push(0);
        }
    }
    println!("types: {:?}", types);
    let mut total = 0;
    let mut real: Vec<(i32, i32, Vec<i32>)> = vec![];
    let length = orig_cards.len();
    for i in 0..length {
        real.push((bids[i], types[i], orig_cards[i].clone()));
    }

    real.sort_by(|x, y| {
        if x.1 < y.1 {
            Ordering::Less
        }
        else if x.1 > y.1 {
            Ordering::Greater
        }
        else {
            for i in 0..x.2.len() {
                if x.2[i] < y.2[i] {
                    return Ordering::Less;
                }
                else if x.2[i] > y.2[i] {
                    return Ordering::Greater;
                }
            }
            println!("cards wrong: {:?}, {:?}", x, y);
            panic!();
        }
    });
    println!("reals: {:?}", real);
    for r in real.iter().enumerate() {
        total += (r.0 + 1) as i32 * r.1.0;
    }
    println!("total {total}");

}
