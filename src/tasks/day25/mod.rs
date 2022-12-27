use crate::tasks::Task;

pub struct TDay {}

impl Task for TDay { 
    fn compute(&self) -> String {
        let data = include_str!("data.txt"); 

        TDay::snifsnaf(data)
            .expect("something should be here")
            .to_string()
    }
}

impl TDay { 
    fn snifsnaf(data: &str) -> Option<String> {
        let s: i64 = data
            .lines()
            .map(|l| {
                l.chars()
                    .rev()
                    .enumerate()
                    .fold(0, |acc, (p, d)| acc + 5_i64.pow(p as u32) * match d {
                        '=' => -2,
                        '-' => -1,
                        '0' => 0,
                        '1' => 1,
                        '2' => 2,
                        _   => unreachable!(),
                    })
            })
            .sum();

        Some(convert_to_snafu(s))
    }
}

fn convert_to_snafu(mut number: i64) -> String {
    let mut result: Vec<i8> = vec![];
    let mut borrow = 0;

    loop {
        let mut rem = (number % 5) as i8;
        rem += borrow;
        number /= 5;

        if rem >= 3 {
            result.push(rem-5);
            borrow = 1;
        } else {
            result.push(rem);
            borrow = 0;
        }

        if number == 0 {
            break;
        }
    }

    result
        .into_iter()
        .rev()
        .map(|d| match d {
            -2 => '=',
            -1 => '-',
            _  => d.to_string().chars().next().unwrap(),
        })
        .collect::<String>()
}
