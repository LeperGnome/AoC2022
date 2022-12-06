use crate::tasks::Task;

pub struct TDay {}

impl Task for TDay { 
    fn compute(&self) -> String {
        let data = include_str!("data.txt"); 
        TDay::get_first_id(&data, 14).expect("something should be here").to_string()
    }
}

impl TDay { 
    fn get_first_id(data: &str, n: usize) -> Option<usize> {
        use std::collections::HashSet;

        for (idx, win) in data
            .chars()
            .collect::<Vec<char>>()
            .windows(n)
            .enumerate() {
                if HashSet::<char>::from_iter(win.iter().cloned()).len() == n {
                    return Some(idx+n)
                }
        }
        None
    }
}
