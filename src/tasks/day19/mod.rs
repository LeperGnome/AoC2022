use crate::tasks::Task;
use std::collections::HashMap;


#[derive(Hash, Debug, Eq, PartialEq, Clone, Copy)]
enum ResourceType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

type Resources = HashMap<ResourceType, usize>;
type RobotsCosts = HashMap<ResourceType, HashMap<ResourceType, usize>>;

pub struct TDay {}

impl Task for TDay { 
    fn compute(&self) -> String {
        let data = include_str!("data.txt"); 

        TDay::minerals(data)
            .expect("something should be here")
            .to_string()
    }
}

fn build_robot(
    robots: &mut HashMap<ResourceType, usize>,
    robot_type: &ResourceType,
    resourcs: &mut Resources,
    robots_costs: &RobotsCosts,
) {
    if !can_build(robot_type, &resourcs, &robots_costs) { return; }

    for (needed_resource, quantity) in robots_costs.get(robot_type).unwrap() {
        *resourcs.get_mut(needed_resource).unwrap() -= *quantity;
    }
    *robots.get_mut(robot_type).unwrap() += 1;
}

fn can_build(robot_type: &ResourceType, resourcs: &Resources, robots_costs: &RobotsCosts) -> bool {
    for (needed_resource, quantity) in robots_costs.get(robot_type).unwrap() {
        if *resourcs.get(needed_resource).unwrap() < *quantity {
            return false;
        }
    }
    return true;
}

fn need_to_build(
    robot_type: &ResourceType,
    robots: &HashMap<ResourceType, usize>,
    robots_costs: &RobotsCosts
) -> bool {
    match robot_type {
        ResourceType::Geode => true,
        ResourceType::Obsidian => {
            *robots_costs
                .get(&ResourceType::Geode)
                .unwrap()
                .get(&ResourceType::Obsidian).unwrap() > *robots.get(&ResourceType::Obsidian).unwrap()
        },
        ResourceType::Clay => {
            *robots_costs
                .get(&ResourceType::Obsidian)
                .unwrap()
                .get(&ResourceType::Clay).unwrap() > *robots.get(&ResourceType::Clay).unwrap()
        },
        ResourceType::Ore => {
            let mut max_ore = 0;
            robots_costs.iter()
                .for_each(|(_, v)|{
                    if let Some(needed) = v.get(&ResourceType::Ore) {
                        max_ore = max_ore.max(*needed);
                    }
                });
            return max_ore > *robots.get(&ResourceType::Clay).unwrap();
        },
    }
}

fn update_resources(
    robots: &HashMap<ResourceType, usize>,
    resourcs: &mut Resources,
) {
   for (robot_type, count) in robots.iter() {
       *resourcs.get_mut(robot_type).unwrap() += *count;
   }
}


fn get_max_geodes_2(
    robots: HashMap<ResourceType, usize>,
    mut resourcs: Resources,
    robots_costs: &RobotsCosts,
    time_passed: usize,
    total_time: usize,
) -> usize {
    let mut already_built: HashMap<ResourceType, bool> = HashMap::from([
        (ResourceType::Geode, false),
        (ResourceType::Obsidian, false),
        (ResourceType::Clay, false),
        (ResourceType::Ore, false),
    ]);
    let mut score = *resourcs.get(&ResourceType::Geode).unwrap();

    for m in time_passed..total_time {
        for t in [
            ResourceType::Geode,
            ResourceType::Obsidian,
            ResourceType::Clay,
            ResourceType::Ore
        ].iter() {
            if need_to_build(t, &robots, &robots_costs) 
            && can_build(t, &resourcs, &robots_costs) 
            && !already_built.get(t).unwrap() 
            {
                let mut res_c = resourcs.clone();
                let mut rob_c = robots.clone();
                build_robot(&mut rob_c, t, &mut res_c, &robots_costs);
                update_resources(&robots, &mut res_c);

                score = get_max_geodes_2(rob_c, res_c, &robots_costs, m+1, total_time).max(score);
                *already_built.get_mut(t).unwrap() = true;
            }
        }
        update_resources(&robots, &mut resourcs);
    }
    score = score.max(*resourcs.get(&ResourceType::Geode).unwrap());
    return score;
}

impl TDay { 
    fn minerals(data: &str) -> Option<usize> {
        let mut blueprits = vec![];
        for line in data.lines() {
            let mut costs: RobotsCosts = HashMap::new();
            let (_, line) = line.split_once(": ").unwrap();
            for (idx, c) in line.split(". ").enumerate() {
                match idx {
                    0 => {
                        let v = c.split_whitespace().nth(4).unwrap().parse::<usize>().unwrap();
                        costs.insert(ResourceType::Ore, HashMap::from([(ResourceType::Ore, v)]));
                    },
                    1 => {
                        let v = c.split_whitespace().nth(4).unwrap().parse::<usize>().unwrap();
                        costs.insert(ResourceType::Clay, HashMap::from([(ResourceType::Ore, v)]));
                    },
                    2 => {
                        let v = c.split_whitespace().nth(4).unwrap().parse::<usize>().unwrap();
                        let t = c.split_whitespace().nth(7).unwrap().parse::<usize>().unwrap();
                        costs.insert(ResourceType::Obsidian, HashMap::from([
                            (ResourceType::Ore, v),
                            (ResourceType::Clay, t),
                        ]));
                    },
                    3 => {
                        let v = c.split_whitespace().nth(4).unwrap().parse::<usize>().unwrap();
                        let t = c.split_whitespace().nth(7).unwrap().parse::<usize>().unwrap();
                        costs.insert(ResourceType::Geode, HashMap::from([
                            (ResourceType::Ore, v),
                            (ResourceType::Obsidian, t),
                        ]));
                    },
                    _ => unreachable!(),
                }
            }
            blueprits.push(costs);
        }
        
        let mut results = vec![];

        for (i, robots_costs) in blueprits.iter().enumerate() {
            results.push(get_max_geodes_2(
                HashMap::from([
                    (ResourceType::Ore, 1),
                    (ResourceType::Clay, 0),
                    (ResourceType::Obsidian, 0),
                    (ResourceType::Geode, 0),
                ]),
                HashMap::from([
                    (ResourceType::Ore, 0),
                    (ResourceType::Clay, 0),
                    (ResourceType::Obsidian, 0),
                    (ResourceType::Geode, 0),
                ]),
                &robots_costs,
                0,
                32,
            ));
            println!("Done with bp #{i}");
        }

        dbg!(&results);

        Some(results
            .into_iter()
            .reduce(|acc, v| acc * v)
            .unwrap())
    }
}
