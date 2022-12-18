use crate::tasks::Task;

use std::iter::Cycle;
use std::str::FromStr;
use std::collections::VecDeque;

type PatternCycle = Cycle<std::vec::IntoIter<JetDir>>;
type Rock = Vec<VecDeque<bool>>;
type Map  = Vec<VecDeque<bool>>;

#[derive(Debug, Clone)]
enum JetDir {
    Left,
    Right,
}

impl FromStr for JetDir { 
    type Err = std::str::Utf8Error; // too lazy

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "<" => Ok(JetDir::Left),
            ">" => Ok(JetDir::Right),
            _   => panic!("Bad char"),
        }
    }
}

fn could_be_merged(rock: &Rock, map: &Map) -> bool {
    // rock and map are equal sized
    let n_lines = rock.len();
    let n_cols  = rock[0].len();
    for i in 0..n_lines {
        for j in 0..n_cols {
            if rock[i][j] && map[i][j] {
                return false;
            }
        }
    }
    return true;
}

fn merged(rock: &Rock, map: &Map) -> Rock {
    let mut res = rock.clone();

    let n_lines = rock.len();
    let n_cols  = rock[0].len();

    for i in 0..n_lines {
        for j in 0..n_cols {
            res[i][j] = rock[i][j] || map[i][j];
        }
    }
    return res;
}

fn move_rock(rock: &mut Rock, dir: &JetDir) {
    match dir {
        JetDir::Left => {
            // print!("<");
            let can_move_left = rock
                .iter()
                .fold(false, |acc, x| acc || x[0]) == false;
            if can_move_left { 
                for line in rock {
                    line.rotate_left(1);
                }
            }
        }
        JetDir::Right => {
            // print!(">");
            let can_move_left = rock
                .iter()
                .fold(false, |acc, x| acc || *x.iter().last().unwrap()) == false;
            if can_move_left { 
                for line in rock {
                    line.rotate_right(1);
                }
            }
        }
    }
}

struct Chamber {
    map: Vec<VecDeque<bool>>,
    jet_pattern: PatternCycle,
    drained: usize,
}

impl Chamber {
    fn new(jet_pattern: PatternCycle) -> Self {
        Chamber {
            map: vec![VecDeque::from([true; 7])],
            jet_pattern,
            drained: 0,
        }
    }

    fn add_rock(&mut self, mut rock: Rock) {
        let original_map_len = self.map.len();

        // print_rock(&rock);
        // moves rock left/right, so it's in position close to other rocks/floor
        for _ in 0..4 { 
            move_rock(&mut rock, &self.jet_pattern.next().unwrap());
            //print_rock(&rock);
        }

        // adds empty space to chamber
        for _ in 0..rock.len() {
            self.map.push(VecDeque::from([false; 7]));
        }

        let mut shift = 0;
        loop {
            let map_window = self.map
                .iter()
                .rev()
                .skip(shift + 1)
                .take(rock.len())
                .rev()
                .cloned()
                .collect();

            // check if can move down
            if !could_be_merged(&rock, &map_window) {
                break;
            }
            shift += 1;

            // check if can move left/right after moving down
            let mut rock_tmp = rock.clone();
            let next_dir = self.jet_pattern.next().unwrap();
            // println!("checking move {:?}", next_dir);
            move_rock(&mut rock_tmp, &next_dir);
            // print_rock(&rock);
            if could_be_merged(
                &rock_tmp,
                &map_window
            ) {
                // println!("could move, so moved {:?}", next_dir);
                // moving if can
                move_rock(&mut rock, &next_dir);
            } else {
                // println!("could not move {:?}", next_dir);
            }
            // print_rock(&rock);
        } 

        // replace map part with rock
        let map_len = self.map.len();
        let integrated_rock = merged(&rock, &self.map[map_len-rock.len()-shift..map_len-shift].to_vec());
        self.map.splice(
            map_len-rock.len()-shift..map_len-shift,
            integrated_rock,
        );


        // remove extra empty lines from map
        for _ in 0..shift {
            if self.map.len() == original_map_len {
                break;
            }
            self.map.pop();
        }

        // draining redundant parts of map
        let mut strip_to: Option<usize> = None;
        for (ridx, line) in self.map.iter().rev().enumerate() {
            if line.iter().all(|x| *x) && ridx != self.map.len()-1 {
                let end_idx = self.map.len() - ridx - 1;
                self.drained += end_idx;
                strip_to = Some(end_idx);
                break;
            }
        }

        if let Some(idx) = strip_to {
            self.map.drain(0..idx);
        }

        // self.print_map();

    }

    fn print_map(&self) {
        println!();
        for row in self.map.iter().rev() { 
            for col in row {
                match col {
                    true => print!("#"),
                    false => print!("."),
                }
            }
            println!();
        }
        println!();
    }
}

fn print_rock(rock: &Rock) {
    println!();
    for row in rock.iter().rev() { 
        for col in row {
            match col {
                true => print!("#"),
                false => print!("."),
            }
        }
        println!();
    }
    println!();
}

pub struct TDay {}

impl Task for TDay { 
    fn compute(&self) -> String {
        let data = include_str!("data.txt"); 

        let directions = data
            .strip_suffix("\n")
            .unwrap()
            .chars()
            .map(|x| x.to_string().parse::<JetDir>().unwrap())
            .collect::<Vec<JetDir>>();

        TDay::rock_tetris(directions)
            .expect("something should be here")
            .to_string()
    }
}

impl TDay {
    fn rock_tetris(directions: Vec<JetDir>) -> Option<usize> {
        let mut chamber = Chamber::new(directions.into_iter().cycle());

        let rocks: Vec<Rock> = vec![
            Vec::from([VecDeque::from([false, false, true, true, true, true, false])]),
            Vec::from([
                VecDeque::from([false, false, false, true, false, false, false]),
                VecDeque::from([false, false, true,  true, true,  false, false]),
                VecDeque::from([false, false, false, true, false, false, false]),
            ]),
            Vec::from([
                VecDeque::from([false, false, true,  true,  true, false, false]),
                VecDeque::from([false, false, false, false, true, false, false]),
                VecDeque::from([false, false, false, false, true, false, false]),
            ]),
            Vec::from([
                VecDeque::from([false, false, true, false, false, false, false]),
                VecDeque::from([false, false, true, false, false, false, false]),
                VecDeque::from([false, false, true, false, false, false, false]),
                VecDeque::from([false, false, true, false, false, false, false]),
            ]),
            Vec::from([
                VecDeque::from([false, false, true, true, false, false,  false]),
                VecDeque::from([false, false, true, true, false, false,  false]),
            ]),
        ];
        
        let mut rocks_cycle = rocks.iter().cycle();

        let steps: usize = 1_000_000_000_000;

        for n in 0..steps {
            if n % 100_000 == 0 { println!("Done with {} / {}", n, steps);}

            let rock = rocks_cycle.next().unwrap();
            chamber.add_rock(rock.clone())
        }

        // chamber.print_map();

        return Some(chamber.map.len() + chamber.drained - 1);  // floor included in len(), so -1
    }
}
