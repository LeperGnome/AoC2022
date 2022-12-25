use crate::tasks::Task;

use std::collections::HashMap;

type Map = Vec<Vec<Option<bool>>>;
type Sector = Vec<Vec<Point>>;

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
struct Point {
    blocked: bool,
    global_x: usize,
    global_y: usize,
}


// 
// impl Sector {
//     fn contains(&self, x: &usize, y: &usize) -> bool {
//         return 
//            x >= &self.xfrom 
//         && x <= &self.xto
//         && y >= &self.yfrom
//         && y <= &self.yto;
//     }
// }
// 
// fn get_current_sector_number(x: &usize, y: &usize, sectors: &HashMap<usize, Sector>) -> usize {
//     for (n, sec) in sectors.iter() {
//         if sec.contains(x, y) {
//             return *n;
//         }
//     }
//     return 0;
// }


#[derive(Debug)]
enum Instruction {
    Walk(usize),
    RotateLeft,
    RotateRight,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn get_score(&self) -> usize {
        match self {
            Self::Up => 3,
            Self::Left => 2,
            Self::Down => 1,
            Self::Right => 0,
        }
    }
}

fn rotate(cur_dir: &Direction, instr: &Instruction) -> Direction {
    match instr {
        Instruction::RotateLeft => {
            match cur_dir {
                Direction::Up => Direction::Left,
                Direction::Right => Direction::Up,
                Direction::Down => Direction::Right,
                Direction::Left => Direction::Down,
            }
        },
        Instruction::RotateRight => {
            match cur_dir {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            }
        }
        Instruction::Walk(_) => unreachable!(),
    }
}

fn get_bound(cur_x: &usize, cur_y: &usize, dir: &Direction, map: &Map) -> (usize, usize){
    match dir {
        Direction::Right | Direction::Left => {
            let lb = map[*cur_y]
                .iter()
                .position(|x| if let Some(_) = x { true } else { false })
                .unwrap();
            let rb = map[*cur_y]
                .iter()
                .rposition(|x| if let Some(_) = x { true } else { false })
                .unwrap();
            return (lb, rb);
        }
        Direction::Up | Direction::Down => {
            let lb = map
                .iter()
                .position(|x| if let Some(_) = x[*cur_x] { true } else { false })
                .unwrap();
            let rb = map
                .iter()
                .rposition(|x| if let Some(_) = x[*cur_x] { true } else { false })
                .unwrap();
            return (lb, rb);
        }
    }
}

fn move_me(cur_x: usize, cur_y: usize, dir: &Direction, map: &Map) -> Option<(usize, usize)> {
    let mut new_x = cur_x;
    let mut new_y = cur_y;

    let (lb, ub) = get_bound(&cur_x, &cur_y, &dir, &map);

    match dir {
        Direction::Right => {
            if cur_x == ub { new_x = lb; } else { new_x = cur_x + 1; }
        },
        Direction::Left => {
            if cur_x == lb { new_x = ub; } else { new_x = cur_x - 1; }
        },
        Direction::Down => {
            if cur_y == ub { new_y = lb; } else { new_y = cur_y + 1; }
        },
        Direction::Up => {
            if cur_y == lb { new_y = ub; } else { new_y = cur_y - 1; }
        },
    }

    match map[new_y][new_x] {
        Some(v) => {
            if v { None } else { Some((new_x, new_y)) }
        },
        None => unreachable!(),
    }
}

pub struct TDay {}

impl Task for TDay { 
    fn compute(&self) -> String {
        let data = include_str!("data.txt"); 

        let (map_raw, instr) = data.split_once("\n\n").unwrap();

        let mut sectors: HashMap<usize, Sector> = HashMap::new();
        let side = 50;

        for (y, l) in  map_raw.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                let sector_n = (x / side) + ((y / side) * 4);

                let p = match c {
                    '.' => Point{ global_x: x, global_y: y, blocked: false},
                    '#' => Point{ global_x: x, global_y: y, blocked: true},
                    _ => continue,
                };

                (*sectors.entry(sector_n).or_insert(vec![vec![]; 50]))[y % 50].push(p);
            }
        }

        dbg!(&sectors);

        // requires trailing whitespaces!
        let map = map_raw.lines()
            .map(|l| {
                l.chars().map(|c| {
                    match c {
                        ' ' => None,
                        '.' => Some(false),
                        '#' => Some(true),
                        _ => unreachable!(),
                    }
                }).collect::<Vec<Option<bool>>>()
            }).collect::<Map>();

        let mut instructions: Vec<Instruction> = vec![];
        let mut buf = vec![];
        for c in instr.chars() {
            if c.is_digit(10) {
                buf.push(c.to_digit(10).unwrap());
            } else {
                let num = buf
                    .iter()
                    .rev()
                    .enumerate()
                    .fold(0, |acc, (idx, x)| acc + x * 10_u32.pow(idx as u32));

                buf.clear();

                instructions.push(Instruction::Walk(num as usize));
                match c {
                    'R' => instructions.push(Instruction::RotateRight),
                    'L' => instructions.push(Instruction::RotateLeft),
                    '\n' => (),
                    _ => unreachable!(),
                }
            }
        }

        TDay::monkey_map(map, instructions)
            .expect("something should be here")
            .to_string()
    }
}



impl TDay { 
    fn monkey_map(map: Map, instructions: Vec<Instruction>) -> Option<usize> {
        let mut start_x = 0;

        for (x, el) in map.first().unwrap().iter().enumerate() {
            if let Some(_) = el {
                start_x = x;
                break;
            }
        }

        let mut cur_x: usize = start_x;
        let mut cur_y: usize = 0;
        let mut cur_dir = Direction::Right;

        for instruction in instructions {
            match instruction {
                Instruction::Walk(n) => { 
                    for _ in 1..=n {
                        match move_me(cur_x, cur_y, &cur_dir, &map) {
                            Some((new_x, new_y)) => {(cur_x, cur_y) = (new_x, new_y);},
                            None => break,
                        }
                    }
                },
                _ => { cur_dir = rotate(&cur_dir, &instruction) }
            }
        }
        Some(1000 * (cur_y+1) + 4 * (cur_x+1) + cur_dir.get_score())
    }
}
