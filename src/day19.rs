use std::collections::HashMap;

use regex::Regex;

pub fn day19_1() {
    let s = std::fs::read_to_string("src/day19.txt").unwrap();

    const ACCEPT_NUM: i32 = -1;
    const REJECT_NUM: i32 = -2;
    {
        let mut a = s.split("\n\n");
        let workflow_data = a.next().unwrap();
        let parts = a.next().unwrap();

        let workflow2num = {
            let mut workflow2num = HashMap::<&str, i32>::new();

            for (idx, line) in workflow_data.lines().enumerate() {
                workflow2num.insert(line.split('{').next().unwrap(), idx as _);
            }

            workflow2num.insert("A", ACCEPT_NUM);
            workflow2num.insert("R", REJECT_NUM);

            workflow2num
        };
        println!("workflow2num: {:?}", workflow2num);

        // x: 0, m: 1, a: 2, s: 3
        #[derive(Debug, Clone, Copy)]
        enum MoreOrLess {
            Greater,
            Less,
        }

        #[derive(Debug, Clone)]
        struct Workflow {
            work: Vec<(i8, MoreOrLess, i32, i32)>,
            last: i32,
        }

        let workflows = {
            let mut workflows = Vec::<Workflow>::new();
            for (line_idx, line) in workflow_data.lines().enumerate() {
                workflows.push(Workflow {
                    work: Vec::new(),
                    last: -3,
                });
                let my_workflow = &mut workflows[line_idx];
                let line = line.split('{').nth(1).unwrap();
                for work in line.split(',') {
                    match work.chars().nth(1).unwrap() {
                        '>' | '<' => {
                            let piece = match work.chars().nth(0).unwrap() {
                                'x' => 0,
                                'm' => 1,
                                'a' => 2,
                                's' => 3,
                                wrong => panic!("ERROR: got {}", wrong),
                            };
                            let more_or_less = match work.chars().nth(1).unwrap() {
                                '>' => MoreOrLess::Greater,
                                '<' => MoreOrLess::Less,
                                _ => panic!("wrong more or less sign"),
                            };

                            let mut sep = work.split(':');
                            let num = sep.next().unwrap()[2..].parse::<i32>().unwrap();
                            let go_to = *workflow2num.get(sep.next().unwrap()).unwrap();

                            my_workflow.work.push((piece, more_or_less, num, go_to));
                        }
                        _ => {
                            // last work item
                            let go_to = *workflow2num.get(work.split('}').next().unwrap()).unwrap();
                            my_workflow.last = go_to;
                        }
                    }
                }
            }
            workflows
        };
        println!("workflows: {:?}", workflows);

        // x: 0, m: 1, a: 2, s: 3
        let works = {
            let mut works = Vec::<[i32; 4]>::new();
            let reg = Regex::new(r"\w=\d+").unwrap();
            for part in parts.lines() {
                works.push([1230910; 4]);
                println!("{}", part);
                for cap in reg.captures_iter(part) {
                    println!("capture: {:?}", &cap[0]);
                    let which = match cap[0].chars().nth(0).unwrap() {
                        'x' => 0,
                        'm' => 1,
                        'a' => 2,
                        's' => 3,
                        wrong => panic!("ERROR: got {}", wrong),
                    };
                    let num = cap[0].split("=").nth(1).unwrap().parse::<i32>().unwrap();

                    works.last_mut().unwrap()[which] = num;
                }
            }
            works
        };
        println!("works: {:?}", works);

        // actually go through the work and stuff

        let start_workflow = *workflow2num.get("in").unwrap();

        {
            let mut total = 0;
            for &part in works.iter() {
                let mut cur_workflow = start_workflow;

                'leloop: while cur_workflow != ACCEPT_NUM && cur_workflow != REJECT_NUM {
                    let ac_workflow = &workflows[cur_workflow as usize];
                    for &work in ac_workflow.work.iter() {
                        let should_go_to = match work.1 {
                            MoreOrLess::Greater => part[work.0 as usize] > work.2,
                            MoreOrLess::Less => part[work.0 as usize] < work.2,
                        };
                        if should_go_to {
                            cur_workflow = work.3;
                            continue 'leloop;
                        }
                    }
                    // none of them did it
                    cur_workflow = ac_workflow.last;
                }
                if cur_workflow == ACCEPT_NUM {
                    let add: i32 = part.iter().sum();
                    println!("got: {}", add);
                    total += add;
                }
                // println!("end workflow: {}", cur_workflow);
            }
            println!("total: {}", total);
        }
    }
}

