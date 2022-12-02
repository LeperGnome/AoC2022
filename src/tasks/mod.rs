mod day1;
mod day2;

trait Task {
    fn compute(&self) -> String;
}

impl dyn Task {
    fn get_by_day(day_n: usize) -> Option<Box<dyn Task>>{
        match day_n {
            1 => Some(Box::new(day1::TDay1{})),
            2 => Some(Box::new(day2::TDay2{})),
            _ => None,
        }
    }
}

pub fn execute(day_n: usize) -> String { 
    <dyn Task>::get_by_day(day_n)
        .expect("Task not found")
        .compute()
}
