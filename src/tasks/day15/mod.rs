use crate::tasks::Task;

use std::mem::size_of_val;
use std::str::FromStr;
use std::hash::{Hash, Hasher};
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Beacon {
    x: i64,
    y: i64,
}

impl FromStr for Beacon{
    type Err = std::str::Utf8Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = get_xy(s);
        Ok(Self{x,y})
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Sensor {
    x: i64,
    y: i64,
    view_radius: i64,
}

impl Hash for Sensor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Sensor {
    fn project_view_on_line(&self, bitline: &mut Vec<bool>, dist_to_line: i64) {
        let view_left = self.view_radius - dist_to_line;
        let linex = self.x;
        if view_left > 0 {
            bitline[linex as usize] = true;
            for i in 1..=view_left {
                if linex + i < bitline.len() as i64 {bitline[(linex+i) as usize] = true;}
                if linex - i >= 0 {bitline[(linex-i) as usize] = true;}
            }
        } else if view_left == 0 {
            bitline[linex as usize] = true;
        }
    }

    fn project_view(&self, bitmap: &mut Vec<Vec<bool>>, offset: i64) {
        let liney = self.y - offset;

        for dy in 0..=self.view_radius {
            // println!("viewing distance {}", dy);
            for new_line in [liney + dy, liney - dy] {
                if new_line < bitmap.len() as i64 && new_line >= 0{
                    let line_up = &mut bitmap[new_line as usize];
                    self.project_view_on_line(line_up, dy);
                }
            }
        }
    }
}

fn get_xy(s: &str) -> (i64, i64) {
    let (wx, wy) = s.split_once(',').unwrap();
    (wx.split_once('=').unwrap().1.parse::<i64>().unwrap(),
     wy.split_once('=').unwrap().1.parse::<i64>().unwrap())
}

pub struct TDay {}

impl Task for TDay { 
    fn compute(&self) -> String {
        let data = include_str!("data.txt"); 

        let mut sensors = HashSet::new();

        for line in data.lines() {
            let (sp, bp) = line.split_once(':').unwrap();
            let beacon = bp.parse::<Beacon>().unwrap();
            let (sensor_x, sensor_y) = get_xy(sp);
            let sensor = Sensor {
                x: sensor_x,
                y: sensor_y,
                view_radius: (sensor_x - beacon.x).abs() +
                             (sensor_y - beacon.y).abs(),
            };
            sensors.insert(sensor);
        }
        TDay::beacon_detection(sensors)
            .expect("something should be here")
            .to_string()
    }
}

impl TDay { 
    fn beacon_detection(sensors: HashSet<Sensor>) -> Option<usize> {
        let bound: usize = 4000000;
        let chunks = 10_000;
        let chunksize = bound / chunks;

        for chidx in 0..chunks {
            let offset = chidx * chunksize;
            let mut bitmap = vec![];

            for _ in 0..=chunksize {
                let bitline = vec![false; bound+1];
                bitmap.push(bitline);
            }

            for (si, s) in sensors.iter().enumerate() {
                println!("viewing sensor #{}/{} with distance {}", si, sensors.len(), s.view_radius);

                s.project_view(&mut bitmap, offset as i64);
            }

            for (y, l) in bitmap.iter().enumerate() {
                for (x, el) in l.iter().enumerate() {
                    if !*el { 
                        println!("{}", size_of_val(&*bitmap));
                        return Some((x*4000000)+y+offset);
                    }
                }
            }
            println!("done with chunk #{}", chidx);
        }

        
        Some(1)
    }
}
