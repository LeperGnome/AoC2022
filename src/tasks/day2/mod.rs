use crate::tasks::Task;

pub struct TDay {}

impl Task for TDay { 
    fn compute(&self) -> String {
        let data = include_str!("data.txt"); 
        TDay::calculate_score(&data).to_string()
    }
}

impl TDay { 
    fn _calculate_score_1(data: &str) -> u32 {
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
    fn calculate_score(data: &str) -> u32 {
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

    fn _calculate_score_exp(data: &String) -> u32 {
        use std::collections::HashMap;

        let rps = HashMap::from([("A", 0), ("B", 1), ("C", 2)]);
        let scores = vec![1,2,3];

        let mut score: u32 = 0;
        for l in data.lines() {
            let (el, you) = l.split_once(' ').unwrap();
            match you {
                "X" => { score += 0; score += scores[(rps[el]-1)%3] },
                "Y" => { score += 3; score += scores[rps[el]]; },
                "Z" => { score += 6; score += scores[(rps[el]+1)%3]; },
                _ => ()
            }
        }
        score
    }
}

