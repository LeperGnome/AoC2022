use crate::tasks::Task;
use std::collections::{HashSet, HashMap, VecDeque};

pub struct TDay {}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Directoin {
    Up,
    Down,
    Left, 
    Right,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Blizzard {
    dir: Directoin,
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct BlizzardMap {
    vertical: HashMap<i32, Vec<Blizzard>>,
    horizontal: HashMap<i32, Vec<Blizzard>>,
    maxx: i32,
    maxy: i32,
}

impl BlizzardMap {
    fn new() -> Self {
        Self {
            maxx: 0,
            maxy: 0,
            vertical: HashMap::new(),
            horizontal: HashMap::new(),
        }
    }

    fn can_move(&self, pos: &(i32, i32), step: i32) -> bool {
        if *pos == (0, -1) { return true; }

        if pos.0 >= self.maxx
        || pos.0 < 0
        || pos.1 >= self.maxy
        || pos.1 < 0
        {
            return false;
        }

        
        if let Some(horizontals) = self.horizontal.get(&pos.1) {
            for hb in horizontals {
                match hb.dir {
                    Directoin::Left => {
                        let after_x = (-1*step + hb.x).rem_euclid(self.maxx);
                        if (after_x, pos.1) == *pos { return false; }
                    },
                    Directoin::Right => {
                        let after_x = (1*step + hb.x) % self.maxx;
                        if (after_x, pos.1) == *pos { return false; }
                    },
                    _ => unreachable!(),
                }
            }
        }

        if let Some(verticals) = self.vertical.get(&pos.0) {
            for vb in verticals {
                match vb.dir {
                    Directoin::Up => {
                        let after_y = (-1*step + vb.y).rem_euclid(self.maxy);
                        if (pos.0, after_y) == *pos { return false; }
                    },
                    Directoin::Down => {
                        let after_y = (1*step + vb.y) % self.maxy;
                        if (pos.0, after_y) == *pos { return false; }
                    },
                    _ => unreachable!(),
                }
            }
        }

        return true;
    }
}

impl Task for TDay { 
    fn compute(&self) -> String {
        let data = include_str!("data.txt"); 

        let mut bm = BlizzardMap::new();

        for (y, l) in data.lines().skip(1).enumerate() {
            for (x, c) in l.chars().skip(1).enumerate() {
                let x = x as i32;
                let y = y as i32;
                bm.maxx = bm.maxx.max(x);
                bm.maxy = bm.maxy.max(y);
                match c {
                    '<' => bm.horizontal.entry(y).or_insert(vec![]).push(Blizzard{x, y, dir: Directoin::Left}),
                    '>' => bm.horizontal.entry(y).or_insert(vec![]).push(Blizzard{x, y, dir: Directoin::Right}),
                    '^' => bm.vertical.entry(x).or_insert(vec![]).push(Blizzard{x, y, dir: Directoin::Up}),
                    'v' => bm.vertical.entry(x).or_insert(vec![]).push(Blizzard{x, y, dir: Directoin::Down}),
                    _   => (),
                };
            }
        }

        let start: (i32, i32)  = (0, -1);
        let finish: (i32, i32) = (bm.maxx-1, bm.maxy);

        let res1 = TDay::blizzard_basin(&bm, start, finish, 9)
            .expect("something should be here");

        let res2 = TDay::blizzard_basin(&bm, finish, start, res1)
            .expect("something should be here");

        let res3 = TDay::blizzard_basin(&bm, start, finish, res2)
            .expect("something should be here");

        return res3.to_string();

    }
}

impl TDay { 
    fn blizzard_basin(bm: &BlizzardMap, start: (i32, i32), finish: (i32, i32), init_step: i32) -> Option<i32> {

        let mut q = VecDeque::from([(start, init_step)]);
        let mut seen = HashSet::from([(start, init_step)]);

        while q.len() > 0 {
            let (cur_pos, step) = q.pop_front().unwrap();
            let next_step = step + 1;

            for n_diff in [(-1, 0), (1, 0), (0, 1), (0, -1), (0, 0),] {
                let next_pos = (cur_pos.0 + n_diff.0, cur_pos.1 + n_diff.1);

                if next_pos == finish {
                    return Some(next_step);
                } 

                if !seen.contains(&(next_pos, next_step)) && bm.can_move(&next_pos, next_step) {
                    q.push_back((next_pos, next_step));
                    seen.insert((next_pos, next_step));
                }
            }
        }
        None
    }
}
