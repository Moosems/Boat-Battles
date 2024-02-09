// board_manager.rs
#![allow(dead_code)]
#![allow(unused_imports)]

use crate::board::Board;
use crate::ships::Ships;

pub struct BoardManager {
    pub board: Board,
}

impl BoardManager {
    pub fn new() -> BoardManager {
        BoardManager {
            board: Board::new(),
        }
    }
}
