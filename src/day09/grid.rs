#[derive(Clone)]
pub struct VecGrid<T> {
    pub grid: Vec<T>,
    pub grid_transposed: Vec<T>,
    pub cols: usize,
    pub rows: usize,
}

#[derive(PartialEq, Clone, Copy)]
pub struct VecGridCoordinate {
    pub col: usize,
    pub row: usize,
}

impl From<(usize, usize)> for VecGridCoordinate {
    fn from(tup: (usize, usize)) -> Self {
        Self {
            col: tup.0,
            row: tup.1,
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Display for VecGrid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "cols:{}  rows:{}", self.cols, self.rows);
        writeln!(f, "grid:");
        for row in self.grid.chunks(self.cols) {
            writeln!(f, "{:?}", row);
        }
        Ok(())
    }
}

impl<T: Copy> VecGrid<T> {
    pub fn new(grid: Vec<T>, cols: usize, rows: usize) -> Self {
        let grid_transposed = vec![];
        let mut s = Self {
            grid,
            grid_transposed,
            cols,
            rows,
        };
        s.calculate_transposed();
        s
    }

    //pub fn contains()

    fn calculate_transposed(&mut self) {
        let cols = self.cols;
        let rows = self.rows;
        let mut transposed_grid = vec![];
        for i in (1..=cols) {
            for j in (1..=rows) {
                transposed_grid.push(*self.cell(i, j));
            }
        }
        self.grid_transposed = transposed_grid;
    }

    pub fn cell(&self, col: usize, row: usize) -> &T {
        &self.grid[self.coord_to_idx((col, row).into())]
    }

    pub fn cell_transposed(&self, col: usize, row: usize) -> &T {
        &self.grid_transposed[self.coord_to_idx_transposed((col, row).into())]
    }

    pub fn set_cell(&mut self, coord: VecGridCoordinate, val: T) {
        let idx = self.coord_to_idx(coord);
        self.grid[idx] = val;
    }

    pub fn coord_to_idx(&self, coord: VecGridCoordinate) -> usize {
        assert!(coord.col <= self.cols);
        assert!(coord.row <= self.rows);
        let col_0_idx = coord.col - 1;
        let row_0_idx = coord.row - 1;
        (row_0_idx * self.cols) + col_0_idx
    }

    pub fn coord_to_idx_transposed(&self, coord: VecGridCoordinate) -> usize {
        assert!(coord.row <= self.cols);
        assert!(coord.col <= self.rows);
        let col_0_idx = coord.row - 1;
        let row_0_idx = coord.col - 1;
        (col_0_idx * self.rows) + row_0_idx
    }

    pub fn idx_to_cell_coord(&self, idx: usize) -> VecGridCoordinate {
        let col_0_idx = idx % self.cols;
        let row_0_idx = idx / self.cols;

        (col_0_idx + 1, row_0_idx + 1).into()
    }

    pub fn row(&self, row: usize) -> &[T] {
        let r_idx = self.coord_to_idx((1, row).into());
        let row_len = r_idx + self.cols;
        &self.grid[r_idx..row_len]
    }
    pub fn col(&self, col: usize) -> &[T] {
        let c_idx = self.coord_to_idx_transposed((1, col).into());
        let col_len = c_idx + self.rows;
        &self.grid_transposed[c_idx..col_len]
    }
}
