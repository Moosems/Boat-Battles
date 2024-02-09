// board.rs

use crate::ships;

const BOARDWIDTH: i32 = 15;
const BOARDHEIGHT: i32 = 16;

pub struct Board {
    pub width: i32,
    pub height: i32,
    pub grid: Vec<Vec<Option<ships::Ships>>>,
}

impl Board {
    pub fn new() -> Board {
        // Create a grid of None
        let mut grid = Vec::new();
        for _ in 0..BOARDHEIGHT {
            let mut row = Vec::new();
            for _ in 0..BOARDWIDTH {
                row.push(None);
            }
            grid.push(row);
        }
        Board {
            width: BOARDWIDTH,
            height: BOARDHEIGHT,
            grid,
        }
    }
}
