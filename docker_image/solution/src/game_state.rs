pub use crate::board::*;
pub use crate::piece::*;
pub use crate::player::*;
pub use crate::strategy::*;

/// Main game state tracker
/// Manages the current game, player states, and placement history
#[derive(Debug, Clone)]
pub struct GameState {
    pub our_player: Player,
    pub opponent: Player,
    pub board: Board,
    pub placement_history: Vec<Piece>,
    pub turn_number: usize,
}

/// Represents a potential piece placement with its calculated score
#[derive(Debug, Clone)]
pub struct PotentialMove {
    pub position: Position,
    pub score: i32,
    pub piece: Piece,
}

impl GameState {
    /// Creates a new game state
    pub fn new(our_player: Player, opponent: Player, board: Board) -> Self {
        Self {
            our_player,
            opponent,
            board,
            placement_history: Vec::new(),
            turn_number: 0,
        }
    }

    /// Calculates the best move for the given piece
    /// Returns (x, y) coordinates or (0, 0) if no valid placement exists
    pub fn calculate_best_move(&mut self, piece: Piece) -> (i32, i32) {
        self.turn_number += 1;

        // Quick check: piece too big for board
        if piece.trimmed_dimensions.height > self.board.dimensions.height
            || piece.trimmed_dimensions.width > self.board.dimensions.width
        {
            return (0, 0);
        }

        // Calculate opponent's average position for strategic targeting
        let opponent_center = calculate_average_position(
            &self.board,
            self.our_player.symbols,
            true, // looking for opponent
        );

        // Find all valid placements for this piece
        let mut valid_placements: Vec<PotentialMove> = Vec::new();

        // Try every possible position on the board
        // Start from piece offset to avoid checking positions where piece would be off-board
        let max_row = self.board.dimensions.height - piece.trimmed_dimensions.height;
        let max_col = self.board.dimensions.width - piece.trimmed_dimensions.width;

        for row in piece.trim_offset.0..=max_row {
            for col in piece.trim_offset.1..=max_col {
                // Check if placement at this position is valid
                if let Some(valid_move) = self.validate_placement(&piece, Position { row, col }) {
                    valid_placements.push(valid_move);
                }
            }
        }

        // No valid moves found
        if valid_placements.is_empty() {
            return (0, 0);
        }

        // Evaluate all valid placements and pick the best one
        let best_move = evaluate_all_placements(
            &self.board,
            valid_placements,
            opponent_center,
            self.turn_number,
            self.our_player.symbols,
            &self.placement_history,
        );

        // Update game state
        self.placement_history.push(piece);
        self.our_player.cells_captured += 1;

        // Convert from trimmed position to original piece coordinates
        (
            best_move.position.col as i32 - best_move.piece.trim_offset.1 as i32,
            best_move.position.row as i32 - best_move.piece.trim_offset.0 as i32,
        )
    }

    /// Validates if a piece can be placed at the given position
    /// Returns Some(PotentialMove) if valid, None otherwise
    ///
    /// Rules:
    /// 1. Piece must overlap exactly ONE of our existing cells
    /// 2. Piece cannot overlap any opponent cells
    /// 3. All piece cells must fit on the board
    fn validate_placement(&self, piece: &Piece, position: Position) -> Option<PotentialMove> {
        let mut overlap_count = 0;

        // Check each filled cell in the piece
        for (piece_row, row_data) in piece.trimmed_cells.iter().enumerate() {
            for (piece_col, &piece_cell) in row_data.iter().enumerate() {
                // Skip empty cells in the piece
                if piece_cell == '.' {
                    continue;
                }

                // Calculate board position for this piece cell
                let board_row = piece_row + position.row;
                let board_col = piece_col + position.col;

                let board_cell = self.board.cells[board_row][board_col];

                // Check if overlapping with our own pieces
                if self.our_player.owns_cell(&board_cell) {
                    overlap_count += 1;
                    // Invalid: can only overlap exactly 1 cell
                    if overlap_count > 1 {
                        return None;
                    }
                }
                // Check if overlapping with opponent pieces
                else if self.opponent.owns_cell(&board_cell) {
                    return None; // Invalid: cannot overlap opponent
                }
            }
        }

        // Must overlap exactly 1 of our cells
        if overlap_count != 1 {
            return None;
        }

        // Calculate strategic score for this placement
        let mut total_score = 0;
        for (piece_row, row_data) in piece.trimmed_cells.iter().enumerate() {
            for (piece_col, &piece_cell) in row_data.iter().enumerate() {
                let board_pos = Position {
                    row: position.row + piece_row,
                    col: position.col + piece_col,
                };
                total_score += self.score_cell_placement(piece_cell, board_pos);
            }
        }

        Some(PotentialMove {
            position,
            score: total_score,
            piece: piece.clone(),
        })
    }

