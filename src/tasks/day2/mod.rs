use crate::tasks::Task;
use std::fs::read_to_string;

pub struct TDay2 {}

impl Task for TDay2 { 
    fn compute(&self) -> String {
        let data = read_to_string("./src/tasks/day2/data.txt").unwrap(); 
        TDay2::calculate_score(&data).to_string()
    }
}

impl TDay2 { 
    fn _calculate_score_1(data: &String) -> u32 {
        let win_pairs = vec![("A", "Y"), ("B", "Z"), ("C", "X")];
        let draw_pairs = vec![("A", "X"), ("B", "Y"), ("C", "Z")];
        let mut score: u32 = 0;
        for l in data.lines() {
            let (el, you) = l.split_once(' ').unwrap();
            match you {
                "X" => score += 1,
                "Y" => score += 2,
                "Z" => score += 3,
                _ => ()
            }
            if draw_pairs.contains(&(el, you)) {
                score += 3;
                continue;
            } else if win_pairs.contains(&(el, you)) {
                score += 6;
                continue;
            }
        }
        score
    }
    fn calculate_score(data: &String) -> u32 {
        use std::collections::HashMap;

        let win_scores = HashMap::from([
            ("A", 2),
            ("B", 3),
            ("C", 1),
        ]);
        let draw_scores = HashMap::from([
            ("A", 1),
            ("B", 2),
            ("C", 3),
        ]);
        let loose_scores = HashMap::from([
            ("A", 3),
            ("B", 1),
            ("C", 2),
        ]);

        let mut score: u32 = 0;
        for l in data.lines() {
            let (el, you) = l.split_once(' ').unwrap();
            match you {
                "X" => { score += 0; score += loose_scores[el]; },
                "Y" => { score += 3; score += draw_scores[el]; },
                "Z" => { score += 6; score += win_scores[el]; },
                _ => ()
            }
        }
        score
    }
}

