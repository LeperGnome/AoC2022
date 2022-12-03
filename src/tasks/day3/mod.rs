use crate::tasks::Task;
use std::collections::{HashSet, HashMap};
use std::fs::read_to_string;

pub struct TDay3 {}

impl Task for TDay3 { 
    fn compute(&self) -> String {
        let data = read_to_string("./src/tasks/day3/data.txt").unwrap(); 
        TDay3::calculate_priority(&data).to_string()
    }
}

impl TDay3 { 
    fn calculate_priority(data: &String) -> u32 {
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
