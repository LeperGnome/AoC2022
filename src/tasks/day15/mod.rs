use crate::tasks::Task;

use std::str::FromStr;
use std::hash::Hash;
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

#[derive(PartialEq, Eq, Hash, Debug)]
struct Sensor {
    x: i64,
    y: i64,
    view_radius: i64,
}

impl Sensor {
    fn get_perimeter_points(&self) -> HashSet<(i64, i64)> {
        let mut points = HashSet::new();
        for i in 0..=self.view_radius+1 {
            points.insert(( i + self.x,   self.view_radius + 1 - i + self.y));
            points.insert((-i + self.x,   self.view_radius + 1 - i + self.y));
            points.insert((-i + self.x, -(self.view_radius + 1 - i) + self.y));
            points.insert(( i + self.x, -(self.view_radius + 1 - i) + self.y));
        }
        return points;
    }

    fn contains(&self, point: (i64, i64)) -> bool {
        ((self.x - point.0).abs() + (self.y - point.1).abs()) <= self.view_radius
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
    fn beacon_detection(sensors: HashSet<Sensor>) -> Option<i64> {
        for s1 in sensors.iter() {
            dbg!(&s1);
            let pp = s1.get_perimeter_points();
            'point: for p in pp.iter() {
                if p.0 < 0 || p.1 < 0 || p.1 > 4000000 || p.0 > 4000000 {
                    continue 'point;
                }
                for s2 in sensors.iter() {
                    if s2.contains(*p) {
                        continue 'point;
                    }
                }
                println!("Found point: {:?}", p);
                return Some(p.0*4000000 + p.1);
            }
        }
        None
    }
}
