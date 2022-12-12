use crate::tasks::Task;
use std::collections::VecDeque;

#[derive(Debug)]
struct Point {
    row: usize,
    col: usize,
    value: char,
    distance: usize,
}

type Heatmap = Vec<Vec<char>>;
type VisitedMap = Vec<Vec<bool>>;


pub struct TDay {}

impl Task for TDay { 
    fn compute(&self) -> String {
        let data = include_str!("data.txt"); 

        let mut heatmap: Heatmap = vec![];
        for row in data.lines(){
            let mut tmp = vec![];
            for c in row.chars() {
                tmp.push(c);
            }
            heatmap.push(tmp);
        }
        TDay::hill_climb(heatmap).expect("something should be here").to_string()
    }
}

impl TDay { 
    fn hill_climb(heatmap: Heatmap) -> Option<usize> {
        let mut visited: VisitedMap = vec![];

        let mut lowest_positions = vec![];

        // find starting point & initializing visited
        for (ri, row) in heatmap.iter().enumerate() {
            let mut tmp = vec![];
            for (ci, el) in row.iter().enumerate() {
                if *el == 'a' {
                    let current_position = Point { 
                        row: ri,
                        col: ci,
                        value: 'a',
                        distance: 0,
                    };
                    lowest_positions.push(current_position)
                } 
                tmp.push(false);
            }
            visited.push(tmp);
        }

        let mut possible_path_lengths = vec![];

        for lp in lowest_positions {
            let mut visited = visited.clone();
            visited[lp.row][lp.col] = true;

            let mut current_position = lp;

            let mut queue: VecDeque<Point> = VecDeque::new();
            queue.push_back(current_position);

            // BFS
            while queue.len() != 0 {
                current_position = queue.pop_front().unwrap();

                if current_position.value == 'E' {
                    possible_path_lengths.push(current_position.distance);
                    break;
                }

                for dir in ['>', '<', '^', 'v'] {
                    make_move(dir, &mut queue, &current_position, &heatmap, &mut visited);
                }
            }
        }
        possible_path_lengths.into_iter().min()
    }
}

fn make_move(
    dir: char,
    q: &mut VecDeque<Point>,
    current_position: &Point,
    heatmap: &Heatmap,
    visited: &mut VisitedMap
) {
    let mut new_row = current_position.row as i32;
    let mut new_col = current_position.col as i32;

    if dir == '>' { new_col += 1 }
    if dir == '<' { new_col -= 1 }
    if dir == 'v' { new_row += 1 }
    if dir == '^' { new_row -= 1 }

    if is_valid_move(
        &current_position, 
        new_row,
        new_col,
        &heatmap,
        &visited
    ) {
        let new_row = new_row as usize;
        let new_col = new_col as usize;
        let new_val = heatmap[new_row][new_col];

        q.push_back(Point {
            row: new_row,
            col: new_col,
            value: new_val,
            distance: current_position.distance + 1,
        });
        visited[new_row][new_col] = true;
    }
}

fn is_valid_move(
    from_point: &Point,
    row: i32,
    col: i32,
    heatmap: &Heatmap,
    visited: &VisitedMap
) -> bool {
    if row >= 0 && col >= 0 && row < heatmap.len() as i32 &&  col < heatmap[0].len() as i32 {
        let row = row as usize;
        let col = col as usize;

        let mut dest_val = heatmap[row][col];
        // by defenition, end has a hight of 'z'
        if dest_val == 'E' { dest_val = 'z' }

        return visited[row][col] == false && 
        dest_val as i32 - from_point.value as i32 <= 1
    } 
    false
}
