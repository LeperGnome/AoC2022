use crate::tasks::Task;

pub struct TDay {}

impl Task for TDay { 
    fn compute(&self) -> String {
        let data = include_str!("data.txt"); 
        TDay::get_top_crates(&data)
    }
}

impl TDay { 
    fn get_top_crates(data: &str) -> String {
        let (crates, instructions) = data.split_once("\n\n").unwrap();
        let mut crates_rev_iter = crates.lines().rev();
        let stacks_num = crates_rev_iter
            .next()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let mut stacks: Vec<Vec<char>> = vec![Vec::<char>::new(); stacks_num];
        for crate_level in crates_rev_iter {
            let crate_level = crate_level.clone().replace("    ", " [-] ");
            for (i, cr) in crate_level.split_whitespace().enumerate() {
                let tag = cr.chars().nth(1).unwrap();
                if tag != '-' { stacks[i].push(tag); }
            }

        }

        for inst in instructions.lines() {
            let inst = inst
                .replace("move ", "")
                .replace("from ", "")
                .replace("to" ,   "");
            let params = inst
                .split_whitespace()
                .map(|x| str::parse::<usize>(x).unwrap())
                .collect::<Vec<usize>>();
            mv(params[0],params[1], params[2], &mut stacks);

        }
        get_top(&stacks)
    }
}

fn mv(n: usize, from: usize, to: usize, stacks: &mut Vec<Vec<char>>){
    let to_copy = stacks[from-1].as_slice()[stacks[from-1].len()-n..].to_vec();
    stacks[to-1].extend(to_copy);
    let to_len = stacks[from-1].len()-n;
    stacks[from-1].truncate(to_len);

}

fn get_top(stacks: &Vec<Vec<char>>) -> String {
    stacks
        .iter()
        .map(|x| x.last().unwrap())
        .collect()
}
