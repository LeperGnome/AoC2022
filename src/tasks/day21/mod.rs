use crate::tasks::Task;
use std::collections::HashMap;
use std::str::FromStr;

pub struct TDay {}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Monkey {
    name: String,
    number: Option<usize>,
    depends_on: Option<(String, String)>,
    oper: Option<char>,
}

impl FromStr for Monkey {
    type Err = std::str::Utf8Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut number = None;
        let mut depends_on = None;
        let mut oper = None;
        let (name, r) = s.split_once(": ").unwrap();
        let name = name.to_string();
        let r_parts = r.split_whitespace().collect::<Vec<&str>>();

        if r_parts.len() == 1 {
            number = Some(r_parts.first().unwrap().parse::<usize>().unwrap());
        } else {
            depends_on = Some((r_parts[0].to_string(), r_parts[2].to_string()));
            oper = Some(r_parts[1].chars().next().unwrap());
        }

        Ok(Self {
            name,
            number,
            depends_on,
            oper,
        })
    }
}

impl Monkey {
    fn get_number (&self, monkeys: &HashMap<String, Monkey>) -> usize {
        match self.number {
            Some(num) => num,
            None => {
                let depends_names = self.depends_on.clone().unwrap();
                let values = (
                    monkeys.get(&depends_names.0).unwrap().get_number(&monkeys),
                    monkeys.get(&depends_names.1).unwrap().get_number(&monkeys),
                );
                let oper = self.oper.expect("If monkey has no number, it should have an operation!");

                match oper {
                    '+' => values.0 + values.1,
                    '-' => values.0 - values.1,
                    '*' => values.0 * values.1,
                    '/' => { 
                        values.0 / values.1 
                    },
                    _ => unreachable!(),
                }
            }
        }
    }
}


// root = 123 + ((12 * 3) + (5 * humn))

impl Task for TDay { 
    fn compute(&self) -> String {
        let data = include_str!("data.txt"); 

        TDay::monkeys_again(data)
            .expect("something should be here")
            .to_string()
    }
}

impl TDay { 
    fn monkeys_again(data: &str) -> Option<usize> {
        let mut monkeys = HashMap::from_iter(data
            .lines()
            .map(|x| {
                let m = x.parse::<Monkey>().unwrap();
                (m.name.clone(), m)
            })
        );

        let mut are_not_eq = true;

        // need to match 7012559479583
        // I binary-searched the estimate =)
        let mut cur_humn = 3665520865930;

        let root = monkeys.get("root").unwrap();
        let depends_names = root.depends_on.clone().unwrap();

        while are_not_eq {
            cur_humn += 1;
            monkeys.get_mut("humn").unwrap().number = Some(cur_humn);
            let values = (
                monkeys.get(&depends_names.0).unwrap().get_number(&monkeys),
                monkeys.get(&depends_names.1).unwrap().get_number(&monkeys),
            );
            dbg!(&values);
            are_not_eq = values.0 != values.1;

        }
        println!("It's a match!");
        Some(cur_humn)
    }
}
