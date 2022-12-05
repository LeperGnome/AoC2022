mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

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
            _ => None,
        }
    }
}

pub fn execute(day_n: usize) -> String { 
    <dyn Task>::get_by_day(day_n)
        .expect("Task not found")
        .compute()
}