    /// Calculates strategic value of placing a piece cell at a position
    /// Higher scores indicate better strategic positions
    fn score_cell_placement(&self, piece_cell: char, board_position: Position) -> i32 {
        let is_filled_cell = piece_cell == 'O';

        // Get cells adjacent to this position (up, down, left, right)
        let adjacent = get_adjacent_cells(&self.board, &board_position);

        if !is_filled_cell {
            // Empty cell in piece - check what's on board
            let current_cell = self.board.cells[board_position.row][board_position.col];

            if self.our_player.owns_cell(&current_cell) {
                1 // Slightly good - we're near our territory
            } else if self.opponent.owns_cell(&current_cell) {
                2 // Better - we're near opponent (good for blocking)
            } else {
                0 // Neutral empty space
            }
        } else {
            // Filled cell in piece - prioritize placing next to opponent
            // This helps us "chase" and block them

            if adjacent.up.is_some() && self.opponent.owns_cell(&adjacent.up.unwrap()) {
                return 4; // High value - blocking opponent from above
            }
            if adjacent.down.is_some() && self.opponent.owns_cell(&adjacent.down.unwrap()) {
                return 4; // High value - blocking opponent from below
            }
            if adjacent.left.is_some() && self.opponent.owns_cell(&adjacent.left.unwrap()) {
                return 4; // High value - blocking opponent from left
            }
            if adjacent.right.is_some() && self.opponent.owns_cell(&adjacent.right.unwrap()) {
                return 4; // High value - blocking opponent from right
            }

            0 // No immediate tactical advantage
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid_trait::Dimensions;

    fn create_test_game() -> GameState {
        let player = Player {
            number: 1,
            symbols: ('a', '@'),
            cells_captured: 0,
        };
        let opponent = Player {
            number: 2,
            symbols: ('s', '$'),
            cells_captured: 0,
        };

        let board = Board {
            dimensions: Dimensions {
                width: 4,
                height: 4,
            },
            cells: vec![
                vec!['.', 'a', '.', '.'], // Our piece at (0,1)
                vec!['.', '.', '.', '.'],
                vec!['.', '.', 's', '.'], // Opponent at (2,2)
                vec!['.', '.', '.', '.'],
            ],
        };

        GameState::new(player, opponent, board)
    }

    fn create_test_piece() -> Piece {
        Piece {
            original_dimensions: Dimensions {
                width: 2,
                height: 2,
            },
            original_cells: vec![vec!['O', '.'], vec!['.', 'O']],
            trimmed_dimensions: Dimensions {
                width: 2,
                height: 2,
            },
            trimmed_cells: vec![vec!['O', '.'], vec!['.', 'O']],
            filled_cell_count: 2,
            trim_offset: (0, 0),
        }
    }

    #[test]
    fn test_valid_placement() {
        let game = create_test_game();
        let piece = create_test_piece();

        // Place at (1,0) so piece overlaps our 'a' at (0,1)
        let result = game.validate_placement(&piece, Position { row: 0, col: 1 });

        assert!(result.is_some());
        let placement = result.unwrap();
        assert_eq!(placement.position.col, 1);
        assert_eq!(placement.position.row, 0);
    }

    #[test]
    fn test_invalid_placement_overlaps_opponent() {
        let game = create_test_game();
        let piece = create_test_piece();

        // Try to place where it overlaps opponent at (2,2)
        let result = game.validate_placement(&piece, Position { row: 1, col: 1 });

        assert!(result.is_none()); // Should be invalid
    }
}
