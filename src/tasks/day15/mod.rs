use crate::tasks::Task;

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
    closest_beacon: Option<Beacon>,
}

impl Hash for Sensor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Sensor {
    fn view_radius(&self) -> i64 {
        (self.x - self.closest_beacon.unwrap().x).abs() +
        (self.y - self.closest_beacon.unwrap().y).abs()
    }

    fn project_view_on_line(&self, bitline: &mut Vec<bool>, dist_to_line: i64) {
        let view_left = self.view_radius() - dist_to_line;
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

    fn project_view(&self, bitmap: &mut Vec<Vec<bool>>) {
        let liney = self.y;
        for dy in 1..=self.view_radius() {
            dbg!(self.y + dy);
            if liney + dy < bitmap.len() as i64 {
                let line_up = &mut bitmap[(liney + dy) as usize];
                self.project_view_on_line(line_up, liney + dy);
            }
            dbg!(self.y - dy);
            if liney - dy >= 0 {
                let line_down = &mut bitmap[(liney - dy) as usize ];
                self.project_view_on_line(line_down, dy);
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
        let mut beacons = HashSet::new();

        for line in data.lines() {
            let (sp, bp) = line.split_once(':').unwrap();
            let beacon = bp.parse::<Beacon>().unwrap();
            let (sensor_x, sensor_y) = get_xy(sp);
            beacons.insert(beacon);
            let sensor = Sensor {
                x: sensor_x,
                y: sensor_y,
                closest_beacon: Some(beacon),
            };
            sensors.insert(sensor);
        }
        TDay::beacon_detection(sensors, beacons)
            .expect("something should be here")
            .to_string()
    }
}

impl TDay { 
    fn beacon_detection(sensors: HashSet<Sensor>, beacons: HashSet<Beacon>) -> Option<usize> {
        let minx_s = sensors.iter().min_by_key(|s| s.x).unwrap().x;
        let miny_s = sensors.iter().min_by_key(|s| s.y).unwrap().y;

        let maxx_s = sensors.iter().max_by_key(|s| s.x).unwrap().x;
        let maxy_s = sensors.iter().max_by_key(|s| s.y).unwrap().y;

        let minx_b = beacons.iter().min_by_key(|s| s.x).unwrap().x;
        let miny_b = beacons.iter().min_by_key(|s| s.y).unwrap().y;

        let maxx_b = beacons.iter().max_by_key(|s| s.x).unwrap().x;
        let maxy_b = beacons.iter().max_by_key(|s| s.y).unwrap().y;

        let minx = minx_s.min(minx_b);
        let maxx = maxx_s.max(maxx_b);

        let miny = miny_s.min(miny_b);
        let maxy = maxy_s.max(maxy_b);

        let scale_factor = 1;
        
        let mut bitmap = vec![];

        for _ in 0..(maxy*scale_factor) {
            let bitline = vec![false; (maxx*scale_factor - minx*scale_factor) as usize];
            bitmap.push(bitline);
        }

        for s in sensors.iter() {
            println!("{:?}", s);
            s.project_view(&mut bitmap);
        }

        for l in bitmap.iter() {
            for el in l.iter() {
                if *el {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        Some(1)
    }
}
