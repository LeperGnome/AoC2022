use crate::tasks::Task;

pub struct TDay {}

impl Task for TDay { 
    fn compute(&self) -> String {
        let data = include_str!("data.txt"); 
        TDay::some(&data).expect("something should be here").to_string()
    }
}

impl TDay { 
    fn some(_data: &str) -> Option<usize> {
        None
    }
}
