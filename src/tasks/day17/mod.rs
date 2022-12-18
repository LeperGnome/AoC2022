use crate::tasks::Task;

use std::iter::{Cycle, Peekable};
use std::str::FromStr;
use std::collections::{VecDeque, HashMap};

type PatternCycle = Peekable<Cycle<std::vec::IntoIter<JetDir>>>;
type Rock = Vec<VecDeque<bool>>;
type Map  = Vec<VecDeque<bool>>;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
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
    snapsots: HashMap<SnapshotKey, Vec<SnapshotValue>>,
}

impl Chamber {
    fn new(jet_pattern: PatternCycle) -> Self {
        Chamber {
            map: vec![VecDeque::from([true; 7])],
            jet_pattern,
            snapsots: HashMap::new(),
        }
    }

    fn add_rock(&mut self, mut rock: Rock, rock_n: usize) {
        let original_map_len = self.map.len();

        // moves rock left/right, so it's in position close to other rocks/floor
        for _ in 0..4 { 
            move_rock(&mut rock, &self.jet_pattern.next().unwrap());
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
            move_rock(&mut rock_tmp, &next_dir);
            if could_be_merged(
                &rock_tmp,
                &map_window
            ) {
                // moving if can
                move_rock(&mut rock, &next_dir);
            }
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

        // adding map snapsots
        let snapsot_key = SnapshotKey {
            current_rock: rock_n % 5,
            n_last_layers: self.map.iter().rev().take(200).cloned().collect(),
            next_dir: self.jet_pattern.peek().unwrap().clone(),
        };

        let snapshot_value = SnapshotValue {
            rock_n,
            hight: self.map.len() - 1,
        };

        self.snapsots.entry(snapsot_key).or_insert(vec![]).push(snapshot_value);
    }

    fn get_best_snapshot(&self) -> Vec<SnapshotValue> {
        let best_snapsot = self.snapsots
            .iter()
            .filter(|(_, vs)| vs.len() > 1)
            .max_by_key(|(_, vs)| vs.len())
            .unwrap()
            .1;
        return best_snapsot.clone();

    }

    #[allow(dead_code)]
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

#[derive(Debug, Hash, Eq, PartialEq)]
struct SnapshotKey {
    current_rock: usize,  // mod of rock_n
    n_last_layers: Map,
    next_dir: JetDir,
}

#[derive(Debug, Clone, Copy)]
struct SnapshotValue {
    rock_n: usize,
    hight: usize,
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

        let first_chamber = TDay::rock_tetris(directions.clone(), 10000);

        let best_snapsot = first_chamber.get_best_snapshot();

        let rock_diff = best_snapsot[1].rock_n - best_snapsot[0].rock_n;
        let hight_diff = best_snapsot[1].hight - best_snapsot[0].hight;

        let rest_rocks = ((1000000000000 -  best_snapsot[0].rock_n) % rock_diff) + best_snapsot[0].rock_n;

        let second_chamber = TDay::rock_tetris(directions.clone(), rest_rocks);

        return (
            (
                ((1000000000000 -  best_snapsot[0].rock_n) / rock_diff) * hight_diff
            ) + second_chamber.map.len() - 1
        ).to_string();
    }
}

impl TDay {
    fn rock_tetris(directions: Vec<JetDir>, n_rocks: usize) -> Chamber {
        let mut chamber = Chamber::new(directions.into_iter().cycle().peekable());

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

        for n in 0..n_rocks{
            let rock = rocks_cycle.next().unwrap();
            chamber.add_rock(rock.clone(), n);
        }

        return chamber;
    }
}
