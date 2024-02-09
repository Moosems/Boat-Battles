// player.rs

use crate::ships::Ships;

pub struct Player {
    pub ships: Vec<Ships>,
    pub draw_remaining: i32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            ships: Vec::new(),
            draw_remaining: 29,
        }
    }
    pub fn add_ship(&mut self, ship: Ships) {
        // Remove the cost of the ship from the player's draw_remaining
        self.draw_remaining -= ship.draw_cost();
        self.ships.push(ship);
    }
}
