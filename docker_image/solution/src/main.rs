mod board;
mod game_state;
mod grid_trait;
mod piece;
mod player;
mod strategy;

use std::io::{self, BufRead};

/// Main entry point for the Filler bot
/// Handles I/O communication with the game engine and coordinates gameplay
fn main() {
    // Establish single stdin connection for reliable I/O throughout the game
    // Using a single lock prevents the race conditions we encountered earlier
    let stdin = io::stdin();
    let mut input_lines = stdin.lock().lines();

    // Parse initial game setup
    // Format: "$$$ exec p1 : [bot_path]" and "Anfield HEIGHT WIDTH:"
    let player_assignment = input_lines.next().unwrap().unwrap();
    let board_dimensions = input_lines.next().unwrap().unwrap();

    // Initialize game state based on our player assignment
    let (our_player, opponent) = player::create_players(&player_assignment);
    let mut game =
        game_state::GameState::new(our_player, opponent, board::Board::new(&board_dimensions));

    // Load initial board configuration
    game.board.load_state(&mut input_lines);

    // Main game loop - process turns until game ends
    loop {
        let line = match input_lines.next() {
            Some(Ok(l)) => l,
            _ => break, // Game ended
        };

        // Board update received - read new state
        if line.starts_with("Anfield") {
            game.board.load_state(&mut input_lines);
        }

        // Our turn - receive piece and calculate best placement
        if line.starts_with("Piece") {
            let mut piece_to_place = piece::Piece::from_header(&line);
            piece_to_place.load_shape(&mut input_lines);

            // Calculate optimal placement using our strategy
            let (best_col, best_row) = game.calculate_best_move(piece_to_place);

            // Send move to game engine
            println!("{} {}", best_col, best_row);
        }
    }
}
