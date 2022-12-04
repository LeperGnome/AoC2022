use crate::tasks::Task;
use std::str::FromStr;

pub struct TDay {}

impl Task for TDay { 
    fn compute(&self) -> String {
        let data = include_str!("data.txt"); 
        TDay::calculate_overlaping(&data).to_string()
    }
}

impl TDay { 
    fn calculate_overlaping(data: &str) -> u32 {
        let mut cnt: u32 = 0;
        for line in data.lines() {
            let (p1, p2) = line
                .split_once(',')
                .expect("Not for 2 elves");
            let range1 = p1.parse::<Range>().unwrap();
            let range2 = p2.parse::<Range>().unwrap();

            // one fully contains the other
            if (range1.start <= range2.start && range1.end >= range2.end) ||
               (range2.start <= range1.start && range2.end >= range1.end) {
                   cnt += 1;
            }
            // overlaping
            else if (range1.end >= range2.start && range1.start <= range2.end) ||
                    (range2.end >= range1.start && range2.start <= range1.end) {
                println!("{:?}, {:?}", range1, range2);
            }
        }
        return cnt;
    }
}

// just for convenience
#[derive(Debug)]
struct Range { 
    start: u32,
    end: u32,
}

impl FromStr for Range {
    type Err = std::str::Utf8Error;  // i'm lazy

    fn from_str(s: &str) -> Result<Range, std::str::Utf8Error> {
        let (l, r) = s.split_once('-').expect("Not a range");
        Ok(Range {
            start: l.parse().unwrap(),
            end: r.parse().unwrap(),
        })
    }
}
