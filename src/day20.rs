use std::collections::HashMap;

use regex::Regex;

pub fn day20_1() {
    let s = std::fs::read_to_string("src/day20.txt").unwrap();

    let mut connections = HashMap::<&str, Vec<String>>::new();
    #[derive(Clone, Debug)]
    enum Type<'a> {
        Flip(bool),
        Conj(HashMap<&'a str, bool>),
        Broadcaster,
    }
    let reg = Regex::new(r"\w+").unwrap();
    let mut types = HashMap::<&str, Type>::new();
    {
        for line in s.lines() {
            let mut splitter = line.split(" ");
            let mut name = splitter.next().unwrap();
            if name.starts_with("%") || name.starts_with("&") {
                let ty = name.chars().next().unwrap();
                name = &name[1..];
                types.insert(name, match ty {
                    '&' => Type::Conj(HashMap::new()),
                    '%' => Type::Flip{0:false},
                    _ => panic!()
                });
            }
            let mut v = vec![];
            splitter.next();
            for connects_to in splitter {
                println!("connects to: {}", connects_to);
                for cap in reg.captures_iter(connects_to) {
                    v.push(cap[0].to_owned());
                }
            }
            connections.insert(name, v);
            
        }
    }

    // initialize conj & ones, so that they have a map for who sends to them
    {
        for ty in types.iter_mut() {
            if let Type::Conj(conj) = ty.1 {
                for cons in connections.iter() {
                    for con in cons.1.iter() {
                        if con == ty.0 {
                            conj.insert(cons.0, false); 
                        }
                    }
                }
            }
        }
    }
    types.insert("broadcaster", Type::Broadcaster);

    println!("connections: {:?}", connections);
    println!("types: {:?}", types);



    // use bools for pulses, false is low, true is high
    
    {
        // sent from, to whom and what value
        let mut cur_pulses = Vec::<(&str, &str, bool)>::new();
        let mut next_pulses = Vec::<(&str, &str, bool)>::new();
        let mut button_presses = 0;
        let mut low_pulses = 0;
        let mut high_pulses = 0;

        // process cur_pulses and add to next_pulses
        loop {
            if cur_pulses.len() == 0 {
                // we have processed current frame of signals, now process next
                cur_pulses = next_pulses.clone();
                next_pulses.clear();
                // if we did not get any new stuff, push button again
                if cur_pulses.len() == 0 {
                    // if button_presses == 1000 {
                    //     break;
                    // }
                    button_presses += 1;
                    cur_pulses.push(("button", "broadcaster", false));

                }
            }
            let (pulse_sent_from, pulse_to, pulse) = cur_pulses.pop().unwrap();
            if pulse {
                high_pulses += 1;
            }
            else {
                low_pulses += 1;
            }
            // apply pulse
            {
                // println!("pulse_to: {}", pulse_to);
                if pulse_to == "rx" && !pulse {
                    println!("took: {}", button_presses);
                    break;
                }
                if pulse_to == "output" {
                    continue;
                }
                let part = match types.get_mut(pulse_to) {
                    Some(x) => x,
                    None => continue,
                };
                let mut should_send = false;
                let mut send_what = false;
                match part {
                    Type::Flip(state) => {
                        if pulse == false {
                            should_send = true;
                            send_what = !*state;
                            *state = !*state;
                        } 
                    },
                    Type::Conj(map) => {
                        map.insert(pulse_sent_from, pulse);
                        should_send = true;
                        send_what = !map.values().all(|x| *x);
                    },
                    Type::Broadcaster => {
                        should_send = true;
                        send_what = pulse;
                    }
                }
                if should_send {
                    for con in connections.get(pulse_to).unwrap() {
                        next_pulses.push((pulse_to, &con, send_what));
                    }
                }
            }
        }

        println!("result: low: {} high: {}, mult: {} ", low_pulses, high_pulses, low_pulses * high_pulses);
    }

}

