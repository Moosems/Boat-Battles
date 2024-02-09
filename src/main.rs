// main.rs
#![allow(dead_code)]
#![allow(unused_variables)]

// Create custom modules
mod board;
mod board_manager;
mod player;
mod ships;

// Use needed modules
use board_manager::BoardManager;
use ships::{
    BLACK_SHIP_COUNT, CAPITAL_SHIP_COUNT, GOLD_SHIP_COUNT, RED_SHIP_COUNT, WHITE_SHIP_COUNT,
};

// Use the standard library
use std::io;
use std::process::exit;

// Import rand for random number generation
use rand::Rng;

// Create constants
const ACTION_POINTS: i32 = 3;

fn main() {
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    let mut board_manager = BoardManager::new();

    // Intro text
    println!("Boat Battles!");
    println!("Start Game? (y/n)");

    // Determine if the game should start
    let mut start_game = String::new();
    io::stdin()
        .read_line(&mut start_game)
        .expect("Failed to read line");
    let start_game = start_game.trim();

    // Start game or exit
    match start_game {
        "y" => {}
        _ => {
            println!("Exiting game");
            exit(0);
        }
    }

    // Create players
    let mut player1 = player::Player::new();
    let mut player2 = player::Player::new();
    println!("Players! Choose between yourselves who is Player 1 and Player 2!");

    // Pick who goes first
    let mut rng = rand::thread_rng();
    let mut current_player: i32 = rng.gen_range(1..=2);
    println!("Player {} goes first!", current_player);

    // Set up the ship pick phase
    let mut white_ships_left: i32 = WHITE_SHIP_COUNT;
    let mut black_ships_left: i32 = BLACK_SHIP_COUNT;
    let mut red_ships_left: i32 = RED_SHIP_COUNT;
    let mut gold_ships_left: i32 = GOLD_SHIP_COUNT;
    let mut capital_ships_left: i32 = CAPITAL_SHIP_COUNT;
    let mut ships_left: i32 =
        WHITE_SHIP_COUNT + BLACK_SHIP_COUNT + RED_SHIP_COUNT + GOLD_SHIP_COUNT + CAPITAL_SHIP_COUNT;

    // Create a loop to pick ships
    let mut skip_player: (bool, i32) = (false, 0);
    println!("Time to pick ships!");
    loop {
        // Determine player from current_player
        let player = if current_player == 1 {
            &mut player1
        } else {
            &mut player2
        };

        // Ensure that there are ships left otherwise break
        if ships_left == 0 {
            break;
        }

        // Check if the player should be skipped
        if skip_player.0 && skip_player.1 == current_player {
            current_player = if current_player == 1 { 2 } else { 1 };
            continue;
        }

        // Ensure that the player has draw points
        if player.draw_remaining <= 0 {
            println!("Player {} has no more draw points!", current_player);
            skip_player = (true, current_player);
            current_player = if current_player == 1 { 2 } else { 1 };
            continue;
        }

        // Print out the ships left
        println!("White Ships left: {}", white_ships_left);
        println!("Black Ships left: {}", black_ships_left);
        println!("Red Ships left: {}", red_ships_left);
        println!("Gold Ships left: {}", gold_ships_left);
        println!("Capital Ships left: {}", capital_ships_left);

        // Let the player pick a ship
        println!("Player {}, pick a ship! (W/B/R/G/C)", current_player);
        let mut ship_pick = String::new();
        io::stdin()
            .read_line(&mut ship_pick)
            .expect("Failed to read line");
        let ship_pick = ship_pick.trim();

        // Check if the ship is available
        match ship_pick {
            "W" => {
                // Check if there are any white ships left
                if white_ships_left == 0 {
                    println!("No more white ships left!");
                    continue;
                }
                // We can be sure that there is at least one draw left
                white_ships_left -= 1;
                ships_left -= 1;
                player.add_ship(ships::Ships::white_ship());
                // Swap players
                current_player = if current_player == 1 { 2 } else { 1 };
            }
            "B" => {
                // Check if there are any black ships left
                if black_ships_left == 0 {
                    println!("No more black ships left!");
                    continue;
                }
                // Check that there are enough draw points
                if player.draw_remaining < ships::Ships::black_ship().draw_cost() {
                    println!("Not enough draw points for any ship above White Ship!");
                    continue;
                }
                black_ships_left -= 1;
                ships_left -= 1;
                player.add_ship(ships::Ships::black_ship());
                // Swap players
                current_player = if current_player == 1 { 2 } else { 1 };
            }
            "R" => {
                // Check if there are any red ships left
                if red_ships_left == 0 {
                    println!("No more red ships left!");
                    continue;
                }
                // Check that there are enough draw points
                if player.draw_remaining < ships::Ships::red_ship().draw_cost() {
                    println!("Not enough draw points for Red Ship!");
                    continue;
                }
                red_ships_left -= 1;
                ships_left -= 1;
                player.add_ship(ships::Ships::red_ship());
                // Swap players
                current_player = if current_player == 1 { 2 } else { 1 };
            }
            "G" => {
                // Check if there are any gold ships left
                if gold_ships_left == 0 {
                    println!("No more gold ships left!");
                    continue;
                }
                // Check that there are enough draw points
                if player.draw_remaining < ships::Ships::gold_ship().draw_cost() {
                    println!("Not enough draw points for Gold Ship!");
                    continue;
                }
                gold_ships_left -= 1;
                ships_left -= 1;
                player.add_ship(ships::Ships::gold_ship());
                // Swap players
                current_player = if current_player == 1 { 2 } else { 1 };
            }
            "C" => {
                // Check if there are any capital ships left
                if capital_ships_left == 0 {
                    println!("No more capital ships left!");
                    continue;
                }
                // Check that there are enough draw points
                if player.draw_remaining < ships::Ships::capital_ship().draw_cost() {
                    println!("Not enough draw points for Capital Ship!");
                    continue;
                }
                capital_ships_left -= 1;
                ships_left -= 1;
                player.add_ship(ships::Ships::capital_ship());
                // Swap players
                current_player = if current_player == 1 { 2 } else { 1 };
            }
            _ => {
                println!("Invalid ship pick!");
                continue;
            }
        }
    }

    // Set up the game board
}
