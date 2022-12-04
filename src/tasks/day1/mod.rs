use crate::tasks::Task;

pub struct TDay {}

impl Task for TDay { 
    fn compute(&self) -> String {
        let data = include_str!("data.txt"); 
        TDay::most_calories(&data, 3).to_string()
    }
}

impl TDay { 
    fn most_calories(data: &str, n: usize) -> usize {
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

