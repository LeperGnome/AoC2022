use crate::tasks::Task;
use std::str::FromStr;
use std::fmt;


#[derive(Debug, Clone)]
struct ParseInstructionError;

impl fmt::Display for ParseInstructionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parsin error occured")
    }
}

enum Instruction {
    Addx(i32),
    Noop,
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(' ') {
            Some((c, a)) => {
                match c {
                    "addx" => Ok(
                        Self::Addx(
                            a
                            .parse::<i32>()
                            .expect("addx instruction expexts valid number")
                        )
                    ),
                    _ => Err(ParseInstructionError)
                }
            },
            None         => Ok(Self::Noop),
        }
    }
}

fn get_busy_dur(instr: &Instruction) -> i32 {
    match instr {
        Instruction::Addx(_) => 2,
        Instruction::Noop => 1,
    }
}

pub struct TDay {}

impl Task for TDay { 
    fn compute(&self) -> String {
        let data = include_str!("data.txt"); 
        TDay::calc_signal(&data).expect("something should be here").to_string()
    }
}

impl TDay { 
    fn calc_signal(data: &str) -> Option<i32> {
        let mut instructions = data
            .lines()
            .map(|x| x.parse::<Instruction>());

        let mut x: i32 = 1;
        let mut cycle: usize = 0;
        let mut cur_inst = Instruction::Noop;
        let mut busy_for = 0;

        loop {
            if busy_for > 1 {
                busy_for -= 1;
            } else {
                match cur_inst {
                    Instruction::Addx(v) => x += v,
                    Instruction::Noop => (),

                }
                if let Some(i) = instructions.next() {
                    let next_inst = i.unwrap();
                    busy_for = get_busy_dur(&next_inst);
                    cur_inst = next_inst;

                } else {
                    break;
                }
            }
            draw_pixel(&cycle, &x);
            cycle += 1;
        }
        println!();
        Some(0)
    }
}

fn draw_pixel(cycle_n: &usize, sprite_pos: &i32) {
    if cycle_n % 40 == 0 {
        println!();
    }
    
    let pixel_n = (cycle_n % 40) as i32;
    let x = *sprite_pos;
    if pixel_n == x || pixel_n == x-1 || pixel_n == x+1 {
        print!("#");
    } else { 
        print!(".");
    }
}
