use std::fmt::Display;

use crate::{day09::Point2d, day12::Input};
use grid::Grid;
use itertools::Itertools;

const INPUT: &str = include_str!("../../input/12/input");
const EXAMPLE: &str = include_str!("../../input/12/example");

// basically a grid
// S = start = elevation a
// E = end = elevation z
// can only increase (/decrease?) by 1 elevation
// maybe store height as u8?

#[derive(Debug, Clone)]
pub struct HeightMap {
    pub grid: Grid<u8>,
    pub display_grid: Grid<char>,
    start_idx: usize,
    goal_idx: usize,
}

impl Display for HeightMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "grid:");
        for r in (0..self.display_grid.rows()) {
            //96
            let row = self.display_grid.iter_row(r).join(""); //.replace("c", "░");
            writeln!(f, "{}", row);
        }
        Ok(())
    }
}

impl HeightMap {
    pub fn new(
        grid: Grid<u8>,
        display_grid: Grid<char>,
        start_idx: usize,
        goal_idx: usize,
    ) -> Self {
        Self {
            grid,
            display_grid,
            start_idx,
            goal_idx,
        }
    }
    pub fn start(&self) -> Point2d {
        let grid_cols = self.grid.cols() as i32;
        let y = self.start_idx as i32 / grid_cols;
        let x = self.start_idx as i32 % grid_cols;

        Point2d { x, y }
    }

    pub fn end(&self) -> Point2d {
        let grid_cols = self.grid.cols() as i32;
        let y = self.goal_idx as i32 / grid_cols;
        let x = self.goal_idx as i32 % grid_cols;

        Point2d { x, y }
    }
}

pub fn read() -> Input {
    read_to_height_map(INPUT)
}

pub fn readex() -> Input {
    read_to_height_map(EXAMPLE)
}

fn read_to_height_map(input: &str) -> HeightMap {
    let cols = input.lines().next().unwrap().len();
    let mut start_idx = 0;
    let mut goal_idx = 0;
    let grid = Grid::from_vec(
        input
            .replace("\n", "")
            .bytes()
            .enumerate()
            .map(|(idx, v)| match (idx, v) {
                (_, 83) => {
                    start_idx = idx;
                    1
                }
                (_, 69) => {
                    goal_idx = idx;
                    27
                }
                (_, (97..=122)) => v - 96,
                (_, _) => panic!(),
            })
            .collect_vec(),
        cols,
    );
    let display_grid = Grid::from_vec(input.replace("\n", "").chars().collect_vec(), cols);

    HeightMap::new(grid, display_grid, start_idx, goal_idx)
}
