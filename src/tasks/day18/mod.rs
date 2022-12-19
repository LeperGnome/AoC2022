use crate::tasks::Task;
use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

pub struct TDay {}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Cube(i32, i32, i32);

impl FromStr for Cube {
    type Err = std::str::Utf8Error; 

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(',').map(str::parse::<i32>); 
        Ok(Self(
                it.next().unwrap().unwrap(), 
                it.next().unwrap().unwrap(),
                it.next().unwrap().unwrap(),
        ))
    }
}

impl Cube {
    fn is_near(&self, c: &Cube) -> bool{
        return ((self.0 - c.0).abs() == 1 && self.1 == c.1 && self.2 == c.2)
            || ((self.1 - c.1).abs() == 1 && self.0 == c.0 && self.2 == c.2)
            || ((self.2 - c.2).abs() == 1 && self.0 == c.0 && self.1 == c.1)
    }

    fn get_neighbours(&self) -> Vec<Cube> {
        [( 0,  0,  1),
         ( 0,  0, -1),
         ( 1,  0,  0),
         (-1,  0,  0),
         ( 0,  1,  0),
         ( 0, -1,  0)].iter()
         .map(|(dx, dy, dz)| Cube(self.0 + dx, self.1 + dy, self.2 + dz))
         .collect()
    }
}

fn get_figure_area(cubes: &HashSet<Cube>) -> usize {
    let max_area = cubes.len() * 6;
    let mut collisions = 0;

    for c1 in cubes.iter() {
        for c2 in cubes.iter() {
            if c1 == c2 { continue; }
            if c1.is_near(c2) {
                collisions += 1;
            }

        }
    }
    return max_area - collisions;
}


impl Task for TDay { 
    fn compute(&self) -> String {
        let data = include_str!("data.txt"); 

        TDay::brbr_lava(data)
            .expect("something should be here")
            .to_string()
    }
}

impl TDay { 
    fn brbr_lava(data: &str) -> Option<usize> {
        let mut minx = i32::MAX;
        let mut miny = i32::MAX;
        let mut minz = i32::MAX;

        let mut maxx = 0;
        let mut maxy = 0;
        let mut maxz = 0;

        let cubes = data
            .lines()
            .map(|x| { 
                let c = x.parse::<Cube>().unwrap();
                minx = minx.min(c.0);
                miny = miny.min(c.1);
                minz = minz.min(c.2);

                maxx = maxx.max(c.0);
                maxy = maxy.max(c.1);
                maxz = maxz.max(c.2);

                return c;
            })
            .collect::<HashSet<Cube>>();

        let figure_area = get_figure_area(&cubes);
        dbg!(&figure_area);

        let mut q = VecDeque::from([Cube(minx-1, miny-1, minz-1)]);
        let mut seen = HashSet::from([Cube(minx-1, miny-1, minz-1)]);
        let mut outer_area = 0;

        loop {
            if q.len() == 0 { break; }
            let current = q.pop_back().unwrap();

            for n in current.get_neighbours() {
                if seen.contains(&n) 
                || n.0 < minx-1 || n.0 >= maxx+2
                || n.1 < miny-1 || n.1 >= maxy+2
                || n.2 < minz-1 || n.2 >= maxz+2
                { continue; }

                if cubes.contains(&n) {
                    outer_area += 1;
                } else {
                    seen.insert(n);
                    q.push_front(n);
                }
            }
        }
        return Some(outer_area);
    }

}
