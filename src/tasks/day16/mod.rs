use crate::tasks::Task;

use std::collections::HashSet;

pub struct TDay {}

#[derive(Eq, Hash, PartialEq, Debug)]
struct Valve {
    tag: String,
    rate: usize,
    open: bool,
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct Path<'a> {
    from: &'a Valve,
    to: &'a Valve,
    duration: usize,
}

#[derive(Debug)]
struct Cave<'a> {
    valves: HashSet<Valve>,
    paths:  HashSet<Path<'a>>,
}

impl<'a> Cave<'a> {
    fn insert_paths<'b: 'a>(&'b mut self, paths: &HashSet<(&str, &str)>) {
        for path in paths {
            let from = self
                .valves
                .iter()
                .find(|x| x.tag == path.0)
                .unwrap();
            let to   = self.valves.iter().find(|x| x.tag == path.1).unwrap();
            self.paths.insert(
                Path {
                    from,
                    to,
                    duration: 1
                }
            );
        }
    }
}

// start at AA
// for each other open valve calculate profit (valve rate * (current time - path duration))
//      if no path is found -> calculate shortest, add to known
// set current to next valve
// add profit to total
//
// repeat until 30 min or all valves closed

impl Task for TDay { 
    fn compute(&self) -> String {
        let data = include_str!("data.txt"); 

        let mut cave = Cave { paths: HashSet::new(), valves: HashSet::new() };
        let mut known_paths: HashSet<(&str, &str)> = HashSet::new();

        for line in data.lines() {
            let (l, r) = line.split_once(';').unwrap();
            let (cur, rate) = l.split_once(" has").unwrap();

            let cur = cur.split_once(' ').unwrap().1;
            let rate = rate.split_once('=').unwrap().1;

            cave.valves.insert(Valve{ tag: cur.to_string(), rate: rate.parse().unwrap(), open: false });

            let to: Vec<&str> = r.split_once("valves ").unwrap().1.split(", ").collect();
            for dest in to {
                known_paths.insert((cur.min(dest), cur.max(dest)));
            }
        }

        cave.insert_paths(&known_paths);

        dbg!(&cave);

        TDay::cute_elephants()
            .expect("something should be here")
            .to_string()
    }
}

impl TDay { 
    fn cute_elephants() -> Option<usize> {
        None
    }
}
