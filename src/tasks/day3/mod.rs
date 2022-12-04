use crate::tasks::Task;
use std::collections::{HashSet, HashMap};

pub struct TDay {}

impl Task for TDay { 
    fn compute(&self) -> String {
        let data = include_str!("data.txt"); 
        TDay::calculate_priority(&data).to_string()
    }
}

impl TDay { 
    fn calculate_priority(data: &str) -> u32 {
        let mut m: HashMap<char, u32> = ('a'..='z').zip(1..=26).collect();
        m.extend(('A'..='Z').zip(27..=52));

        let mut res: u32 = 0;
        let mut buf: Vec<&str> = vec![];

        for sack in data.lines() {
            buf.push(sack);
            if buf.len() == 3 { 
                let common2 = &str_intersection(
                    &buf[0].chars().collect::<HashSet<char>>(), 
                    &buf[1].chars().collect::<HashSet<char>>()
                );
                let common3 = str_intersection(
                    common2,
                    &buf[2].chars().collect::<HashSet<char>>()
                );
                res += m[&common3.iter().next().unwrap()];
                buf.clear(); 
            }
        }
        res
    }

}

fn str_intersection(a: &HashSet<char>, b: &HashSet<char>) -> HashSet<char> {
    a.intersection(b).cloned().collect()
}
