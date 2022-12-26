mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;

trait Task {
    fn compute(&self) -> String;
}

impl dyn Task {
    fn get_by_day(day_n: usize) -> Option<Box<dyn Task>>{
        match day_n {
            1 => Some(Box::new(day1::TDay{})),
            2 => Some(Box::new(day2::TDay{})),
            3 => Some(Box::new(day3::TDay{})),
            4 => Some(Box::new(day4::TDay{})),
            5 => Some(Box::new(day5::TDay{})),
            6 => Some(Box::new(day6::TDay{})),
            7 => Some(Box::new(day7::TDay{})),
            8 => Some(Box::new(day8::TDay{})),
            9 => Some(Box::new(day9::TDay{})),
            10 => Some(Box::new(day10::TDay{})),
            11 => Some(Box::new(day11::TDay{})),
            12 => Some(Box::new(day12::TDay{})),
            14 => Some(Box::new(day14::TDay{})),
            15 => Some(Box::new(day15::TDay{})),
            16 => Some(Box::new(day16::TDay{})),
            17 => Some(Box::new(day17::TDay{})),
            18 => Some(Box::new(day18::TDay{})),
            19 => Some(Box::new(day19::TDay{})),
            20 => Some(Box::new(day20::TDay{})),
            21 => Some(Box::new(day21::TDay{})),
            22 => Some(Box::new(day22::TDay{})),
            23 => Some(Box::new(day23::TDay{})),
            _ => None,
        }
    }
}

pub fn execute(day_n: usize) -> String { 
    <dyn Task>::get_by_day(day_n)
        .expect("Task not found")
        .compute()
}
