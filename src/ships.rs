// ships.rs

pub const WHITE_SHIP_COUNT: i32 = 14;
pub const BLACK_SHIP_COUNT: i32 = 14;
pub const RED_SHIP_COUNT: i32 = 1;
pub const GOLD_SHIP_COUNT: i32 = 1;
pub const CAPITAL_SHIP_COUNT: i32 = 2;

pub struct Ship {
    pub hp: i32,
    pub damage: i32,
    pub move_range: i32,
    pub firing_range: i32,
    pub draw_cost: i32,
    pub alive: bool,
    pub name: String,
    pub short_name: String,
}

pub enum Ships {
    WhiteShip(Ship),
    BlackShip(Ship),
    RedShip(Ship),
    GoldShip(Ship),
    CapitalShip(Ship),
}

impl Ships {
    // Constructors
    pub fn white_ship() -> Ships {
        Ships::WhiteShip(Ship {
            hp: 2,
            damage: 1,
            move_range: 1,
            firing_range: 1,
            draw_cost: 1,
            alive: true,
            name: String::from("White"),
            short_name: String::from("W"),
        })
    }
    pub fn black_ship() -> Ships {
        Ships::BlackShip(Ship {
            hp: 4,
            damage: 2,
            move_range: 2,
            firing_range: 3,
            draw_cost: 2,
            alive: true,
            name: String::from("Black"),
            short_name: String::from("B"),
        })
    }
    pub fn red_ship() -> Ships {
        Ships::RedShip(Ship {
            hp: 3,
            damage: 2,
            move_range: 7,
            firing_range: 2,
            draw_cost: 3,
            alive: true,
            name: String::from("Red"),
            short_name: String::from("R"),
        })
    }
    pub fn gold_ship() -> Ships {
        Ships::GoldShip(Ship {
            hp: 3,
            damage: 3,
            move_range: 1,
            firing_range: 10,
            draw_cost: 3,
            alive: true,
            name: String::from("Gold"),
            short_name: String::from("G"),
        })
    }
    pub fn capital_ship() -> Ships {
        Ships::CapitalShip(Ship {
            hp: 10,
            damage: 5,
            move_range: 1,
            firing_range: 10,
            draw_cost: 5,
            alive: true,
            name: String::from("Capital"),
            short_name: String::from("C"),
        })
    }

    // Getters
    pub fn hp(&self) -> i32 {
        match self {
            Ships::WhiteShip(ship) => ship.hp,
            Ships::BlackShip(ship) => ship.hp,
            Ships::RedShip(ship) => ship.hp,
            Ships::GoldShip(ship) => ship.hp,
            Ships::CapitalShip(ship) => ship.hp,
        }
    }
    pub fn damage(&self) -> i32 {
        match self {
            Ships::WhiteShip(ship) => ship.damage,
            Ships::BlackShip(ship) => ship.damage,
            Ships::RedShip(ship) => ship.damage,
            Ships::GoldShip(ship) => ship.damage,
            Ships::CapitalShip(ship) => ship.damage,
        }
    }
    pub fn move_range(&self) -> i32 {
        match self {
            Ships::WhiteShip(ship) => ship.move_range,
            Ships::BlackShip(ship) => ship.move_range,
            Ships::RedShip(ship) => ship.move_range,
            Ships::GoldShip(ship) => ship.move_range,
            Ships::CapitalShip(ship) => ship.move_range,
        }
    }
    pub fn firing_range(&self) -> i32 {
        match self {
            Ships::WhiteShip(ship) => ship.firing_range,
            Ships::BlackShip(ship) => ship.firing_range,
            Ships::RedShip(ship) => ship.firing_range,
            Ships::GoldShip(ship) => ship.firing_range,
            Ships::CapitalShip(ship) => ship.firing_range,
        }
    }
    pub fn draw_cost(&self) -> i32 {
        match self {
            Ships::WhiteShip(ship) => ship.draw_cost,
            Ships::BlackShip(ship) => ship.draw_cost,
            Ships::RedShip(ship) => ship.draw_cost,
            Ships::GoldShip(ship) => ship.draw_cost,
            Ships::CapitalShip(ship) => ship.draw_cost,
        }
    }
    pub fn alive(&self) -> bool {
        match self {
            Ships::WhiteShip(ship) => ship.alive,
            Ships::BlackShip(ship) => ship.alive,
            Ships::RedShip(ship) => ship.alive,
            Ships::GoldShip(ship) => ship.alive,
            Ships::CapitalShip(ship) => ship.alive,
        }
    }
    pub fn name(&self) -> String {
        match self {
            Ships::WhiteShip(ship) => ship.name.clone(),
            Ships::BlackShip(ship) => ship.name.clone(),
            Ships::RedShip(ship) => ship.name.clone(),
            Ships::GoldShip(ship) => ship.name.clone(),
            Ships::CapitalShip(ship) => ship.name.clone(),
        }
    }
    pub fn short_name(&self) -> String {
        match self {
            Ships::WhiteShip(ship) => ship.short_name.clone(),
            Ships::BlackShip(ship) => ship.short_name.clone(),
            Ships::RedShip(ship) => ship.short_name.clone(),
            Ships::GoldShip(ship) => ship.short_name.clone(),
            Ships::CapitalShip(ship) => ship.short_name.clone(),
        }
    }
}
