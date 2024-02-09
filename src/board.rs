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
    pub fn print_board(&self) {
        let mut rowcount: i32 = 0;
        for row in &self.grid {
            println!("  {}", "–".repeat(self.width as usize * 2 + 1));
            // let print_string =  if rowcount < 10 else
            let print_string = if rowcount < 10 {
                format!("{} ", rowcount)
            } else {
                format!("{}", rowcount)
            };
            print!("{}|", print_string);
            for cell in row {
                match cell {
                    Some(ships::Ships::WhiteShip(_)) => print!("W|"),
                    Some(ships::Ships::BlackShip(_)) => print!("B|"),
                    Some(ships::Ships::RedShip(_)) => print!("R|"),
                    Some(ships::Ships::GoldShip(_)) => print!("G|"),
                    Some(ships::Ships::CapitalShip(_)) => print!("C|"),
                    None => print!(" |"),
                }
            }
            println!();
            rowcount += 1;
        }
        println!("  {}", "–".repeat(self.width as usize * 2 + 1));
        print!("   ");
        for i in 0..self.width {
            print!("{}", (b'a' + i as u8) as char);
            if i < self.width - 1 {
                print!(" ");
            }
        }
        println!();
    }
}
