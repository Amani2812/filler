use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct GameState {
    player_num: u8,
    player_char: char,
    opponent_char: char,
    anfield_width: usize,
    anfield_height: usize,
    anfield: Vec<Vec<char>>,
    piece: Vec<Vec<char>>,
}

impl GameState {
    fn new() -> Self {
        GameState {
            player_num: 0,
            player_char: '.',
            opponent_char: '.',
            anfield_width: 0,
            anfield_height: 0,
            anfield: Vec::new(),
            piece: Vec::new(),
        }
    }

    fn parse_player_line(&mut self, line: &str) {
        // Parse: $$$ exec p1 : [robots/bender]
        if line.contains("p1") {
            self.player_num = 1;
            self.player_char = '@';
            self.opponent_char = '$';
        } else if line.contains("p2") {
            self.player_num = 2;
            self.player_char = '$';
            self.opponent_char = '@';
        }
    }

    fn parse_anfield_header(&mut self, line: &str) {
        // Parse: Anfield 20 15:
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            self.anfield_height = parts[1].parse().unwrap_or(0);
            self.anfield_width = parts[2].trim_end_matches(':').parse().unwrap_or(0);
        }
    }

    fn can_place_piece(&self, row: i32, col: i32) -> bool {
        let mut overlap_count = 0;
        let player_lower = self.player_char.to_lowercase().next().unwrap();
        let opponent_lower = self.opponent_char.to_lowercase().next().unwrap();

        for (piece_row, piece_line) in self.piece.iter().enumerate() {
            for (piece_col, &piece_char) in piece_line.iter().enumerate() {
                if piece_char == '.' {
                    continue;
                }

                let anfield_row = row + piece_row as i32;
                let anfield_col = col + piece_col as i32;

                // Check bounds
                if anfield_row < 0 || anfield_row >= self.anfield_height as i32 {
                    return false;
                }
                if anfield_col < 0 || anfield_col >= self.anfield_width as i32 {
                    return false;
                }

                let cell = self.anfield[anfield_row as usize][anfield_col as usize];
                
                // Cannot overlap opponent
                if cell == self.opponent_char || cell == opponent_lower {
                    return false;
                }

                // Count overlap with our territory
                if cell == self.player_char || cell == player_lower {
                    overlap_count += 1;
                }
            }
        }

        // Must have exactly 1 overlap with our territory
        overlap_count == 1
    }

    fn score_position(&self, row: i32, col: i32) -> i32 {
        let mut score = 0;
        let player_lower = self.player_char.to_lowercase().next().unwrap();
        let opponent_lower = self.opponent_char.to_lowercase().next().unwrap();

        // Calculate center of the board
        let center_row = self.anfield_height as i32 / 2;
        let center_col = self.anfield_width as i32 / 2;

        // Count our current territory for game phase detection
        let mut our_territory = 0;
        for row_vec in &self.anfield {
            for &cell in row_vec {
                if cell == self.player_char || cell == player_lower {
                    our_territory += 1;
                }
            }
        }

        let early_game = our_territory < 20;

        for (piece_row, piece_line) in self.piece.iter().enumerate() {
            for (piece_col, &piece_char) in piece_line.iter().enumerate() {
                if piece_char == '.' {
                    continue;
                }

                let anfield_row = row + piece_row as i32;
                let anfield_col = col + piece_col as i32;

                if anfield_row < 0 || anfield_row >= self.anfield_height as i32 {
                    continue;
                }
                if anfield_col < 0 || anfield_col >= self.anfield_width as i32 {
                    continue;
                }

                // Early game: prefer center control
                if early_game {
                    let dist_to_center = (anfield_row - center_row).abs() + (anfield_col - center_col).abs();
                    score -= dist_to_center * 2;
                }

                // Check adjacent cells
                let directions = [(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)];
                let mut empty_adjacent = 0;
                let mut opponent_adjacent = 0;

                for (dr, dc) in directions.iter() {
                    let adj_row = anfield_row + dr;
                    let adj_col = anfield_col + dc;

                    if adj_row >= 0 && adj_row < self.anfield_height as i32 
                        && adj_col >= 0 && adj_col < self.anfield_width as i32 {
                        let adj_cell = self.anfield[adj_row as usize][adj_col as usize];
                        
                        // Count adjacent opponent cells (blocking strategy)
                        if adj_cell == self.opponent_char || adj_cell == opponent_lower {
                            opponent_adjacent += 1;
                        }
                        
                        // Count adjacent empty cells (expansion potential)
                        if adj_cell == '.' {
                            empty_adjacent += 1;
                        }
                    }
                }

                // Aggressive blocking in mid/late game
                if !early_game && opponent_adjacent > 0 {
                    score += opponent_adjacent * 100;
                }

                // Expansion bonus
                score += empty_adjacent * 15;
            }
        }

        score
    }

    fn find_best_move(&self) -> (i32, i32) {
        let mut best_score = i32::MIN;
        let mut best_pos = (0, 0);
        let mut found_valid = false;

        // Optimize search range
        let piece_height = self.piece.len() as i32;
        let piece_width = if !self.piece.is_empty() { self.piece[0].len() as i32 } else { 0 };

        // Try all possible positions
        for row in -piece_height..self.anfield_height as i32 {
            for col in -piece_width..self.anfield_width as i32 {
                if self.can_place_piece(row, col) {
                    found_valid = true;
                    let score = self.score_position(row, col);
                    
                    if score > best_score {
                        best_score = score;
                        best_pos = (col, row);
                    }
                }
            }
        }

        if !found_valid {
            // No valid move found - return default
            return (0, 0);
        }

        best_pos
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    loop {
        let mut game = GameState::new();
        
        // Read player line
        if let Some(Ok(line)) = lines.next() {
            if line.starts_with("$$$") {
                game.parse_player_line(&line);
            } else {
                break; // End of input
            }
        } else {
            break;
        }

        // Read Anfield header
        if let Some(Ok(line)) = lines.next() {
            game.parse_anfield_header(&line);
        } else {
            break;
        }

        // Read column numbers line (skip)
        if let Some(Ok(_)) = lines.next() {
            // Skip column numbers
        } else {
            break;
        }

        // Read Anfield rows
        for _ in 0..game.anfield_height {
            if let Some(Ok(line)) = lines.next() {
                // Parse line like "000 ...................."
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let row: Vec<char> = parts[1].chars().collect();
                    game.anfield.push(row);
                }
            } else {
                break;
            }
        }

        // Read Piece header
        if let Some(Ok(line)) = lines.next() {
            // Extract piece dimensions from header
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                let piece_height: usize = parts[1].parse().unwrap_or(0);
                
                // Read piece rows
                for _ in 0..piece_height {
                    if let Some(Ok(piece_line)) = lines.next() {
                        let row: Vec<char> = piece_line.chars().collect();
                        game.piece.push(row);
                    }
                }
            }
        } else {
            break;
        }

        // Find and output best move
        let (x, y) = game.find_best_move();
        println!("{} {}", x, y);
    }
}
