use crate::tasks::Task;

#[derive(Copy, Clone)]
struct Element {
    idx: usize,
    val: i64,
}

pub struct TDay {}

impl Task for TDay { 
    fn compute(&self) -> String {
        let data = include_str!("data.txt"); 

        TDay::calculate_coordinates(data)
            .expect("something should be here")
            .to_string()
    }
}

impl TDay { 
    fn calculate_coordinates(data: &str) -> Option<i64> {
        let init = data
            .lines()
            .enumerate()
            .map(|(idx, val)| Element { idx, val: val.parse::<i64>().unwrap() * 811589153 })
            .collect::<Vec<Element>>();
        let mut result = init.clone();

        for _ in 1..=10 {
            mix(&init, &mut result);
        }

        let zero_idx = result
            .iter()
            .position(|x| x.val == 0)
            .unwrap();

        Some([1000, 2000, 3000]
            .iter()
            .map(|x| {
                let el = result[(zero_idx + x) % init.len()].val;
                println!("{x}th: {}", &el);
                return el;
            })
            .sum())
    }
}

fn mix(initial: &Vec<Element>, result: &mut Vec<Element>) {
    for el in initial.iter() {
        let cur_idx = result
            .iter()
            .position(|x| x.idx == el.idx)
            .unwrap();
        let mut new_index = (cur_idx as i64 + el.val) % (result.len() as i64 - 1);
        if new_index <= 0 {
            new_index = result.len() as i64 + new_index - 1;
        }
        result.remove(cur_idx);
        result.insert(new_index as usize, *el);
    }
}
