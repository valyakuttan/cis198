pub mod game;
mod ui;

extern crate rand;
extern crate rustc_serialize;

use std::env::args;
use std::fs::File;

use game::board::Board;
use game::player::Player;

fn main() {
    let mut args = args();
    if let Some(filename) = args.nth(1) {
        let mut file_handle = File::open(&filename)
            .expect(&format!("Unable to open file {}", filename));
        
        let board = Board::build_board(&mut file_handle).unwrap();

        let player = Player::new(board.spawn_location());

        ui::game_loop(player);
    } else {
        println!("Usage: ./main file_name.json | cargo run -- file_name.json");
    }
}
