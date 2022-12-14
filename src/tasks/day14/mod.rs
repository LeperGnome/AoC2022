use crate::tasks::Task;

const ROW_LEN: usize = 1500;
const COL_LEN: usize = 1500;
const OFFSET: i32 = 250;
const START: (usize, usize) = (0, (500 + OFFSET) as usize);

type ScanMap = Vec<Vec<bool>>;

pub struct TDay {}

impl Task for TDay { 
    fn compute(&self) -> String {
        let data = include_str!("data.txt"); 

        let mut scan_map = vec![vec![false; ROW_LEN]; COL_LEN];
        scan_map[START.0][START.1] = true;

        for line in data.lines() {
            line
                .split(" -> ")
                .map(|x| {
                     let p = x.split_once(',').unwrap();
                     ((p.0.parse::<i32>().unwrap() + OFFSET) as usize, 
                      p.1.parse::<usize>().unwrap())
                })
                .reduce(|prev, x| {
                    if x.0 != prev.0 {
                        for (idx, el) in scan_map[x.1].iter_mut().enumerate() {
                            if idx >= x.0.min(prev.0) && idx <= x.0.max(prev.0) {
                                *el = true;
                            }
                        }
                    } else if x.1 != prev.1 {
                        for (idx, el) in scan_map.iter_mut().enumerate() {
                            if idx >= x.1.min(prev.1) && idx <= x.1.max(prev.1) {
                                el[x.0] = true;
                            }
                        }
                    } else {
                        panic!("Incorrect input");
                    }
                    return x;
                });
        }

        let mut to_remove = vec![];
        for (i, line) in scan_map.iter().rev().enumerate() {
            if line.iter().cloned().map(|x| x as u8).sum::<u8>() == 0 {
                to_remove.push(scan_map.len() - i -1 );
            } else {
                break;
            }
        }
        for i in to_remove {
            scan_map.remove(i);
        }
        scan_map.push(vec![false; ROW_LEN]);
        scan_map.push(vec![true; ROW_LEN]);

        TDay::sand_trap(scan_map).expect("something should be here").to_string()
    }
}

impl TDay { 
    fn sand_trap(mut scan_map: ScanMap) -> Option<usize> {
        let mut units = 0;
        'outer: loop {
            let bottom: Option<(usize, usize)> = find_bottom(&scan_map, START.1, 1);

            match bottom {
                Some(p) => {

                    let mut b = p;
                    let mut lp = (b.0, b.1 - 1);
                    let mut rp = (b.0, b.1 + 1);
                    let mut up = (b.0 - 1, b.1);
                    let mut l_free = !scan_map[lp.0][lp.1];
                    let mut r_free = !scan_map[rp.0][rp.1];
                    while l_free || r_free {
                        let new_b: Option<(usize, usize)>;
                        if l_free {
                            new_b = find_bottom(&scan_map, lp.1, lp.0);
                        } else {
                            new_b = find_bottom(&scan_map, rp.1, rp.0);
                        } 
                        match new_b { 
                            Some(x) => b = x,
                            None => break 'outer,
                        }

                        lp = (b.0, b.1 - 1);
                        rp = (b.0, b.1 + 1);
                        up = (b.0 - 1, b.1);
                        l_free = !scan_map[lp.0][lp.1];
                        r_free = !scan_map[rp.0][rp.1];
                    }
                    if up == START {
                        units += 1;
                        break 'outer;
                    }
                    scan_map[up.0][up.1] = true;
                },
                None    => break 'outer,
            }
            units += 1;
        }
        print_scan_map(&scan_map);
        Some(units)
    }
}

fn find_bottom(scan_map: &ScanMap, col: usize, from_row: usize) -> Option<(usize, usize)>{
    for (ri, r) in scan_map.iter().skip(from_row).enumerate() {
        if r[col] {
            return Some((ri + from_row, col));
        }
    }
    return None;
}

fn print_scan_map(scan_map: &ScanMap) {
    for (ri, row) in scan_map.iter().enumerate() {
        for (ci, col) in row.iter().enumerate() {
            if ri == START.0 && ci == START.1 {
                print!("+");
                continue;
            }
            if *col {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
