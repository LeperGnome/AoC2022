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
            _ => None,
        }
    }
}

pub fn execute(day_n: usize) -> String { 
    <dyn Task>::get_by_day(day_n)
        .expect("Task not found")
        .compute()
}
