use crate::tasks::Task;
use std::collections::{VecDeque, HashMap};


pub struct TDay {}

impl Task for TDay { 
    fn compute(&self) -> String {
        let data = include_str!("data.txt"); 
        TDay::monkey_business(&data, 10_000).expect("something should be here").to_string()
    }
}

impl TDay { 
    fn monkey_business(data: &str, rounds: i32) -> Option<usize> {
        let mut common_den = 1;
        let mut monkeys: Vec<Monkey> = data
            .split("\n\n")
            .map(|m_data| {
                let mut stats = m_data.lines().skip(1);

                let items: VecDeque<u128> = VecDeque::from_iter(
                    stats
                    .next().clone().unwrap().split_once(": ").unwrap().1.split(", ")
                    .flat_map(str::parse::<u128>)
                );

                let oper: Box<dyn FnMut(&u128) -> u128>;
                let oper_parts = stats.next().unwrap().split_once("= ").unwrap().1
                    .split_whitespace().collect::<Vec<&str>>();  // -> [old , *, 6]
                let y = oper_parts[2].to_string().clone();
                if oper_parts[1] == "*" {
                    if y == "old" {
                        oper = Box::new(|x| x * x);
                    } else {
                        oper = Box::new(move |x| x * y.parse::<u128>().unwrap());
                    }
                } else {  // + 
                    if y == "old" {
                        oper = Box::new(|x| x + x);
                    } else {
                        oper = Box::new(move |x| x + y.parse::<u128>().unwrap());
                    }
                }

                let denom = stats.next().unwrap().split_once("by ").unwrap().1
                    .parse::<u128>()
                    .unwrap();
                common_den *= denom;
                let true_next = stats.next().unwrap().split_whitespace().last().unwrap()
                    .parse::<u128>()
                    .unwrap();
                let false_next = stats.next().unwrap().split_whitespace().last().unwrap()
                    .parse::<u128>()
                    .unwrap();

                Monkey {
                    items, 
                    operation: oper,
                    test: Box::new(move |x| (x % denom.clone()) == 0),
                    true_next,
                    false_next,
                }

            })
            .collect();

        let mut scores: HashMap<usize, usize> = HashMap::new();
        for _ in 1..=rounds{
            for midx in 0..monkeys.len() {
                let monkey = &monkeys[midx];
                let trn = monkey.true_next;
                let fan = monkey.false_next;

                let [current, trn, fan] = monkeys.get_many_mut([midx, trn as usize, fan as usize]).unwrap();
                let inspected = current.take_turn(trn, fan, common_den);
                *scores.entry(midx).or_insert(0) += inspected;
            }
        }
        let mut scores_v = scores.values().cloned().collect::<Vec<usize>>();
        scores_v.sort();
        scores_v.reverse();

        Some(scores_v.into_iter().take(2).reduce(|acc, x| acc * x).unwrap())
    }
}


struct Monkey {
    items: VecDeque<u128>,
    operation: Box<dyn FnMut(&u128) -> u128>,
    test: Box<dyn FnMut(&u128) -> bool>,
    true_next: u128,
    false_next: u128,
}

impl Monkey {
    fn take_turn(&mut self, true_next: &mut Monkey, false_next: &mut Monkey, den: u128) -> usize{
        let mut inspected = 0;
        for _ in 0..self.items.len() {
            inspected += 1;
            let item = self.items.pop_front().unwrap();

            let oper_res = (self.operation)(&item);
            let cur_worry = oper_res % den;
            if (self.test)(&cur_worry) {
                true_next.items.push_back(cur_worry);
            } else {
                false_next.items.push_back(cur_worry);
            }
        }
        inspected
    }
}