// for part 2 I looked at the ones contributing to rx, and looked at their repetition rate,
// multiplied all of them  toghether because they were prime and got the answer

pub fn day20_2() {
    let s = std::fs::read_to_string("src/day20.txt").unwrap();

    let mut connections = HashMap::<&str, Vec<String>>::new();
    #[derive(Clone, Debug)]
    enum Type<'a> {
        Flip(bool),
        Conj(HashMap<&'a str, bool>),
        Broadcaster,
    }
    let reg = Regex::new(r"\w+").unwrap();
    let mut types = HashMap::<&str, Type>::new();
    {
        for line in s.lines() {
            let mut splitter = line.split(" ");
            let mut name = splitter.next().unwrap();
            if name.starts_with("%") || name.starts_with("&") {
                let ty = name.chars().next().unwrap();
                name = &name[1..];
                types.insert(name, match ty {
                    '&' => Type::Conj(HashMap::new()),
                    '%' => Type::Flip{0:false},
                    _ => panic!()
                });
            }
            let mut v = vec![];
            splitter.next();
            for connects_to in splitter {
                println!("connects to: {}", connects_to);
                for cap in reg.captures_iter(connects_to) {
                    v.push(cap[0].to_owned());
                }
            }
            connections.insert(name, v);
            
        }
    }

    // initialize conj & ones, so that they have a map for who sends to them
    {
        for ty in types.iter_mut() {
            if let Type::Conj(conj) = ty.1 {
                for cons in connections.iter() {
                    for con in cons.1.iter() {
                        if con == ty.0 {
                            conj.insert(cons.0, false); 
                        }
                    }
                }
            }
        }
    }
    types.insert("broadcaster", Type::Broadcaster);

    println!("connections: {:?}", connections);
    println!("types: {:?}", types);



    // use bools for pulses, false is low, true is high
    
    {
        // sent from, to whom and what value
        let mut cur_pulses = Vec::<(&str, &str, bool)>::new();
        let mut next_pulses = Vec::<(&str, &str, bool)>::new();
        let mut button_presses = 0;

        // process cur_pulses and add to next_pulses
        loop {
            if cur_pulses.len() == 0 {
                // we have processed current frame of signals, now process next
                cur_pulses.append(&mut next_pulses);
                next_pulses.clear();
                // if we did not get any new stuff, push button again
                if cur_pulses.len() == 0 {
                    if button_presses % 100000 == 0 {
                        println!("presses: {}", button_presses);
                     }


                    button_presses += 1;
                    cur_pulses.push(("button", "broadcaster", false));
                }
            }
            {
                    let ns = types.get("ns").unwrap();
                    // print when nl is not all ones
                    if let Type::Conj(ha) = ns {
                        for h in ha.iter() {
                            if *h.1 && *h.0 == "dc" { // THIS LINE HERE IS THE ONE YOU
                                                      // CHANGEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEEE
                                println!("{} true at {}", h.0, button_presses);
                            } 
                        }
                    }

            }
            let (pulse_sent_from, pulse_to, pulse) = cur_pulses.pop().unwrap();
            // apply pulse
            {
                // println!("pulse_to: {}", pulse_to);
                let part = match types.get_mut(pulse_to) {
                    Some(x) => x,
                    None => continue,
                };
                let mut should_send = false;
                let mut send_what = false;
                match part {
                    Type::Flip(state) => {
                        if pulse == false {
                            should_send = true;
                            send_what = !*state;
                            *state = !*state;
                        } 
                    },
                    Type::Conj(map) => {
                        map.insert(pulse_sent_from, pulse);
                        should_send = true;
                        send_what = !map.values().all(|x| *x);
                    },
                    Type::Broadcaster => {
                        should_send = true;
                        send_what = pulse;
                    }
                }
                if should_send {
                    for con in connections.get(pulse_to).unwrap() {
                        next_pulses.push((pulse_to, &con, send_what));
                    }
                }
            }
        }
    }
}