pub fn day19_2() {
    let s = std::fs::read_to_string("src/day19.txt").unwrap();

    const ACCEPT_NUM: i32 = -1;
    const REJECT_NUM: i32 = -2;
    {
        let mut a = s.split("\n\n");
        let workflow_data = a.next().unwrap();

        let workflow2num = {
            let mut workflow2num = HashMap::<&str, i32>::new();

            for (idx, line) in workflow_data.lines().enumerate() {
                workflow2num.insert(line.split('{').next().unwrap(), idx as _);
            }

            workflow2num.insert("A", ACCEPT_NUM);
            workflow2num.insert("R", REJECT_NUM);

            workflow2num
        };
        println!("workflow2num: {:?}", workflow2num);

        // x: 0, m: 1, a: 2, s: 3
        #[derive(Debug, Clone, Copy)]
        enum MoreOrLess {
            Greater,
            Less,
        }

        #[derive(Debug, Clone)]
        struct Workflow {
            work: Vec<(i8, MoreOrLess, i32, i32)>,
        }

        let workflows = {
            let mut workflows = Vec::<Workflow>::new();
            for (line_idx, line) in workflow_data.lines().enumerate() {
                workflows.push(Workflow { work: Vec::new() });
                let my_workflow = &mut workflows[line_idx];
                let line = line.split('{').nth(1).unwrap();
                for work in line.split(',') {
                    match work.chars().nth(1).unwrap() {
                        '>' | '<' => {
                            let piece = match work.chars().nth(0).unwrap() {
                                'x' => 0,
                                'm' => 1,
                                'a' => 2,
                                's' => 3,
                                wrong => panic!("ERROR: got {}", wrong),
                            };
                            let more_or_less = match work.chars().nth(1).unwrap() {
                                '>' => MoreOrLess::Greater,
                                '<' => MoreOrLess::Less,
                                _ => panic!("wrong more or less sign"),
                            };

                            let mut sep = work.split(':');
                            let num = sep.next().unwrap()[2..].parse::<i32>().unwrap();
                            let go_to = *workflow2num.get(sep.next().unwrap()).unwrap();

                            my_workflow.work.push((piece, more_or_less, num, go_to));
                        }
                        _ => {
                            // last work item
                            let go_to = *workflow2num.get(work.split('}').next().unwrap()).unwrap();
                            my_workflow.work.push((0, MoreOrLess::Greater, -1, go_to));
                            // greater than -1 will always be true, do this to avoid edge cases
                            // with the last member (which I removed)
                        }
                    }
                }
            }
            workflows
        };
        println!("workflows: {:?}", workflows);

        let rev_workflow = {
            let mut rev_workflow = HashMap::<i32, (i32, i32)>::new();
            for (idx, workflow) in workflows.iter().enumerate() {
                for (work_idx, work) in workflow.work.iter().enumerate() {
                    if work.3 == ACCEPT_NUM || work.3 == REJECT_NUM {
                        continue;
                    }
                    rev_workflow.insert(work.3, (idx as i32, work_idx as i32));
                }
            }
            rev_workflow
        };
        println!("reverse workflows: {:?}", rev_workflow);

        // x: 0, m: 1, a: 2, s: 3
        // println!("works: {:?}", works);

        // reverse engineer from all the A's to get the conditions to get a specific one (could be
        // intervals of the numbers xmas), count them all up and sum them basically.
        //

        #[derive(Debug, Clone)]
        struct Possible {
            xmas: [(i32, i32); 4], // range inclusive
        }

        // first ones shall not happen, last one will be true
        fn hinder_possibility(
            mut pos: Possible,
            works: &[(i8, MoreOrLess, i32, i32)],
        ) -> Option<Possible> {
            for work in works.iter().take(works.len() - 1) {
                let my_pos = &mut pos.xmas[work.0 as usize];
                match work.1 {
                    MoreOrLess::Greater => {
                        if my_pos.0 > work.2 {
                            return None;
                        }
                        if my_pos.1 > work.2 {
                            my_pos.1 = work.2;
                        }
                    }
                    MoreOrLess::Less => {
                        if my_pos.1 < work.2 {
                            return None;
                        }
                        if my_pos.0 < work.2 {
                            my_pos.0 = work.2;
                        }
                    }
                }
            }

            // last one shall be true
            {
                let work = works.last().unwrap();
                let my_pos = &mut pos.xmas[work.0 as usize];

                match work.1 {
                    MoreOrLess::Greater => {
                        if my_pos.1 <= work.2 {
                            return None;
                        }
                        if my_pos.0 <= work.2 {
                            my_pos.0 = work.2 + 1;
                        }
                    }
                    MoreOrLess::Less => {
                        if my_pos.0 >= work.2 {
                            return None;
                        }
                        if my_pos.1 >= work.2 {
                            my_pos.1 = work.2 - 1;
                        }
                    }
                }
            }

            // if pos.xmas.iter().any(|x| x.0 == 0) {return None}

            Some(pos)
        }

        let start_workflow = *workflow2num.get("in").unwrap();

        {
            let mut good_ones = Vec::new();
            for (big_workflow_idx, workflow) in workflows.iter().enumerate() {
                'leloop: for (big_work_idx, &work) in workflow.work.iter().enumerate() {
                    if work.3 == ACCEPT_NUM {
                        // good one
                        let mut pos = Possible {
                            xmas: [(1, 4000); 4],
                        };
                        let mut workflow_idx = big_workflow_idx as i32;
                        let mut work_idx = big_work_idx as i32;

                        loop {
                            // println!("a {} {}", workflow_idx, work_idx);
                            pos = match hinder_possibility(
                                pos,
                                &workflows[workflow_idx as usize].work[0..=(work_idx as usize)],
                            ) {
                                Some(v) => v,
                                None => continue 'leloop,
                            };

                            // go to the one that sent us here
                            if workflow_idx == start_workflow {
                                // calculate stuff
                                good_ones.push(pos);
                                continue 'leloop;
                            }
                            (workflow_idx, work_idx) =
                                *rev_workflow.get(&(workflow_idx as i32)).unwrap();
                        }
                    }
                }
            }

            // filter ones that are inside others

            let mut total: i64 = 0;

            for pos in good_ones.iter() {
                let add = (pos.xmas[0].1 - pos.xmas[0].0 + 1) as i64
                    * (pos.xmas[1].1 - pos.xmas[1].0 + 1) as i64
                    * (pos.xmas[2].1 - pos.xmas[2].0 + 1) as i64
                    * (pos.xmas[3].1 - pos.xmas[3].0 + 1) as i64;

                let add = add as i64;
                println!("got to end, pos: {:?}, add: {}", pos, add);
                total += add;
            }
            println!("total: {}", total);
        }
    }
}
