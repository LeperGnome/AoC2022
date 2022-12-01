use crate::tasks::Task;
use std::fs::read_to_string;

pub struct TDay1 {}

impl Task for TDay1 { 
    fn compute(&self) -> String {
        let data = read_to_string("./src/tasks/day1/data.txt").unwrap(); 
        TDay1::most_calories(&data, 3).to_string()
    }
}

impl TDay1 { 
    fn most_calories(data: &String, n: usize) -> u32 {
        let mut calories = data.split("\n\n")
            .map(|x| {
                x.lines()
                    .map(|x| x
                         .to_string()
                         .parse::<u32>()
                         .unwrap()
                    )
                    .collect::<Vec<u32>>()
                    .into_iter()
                    .sum()
            })
            .collect::<Vec<u32>>();
        calories.sort();
        calories.into_iter()
            .rev()
            .take(n)
            .sum()
    }
}

