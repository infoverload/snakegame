use crate::lib::types::{Cell, SnakeHead, Grid};

// initialize the snake as a SnakeHead 
pub fn snake_init() -> SnakeHead {

    let snakeColor = Cell {
        red: 35_u8,
        green: 15_u8,
        blue: 13_u8,
    });

    SnakeHead { 
        row: 5,
        column: 5,
        cell: snakeColor,   
    } 
}

pub fn snake_move(snakeHead: &mut SnakeHead, row_direction: &u32, col_direction: &u32) {
    snakeHead.row = row_direction;
    snakeHead.column = column_direction;
}


//takes ownership of the grid, changes the color of the square, where the current SnakeHead is located. The grid is then the return value.
pub fn change_grid(snakeHead: &mut SnakeHead, renderer: &mut Canvas<Window>, grid_data: &Grid) -> Grid {


}

