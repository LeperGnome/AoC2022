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
            if range1.overlaps(&range2) { cnt += 1; }
        }
        return cnt;
    }
}

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

impl Range {
    fn overlaps(&self, r: &Range) -> bool {
        (self.end >= r.start && self.start <= r.end) ||
        (r.end >= self.start && r.start <= self.end)
    }
}
