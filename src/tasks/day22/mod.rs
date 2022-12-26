use crate::tasks::Task;

use std::collections::HashMap;

type Sector = Vec<Vec<Point>>;

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
struct Point {
    blocked: bool,
    global_x: usize,
    global_y: usize,
}

#[derive(Debug)]
enum Instruction {
    Walk(usize),
    RotateLeft,
    RotateRight,
}

#[derive(Debug, Clone, Copy)]
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

fn move_me(
    cur_x: &mut usize,
    cur_y: &mut usize,
    cur_sector: &mut usize,
    dir: &mut Direction,
    sectors: &HashMap<usize, Sector>
) -> bool {
    let mut new_x = cur_x.clone();
    let mut new_y = cur_y.clone();
    let mut new_dir = dir.clone();
    let mut new_sector = cur_sector.clone();

    match dir {
        Direction::Up => {
            if *cur_y != 0 { 
                new_y -= 1;
            } else {
                match cur_sector {
                    1 => {
                        new_sector = 9;
                        new_x = 0;
                        new_y = *cur_x;
                        new_dir = Direction::Right;
                    },
                    2 => {
                        new_sector = 9;
                        new_x = *cur_x;
                        new_y = 49;
                        new_dir = Direction::Up;
                    },
                    4 => {
                        new_sector = 1;
                        new_x = *cur_x;
                        new_y = 49;
                        new_dir = Direction::Up;
                    },
                    6 => {
                        new_sector = 4;
                        new_x = 0;
                        new_y = *cur_x;
                        new_dir = Direction::Right;
                    },
                    7 => {
                        new_sector = 4;
                        new_x = *cur_x;
                        new_y = 49;
                        new_dir = Direction::Up;
                    },
                    9 => {
                        new_sector = 6;
                        new_x = *cur_x;
                        new_y = 49;
                        new_dir = Direction::Up;
                    },
                    _ => unreachable!(),
                }
            }
        },
        Direction::Down => {
            if *cur_y != 49 { 
                new_y += 1;
            } else {
                match cur_sector {
                    1 => {
                        new_sector = 4;
                        new_x = *cur_x;
                        new_y = 0;
                        new_dir = Direction::Down;
                    },
                    2 => {
                        new_sector = 4;
                        new_x = 49;
                        new_y = *cur_x;
                        new_dir = Direction::Left;
                    },
                    4 => {
                        new_sector = 7;
                        new_x = *cur_x;
                        new_y = 0;
                        new_dir = Direction::Down;
                    },
                    6 => {
                        new_sector = 9;
                        new_x = *cur_x;
                        new_y = 0;
                        new_dir = Direction::Down;
                    },
                    7 => {
                        new_sector = 9;
                        new_x = 49;
                        new_y = *cur_x;
                        new_dir = Direction::Left;
                    },
                    9 => {
                        new_sector = 2;
                        new_x = *cur_x;
                        new_y = 0;
                        new_dir = Direction::Down;
                    },
                    _ => unreachable!(),
                }
            }
        },
        Direction::Right => {
            if *cur_x != 49 { 
                new_x += 1;
            } else {
                match cur_sector {
                    1 => {
                        new_sector = 2;
                        new_x = 0;
                        new_y = *cur_y;
                        new_dir = Direction::Right;
                    },
                    2 => {
                        new_sector = 7;
                        new_x = 49;
                        new_y = 49 - *cur_y;
                        new_dir = Direction::Left;
                    },
                    4 => {
                        new_sector = 2;
                        new_x = *cur_y;
                        new_y = 49;
                        new_dir = Direction::Up;
                    },
                    6 => {
                        new_sector = 7;
                        new_x = 0;
                        new_y = *cur_y;
                        new_dir = Direction::Right;
                    },
                    7 => {
                        new_sector = 2;
                        new_x = 49;
                        new_y = 49 - *cur_y;
                        new_dir = Direction::Left;
                    },
                    9 => {
                        new_sector = 7;
                        new_x = *cur_y;
                        new_y = 49;
                        new_dir = Direction::Up;
                    },
                    _ => unreachable!(),
                }
            }
        },
        Direction::Left => {
            if *cur_x != 0 { 
                new_x -= 1;
            } else {
                match cur_sector {
                    1 => {
                        new_sector = 6;
                        new_x = 0;
                        new_y = 49 - *cur_y;
                        new_dir = Direction::Right;
                    },
                    2 => {
                        new_sector = 1;
                        new_x = 49;
                        new_y = *cur_y;
                        new_dir = Direction::Left;
                    },
                    4 => {
                        new_sector = 6;
                        new_x = *cur_y;
                        new_y = 0;
                        new_dir = Direction::Down;
                    },
                    6 => {
                        new_sector = 1;
                        new_x = 0;
                        new_y = 49 - *cur_y;
                        new_dir = Direction::Right;
                    },
                    7 => {
                        new_sector = 6;
                        new_x = 49;
                        new_y = *cur_y;
                        new_dir = Direction::Left;
                    },
                    9 => {
                        new_sector = 1;
                        new_x = *cur_y;
                        new_y = 0;
                        new_dir = Direction::Down;
                    },
                    _ => unreachable!(),
                }
            }
        },
    }

    dbg!(&new_sector);
    match sectors.get(&new_sector).unwrap()[new_y][new_x].blocked {
        false => { 
            *cur_x = new_x;
            *cur_y = new_y;
            *dir = new_dir;
            *cur_sector = new_sector;
            false
        }
        true => true,
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
                let sector_n = (x / side) + ((y / side) * 3);

                let p = match c {
                    '.' => Point{ global_x: x, global_y: y, blocked: false},
                    '#' => Point{ global_x: x, global_y: y, blocked: true},
                    _ => continue,
                };

                (*sectors.entry(sector_n).or_insert(vec![vec![]; 50]))[y % 50].push(p);
            }
        }

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

        TDay::monkey_map(sectors, instructions)
            .expect("something should be here")
            .to_string()
    }
}

impl TDay { 
    fn monkey_map(sectors: HashMap<usize, Sector>, instructions: Vec<Instruction>) -> Option<usize> {
        let mut cur_sector = 1_usize;
        let mut cur_x: usize = 0;
        let mut cur_y: usize = 0;
        let mut cur_dir = Direction::Right;

        dbg!(&sectors.keys());

        for instruction in instructions {
            match instruction {
                Instruction::Walk(n) => { 
                    for _ in 1..=n {
                        if move_me(&mut cur_x, &mut cur_y, &mut cur_sector, &mut cur_dir, &sectors){
                            break;
                        }
                    }
                },
                _ => { cur_dir = rotate(&cur_dir, &instruction) }
            }
        }
        let s = sectors.get(&cur_sector).unwrap();
        Some(1000 * (s[cur_y][cur_x].global_y+1) + 4 * (s[cur_y][cur_x].global_x+1) + cur_dir.get_score())
    }
}
