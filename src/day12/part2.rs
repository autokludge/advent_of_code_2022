use std::{
    collections::{BinaryHeap, HashMap},
    ops::Sub,
    thread,
    time::Duration,
    vec,
};

use grid::Grid;

use crate::{
    day09::Point2d,
    day12::{Input, Output},
};

use super::input::HeightMap;

// Sabqponm     [0, 1, 2, 17, 16, 15, 14, 13]
// abcryxxl     [1, 2, 3, 18, 25, 24, 24, 12]
// accszExk     [1, 3, 3, 19, 26, 27, 24, 11]
// acctuvwj     [1, 3, 3, 20, 21, 22, 23, 10]
// abdefghi     [1, 2, 4,  5,  6,  7,  8,  9]

// Construct graph for traversal?
// Lets learn how to do a [A*] like search.
// Seems that we could use 27 - node_value to
// calculate the lower bound distance to exit measurement?
// using grid movement, so all edges are 1 cost moves.

// u64 is fine for node distance rather than a NodeDistance Type or Option<u8>
// would need a seriously large grid to render infinity = 18_446_744_073_709_551_615u64 ineffective
#[derive(Eq)]
struct PriorityPoint {
    point: Point2d,
    f_score: u64,
}

impl PartialEq for PriorityPoint {
    fn eq(&self, other: &Self) -> bool {
        other.f_score.eq(&self.f_score)
    }
}

impl PartialOrd for PriorityPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.f_score.partial_cmp(&self.f_score)
    }
}

impl Ord for PriorityPoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.f_score.cmp(&self.f_score)
    }
}

// BinaryHeap<PriorityPoint>    open_set    (priority queue of open_points)
// HashMap<Point, Point>        came_from   (map points to previous)
// HashMap<Point, u64>          g_score     (map of points to current best known cheapest path to start)
type GScoreMap = HashMap<Point2d, u64>;

// S = coord(0,0) = 00 ie. top left corner
// E = coord(5,2) = 27

//        Connections:
// [ 00-01-02 17-16-15-14-13 ]
// [  |  |  |  |           | ]
// [ 01-02-03 18 25-24-24 12 ]
// [  |  |  |  |  |     |  | ]
// [ 01 03-03 19 26-27 24 11 ]
// [  |  |  |  |        |  | ]
// [ 01 03-03 20-21-22-23 10 ]
// [  |  |  |              | ]
// [ 01-02 04-05-06-07-08-09 ]

fn reconstruct_path(
    came_from: &HashMap<Point2d, Point2d>,
    current_point: &Point2d,
) -> Vec<Point2d> {
    let mut total_path = vec![current_point.clone()];
    let mut point = current_point;

    while let Some(p) = came_from.get(point) {
        point = p;
        total_path.push(p.clone());
    }

    total_path.reverse();
    total_path
}

fn get_valid_neighbours(grid: &Grid<u8>, pos: (i32, i32), goal: &Point2d) -> Vec<(Point2d, u64)> {
    // (x,y,value)
    let mut neighbours: Vec<(Point2d, u64)> = vec![];
    let (x, y) = pos.into();
    let address_shift: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let current_val = grid.get(y as usize, x as usize).unwrap();

    for (x_shift, y_shift) in address_shift {
        let n_y_unchecked = y as i32 + y_shift;
        let n_x_unchecked = x as i32 + x_shift;
        if n_y_unchecked < 0 || n_x_unchecked < 0 {
            continue;
        }
        let n_y = n_y_unchecked as usize;
        let n_x = n_x_unchecked as usize;

        if let Some(n_val) = grid.get(n_y, n_x) {
            // apparently elves can jump down cliffs
            // so either 1 up, or inf down
            if *n_val as i32 - *current_val as i32 <= 1 {
                let p = (n_x as i32, n_y as i32).into();

                let h = 0; // h = 0 makes this basically Djikstra
                neighbours.push((p, h));
            }
        }
    }
    neighbours
}

fn a_star(start: &Point2d, goal: &Point2d, input: HeightMap) -> Option<Vec<Point2d>> {
    let mut input = input.clone();
    let h_values: &Grid<u8> = &input.grid;
    let mut open_set: BinaryHeap<PriorityPoint> = BinaryHeap::new();
    let hv = h_values.get(start.y as usize, start.x as usize);
    let hvu = hv.unwrap();
    open_set.push(PriorityPoint {
        point: start.clone(),
        f_score: *h_values.get(start.y as usize, start.x as usize).unwrap() as u64, /* hardcoded 27, but need a function */
    });

    let mut came_from: HashMap<Point2d, Point2d> = HashMap::new();

    let mut g_score: HashMap<Point2d, u64> = HashMap::new();
    g_score.insert(start.clone(), 0);
    let mut cycle = 0;
    while let Some(current_node) = open_set.pop() {
        let current = &current_node.point;
        if current == goal {
            return Some(reconstruct_path(&came_from, current));
        }

        let cur_dist = *g_score.get(current).unwrap_or(&u64::MAX);

        let path = reconstruct_path(&came_from, current);
        // let g = input.display_grid.clone();
        // for point in path {
        //     input.display_grid[point.y as usize][point.x as usize] = input.display_grid
        //         [point.y as usize][point.x as usize]
        //         .to_uppercase()
        //         .next()
        //         .unwrap();
        // }
        //println!("{}", input);
        //input.display_grid = g;

        // find valid neighbours of current
        for (neighbour, h_neighbour) in get_valid_neighbours(h_values, (current.x, current.y), goal)
        {
            let dist = cur_dist + 1; // all edges are 1
            let neighbour_dist = *g_score.get(&neighbour).unwrap_or(&u64::MAX);
            if dist < neighbour_dist {
                // this path to the neighbour is the best one found so far!
                came_from.insert(neighbour.clone(), current.clone());
                g_score.insert(neighbour.clone(), dist);

                open_set.push(PriorityPoint {
                    point: neighbour.clone(),
                    f_score: dist + h_neighbour,
                })
            }
        }
    }
    // Open set consumed, goal never reached
    None
}

fn idx_to_point(idx: usize, cols: usize) -> Point2d {
    let grid_cols = cols as i32;
    let y = idx as i32 / grid_cols;
    let x = idx as i32 % grid_cols;

    Point2d { x, y }
}

pub fn solve(input: &Input) -> Output {
    println!("Part 1\n==========\nInput: \n {}", input);

    // it's fast enough, lets just try brute forcing the solution
    let cols = input.grid.cols();
    let starts: Vec<Point2d> = input
        .grid
        .iter()
        .enumerate()
        .filter(|(idx, &x)| x == 1)
        .map(|(idx, v)| idx_to_point(idx, cols))
        .collect();

    let mut shortest_path = u64::MAX;
    for start in starts {
        let goal = &input.end();
        if let Some(path) = a_star(&start, goal, input.clone()) {
            if (path.len() as u64) < shortest_path {
                shortest_path = path.len() as u64;
                println!("shortest path: {:?}", start);
            }
        }
    }

    Output::U64(shortest_path - 1)
}
