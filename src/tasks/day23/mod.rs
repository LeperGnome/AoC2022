use crate::tasks::Task;
use std::collections::{HashSet, HashMap};

pub struct TDay {}

impl Task for TDay { 
    fn compute(&self) -> String {
        let data = include_str!("data.txt"); 

        let mut elves = HashSet::new();

        for (y, l) in data.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                match c {
                    '#' => elves.insert((x as i32, y as i32)),
                    _ => continue,
                };
            }
        }

        TDay::unstable_diffusion(elves)
            .expect("something should be here")
            .to_string()
    }
}

impl TDay { 
    fn unstable_diffusion(elves: HashSet<(i32, i32)>) -> Option<usize> {
        for _ in 1..=10 {
            let proposals: HashMap<(i32, i32), ((i32, i32), usize)> = HashMap::new();
            //                      proposal     first elf  elf n

            for elf in elves.iter() {

            }
            // 1. for each elf check if he needs to go somewhere
            // 2. save his tile proposal
            // 3. move each elf to his proposal if proposal count is 1
            // 4. rotate direction vector
        }

        // 5. calculate empty tiles -> area by max coords - number of elves

        Some(0)
    }
}
