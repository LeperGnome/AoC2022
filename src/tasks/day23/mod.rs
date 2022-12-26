use crate::tasks::Task;
use std::collections::{HashSet, HashMap};

pub struct TDay {}

impl Task for TDay { 
    fn compute(&self) -> String {
        let data = include_str!("data.txt"); 

        let mut elves = HashSet::new();

        for (y, l) in data.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                match c {
                    '#' => elves.insert((x as i32, y as i32)),
                    _ => continue,
                };
            }
        }

        TDay::unstable_diffusion(elves)
            .expect("something should be here")
            .to_string()
    }
}

impl TDay { 
    fn unstable_diffusion(mut elves: HashSet<(i32, i32)>) -> Option<usize> {
        let mut directions = vec![
            [( 0, -1), ( 1, -1), (-1 ,-1)],  // north
            [( 0,  1), ( 1,  1), (-1 , 1)],  // south
            [(-1,  0), (-1,  1), (-1 ,-1)],  // west
            [( 1,  0), ( 1, -1), ( 1 , 1)],  // east
        ];

        let mut n = 0;
        loop {
            n += 1;
            let mut proposals: HashMap<(i32, i32), ((i32, i32), usize)> = HashMap::new();
            //                          proposal     first elf  elf n

            // 1. for each elf check if he needs to go somewhere
            // 2. save his tile proposal
            for elf in elves.iter() {
                if let Some(prop) = get_proposal(&elf, &elves, &directions) {
                    proposals.entry(prop).or_insert((*elf, 0)).1 += 1;
                }
            }

            if proposals.len() == 0 {
                break;
            }

            // 3. move each elf to his proposal if proposal count is 1
            proposals
                .iter()
                .filter(|(_, (_, n))|  *n == 1)
                .for_each(|(p, (e, _))| {
                    elves.remove(e);
                    elves.insert(*p);
                });

            // 4. rotate direction vector
            directions.rotate_left(1);
        }
        print_map(&elves);
        return Some(n);
    }
}

fn get_proposal(
    elf: &(i32, i32),
    elves: &HashSet<(i32, i32)>,
    directions: &Vec<[(i32, i32);3]>
) -> Option<(i32, i32)> {
    let mut found = false;

    for n in [
        ( 0,  1), // north 
        ( 1,  1), 
        (-1 , 1),
        ( 0, -1), // south
        ( 1, -1), 
        (-1 ,-1),
        (-1,  0), // west 
        ( 1,  0), // east
    ].iter() {
        if elves.contains(&(elf.0 + n.0, elf.1 + n.1)) {
            found = true;
            break;
        }
    }

    if !found {
        return None
    }

    for dir in directions {
        let mut should_propose = true;
        for cell in dir {
            if elves.contains(&(elf.0 + cell.0, elf.1 + cell.1)) {
                should_propose = false;
                break;
            }
        }
        if should_propose {
            return Some((elf.0 + dir[0].0, elf.1 + dir[0].1));
        }
    }
    return None;
}

fn print_map(elves: &HashSet<(i32, i32)>) {
    let max_x = elves.iter().max_by_key(|x| x.0).unwrap().0;
    let max_y = elves.iter().max_by_key(|x| x.1).unwrap().1;

    let min_x = elves.iter().min_by_key(|x| x.0).unwrap().0;
    let min_y = elves.iter().min_by_key(|x| x.1).unwrap().1;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if !elves.contains(&(x,y)) {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
    }
    println!();
}
