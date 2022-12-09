use crate::tasks::Task;
use std::collections::HashSet;

pub struct TDay {}

impl Task for TDay { 
    fn compute(&self) -> String {
        let data = include_str!("data.txt"); 
        TDay::calc_positions(&data).expect("something should be here").to_string()
    }
}

impl TDay { 
    fn calc_positions(data: &str) -> Option<usize> {
        let mut knots = vec![(0, 0); 10];
        let mut visited: HashSet<(i32, i32)> = HashSet::new();

        visited.insert((0, 0));

        for line in data.lines() { 
            let (dir, times) = line
                .split_once(' ')
                .map(|(d, n)|{
                    (
                        d.parse::<char>().unwrap(),
                        n.parse::<i32>().unwrap()
                    )
                })
                .unwrap();

            for _ in 0..times {
                match dir { 
                    'R' => knots[0].0 += 1,
                    'L' => knots[0].0 -= 1,
                    'U' => knots[0].1 += 1,
                    'D' => knots[0].1 -= 1,
                    _   => unreachable!("wrong instructions"),
                }
                let mut next_knot = knots[0].clone();
                for (idx, knot) in knots.iter_mut().enumerate().skip(1){
                    move_tail(knot, &next_knot);
                    if idx == 9 {  // tail
                        visited.insert(*knot);
                    }
                    next_knot = *knot;
                }
            }
        }
        Some(visited.len())
    }
}

fn move_tail(tail: &mut (i32, i32), head: &(i32, i32)) {
    let diff_x = head.0 - tail.0;
    let diff_y = head.1 - tail.1;

    if  diff_x == 2 || diff_x == -2 {
        if  diff_x == 2 {tail.0 += 1;}
        else if diff_x == -2 {tail.0 -= 1;}

        if diff_y == 1 {
            tail.1 += 1;
        } else if diff_y == -1 {
            tail.1 -= 1;
        }
    }

    if  diff_y == 2 || diff_y == -2 {
        if  diff_y == 2 {tail.1 += 1;}
        else if diff_y == -2 {tail.1 -= 1;}

        if diff_x == 1 {
            tail.0 += 1;
        } else if diff_x == -1 {
            tail.0 -= 1;
        }
    }
}
