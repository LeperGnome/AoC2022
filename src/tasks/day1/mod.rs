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
    fn most_calories(data: &String, n: usize) -> usize {
        let mut calories = data.split("\n\n")
            .map(|x| {
                return x
                    .lines()
                    .flat_map(str::parse::<usize>)
                    .sum::<usize>();
            })
            .collect::<Vec<usize>>();

        calories.sort();
        calories.reverse();

        calories
            .iter()
            .take(n)
            .sum()
    }
}

