///
/// Conway
///
/// Library implements a set of helper functions for generating, updating and
/// managing a Conway's Game of Life implementation.
///

#[derive(Copy, Clone, PartialEq)]
pub enum STATE {
    ALIVE,
    DEAD,
}

#[derive(Copy, Clone)]
pub struct Cell {
    /// Struct containing a cell's state.

    /// x-coordinate of the given cell in grid-space.
    pub row_pos_: usize,
    
    /// y-coordinate of the given cell in grid-space.
    pub col_pos_: usize,

    /// Current state of the cell.
    pub state_: STATE,

    /// Number of ticks that a cell has been alive for.
    pub ticks_alive_: u32,
}

pub struct GridWorld {
    /// Struct containing entire game state.

    /// Number of generations that passed.
    pub ticks_: u32,

    /// Number of columns in grid set by the user.
    pub rows_: usize,

    /// Number of rows in the grid set by the user.
    pub cols_: usize,

    /// Cell elements in the grid (rows x cols)
    pub grid_: Vec<Vec<Cell>>,
}

impl GridWorld {
    /// Constructor
    pub fn new(num_rows: usize, num_cols: usize) -> GridWorld {
        let buffered_num_cols = num_cols + 2 as usize;
        let buffered_num_rows = num_rows + 2 as usize;
        let mut collection_of_cells: Vec<Vec<Cell>> = Vec::new();

        for row in 0..buffered_num_rows {
            let mut collection_of_row_cells: Vec<Cell> = Vec::new();
            for col in 0..buffered_num_cols {
                let mut state = STATE::DEAD;
                if(col == 2) {
                    state = STATE::ALIVE;
                }
                collection_of_row_cells.push(Cell {
                    row_pos_: row,
                    col_pos_: col,
                    state_: state,
                    ticks_alive_: 0,
                });
            }
            collection_of_cells.push(collection_of_row_cells);
        }

        return GridWorld {
            ticks_: 0,
            rows_: buffered_num_rows,
            cols_: buffered_num_cols,
            grid_: collection_of_cells,
        };
    } 
}

/// Returns the count live neighbors that a cell has.
pub fn get_live_neighbor_count(x_coord: usize, y_coord: usize, state: &GridWorld) -> usize {
    return vec![
        state.grid_[y_coord][x_coord - 1],
        state.grid_[y_coord][x_coord + 1],
        state.grid_[y_coord + 1][x_coord - 1],
        state.grid_[y_coord + 1][x_coord + 1],
        state.grid_[y_coord - 1][x_coord + 1],
        state.grid_[y_coord - 1][x_coord - 1],
        state.grid_[y_coord + 1][x_coord],
        state.grid_[y_coord - 1][x_coord],
    ]
    .iter()
    .filter(|&cell| cell.state_ == STATE::ALIVE)
    .count();
}

/// Applies to the rules of the game to determine whether the given cell is
/// alive or not.
fn get_updated_cell(cell: &Cell, state: &GridWorld) -> Cell {
    let is_cell_alive: bool = cell.state_ == STATE::ALIVE;
    let live_neighbor_count: usize = get_live_neighbor_count(cell.col_pos_, cell.row_pos_, state);

    let mut updated_cell_state = STATE::DEAD;
    let mut ticks_alive = cell.ticks_alive_;

    if is_cell_alive && (live_neighbor_count == 2 || live_neighbor_count == 3) {
        ticks_alive += 1;
        updated_cell_state = STATE::ALIVE;
    } else if !is_cell_alive && (live_neighbor_count == 3) {
        ticks_alive = 1;
        updated_cell_state = STATE::ALIVE;
    } else {
        ticks_alive = 0;
    }

    return Cell {
        row_pos_: cell.row_pos_,
        col_pos_: cell.col_pos_,
        state_: updated_cell_state,
        ticks_alive_: ticks_alive,
    };
}

/// Helper function for updating the state of the game.
pub fn update(state: &super::conway::GridWorld, gen: u32) -> super::conway::GridWorld {
    let mut updated_cells: Vec<Vec<Cell>> = state.grid_.clone();

    // Update the cells.
    for row in 1..(state.rows_ - 1) {
        for col in 1..(state.cols_ - 1) {
            // TODO: This should be changed, the API should take a subblock from grid containing
            // the game state i.e. get_updated_cell(get_subblock(state.grid_[row][col]))
            updated_cells[row][col] = get_updated_cell(&state.grid_[row][col], &state);
        }
    }

    return GridWorld {
        ticks_: gen,
        rows_: state.rows_,
        cols_: state.cols_,
        grid_: updated_cells,
    };
}
