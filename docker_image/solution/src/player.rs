/// Represents a player in the game
/// Each player has unique symbols for placed pieces
#[derive(Debug, Clone)]
pub struct Player {
    pub number: usize,         // 1 or 2
    pub symbols: (char, char), // (lowercase, uppercase) e.g., ('a', '@')
    pub cells_captured: usize, // Score tracking
}

impl Player {
    /// Checks if a cell belongs to this player
    pub fn owns_cell(&self, cell: &char) -> bool {
        self.symbols.0 == *cell || self.symbols.1 == *cell
    }
}

/// Parses the player assignment line and creates both players
/// Input format: "$$$ exec p1 : [bot_path]" or "$$$ exec p2 : [bot_path]"
/// Returns: (our_player, opponent)
pub fn create_players(assignment_line: &str) -> (Player, Player) {
    let player_one = Player {
        number: 1,
        symbols: ('a', '@'),
        cells_captured: 0,
    };

    let player_two = Player {
        number: 2,
        symbols: ('s', '$'),
        cells_captured: 0,
    };

    // Determine which player we are
    if assignment_line.contains("p1") {
        (player_one, player_two) // We are player 1
    } else {
        (player_two, player_one) // We are player 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_players_as_p1() {
        let (us, them) = create_players("$$$ exec p1 : [bot]");
        assert_eq!(us.number, 1);
        assert_eq!(us.symbols, ('a', '@'));
        assert_eq!(them.number, 2);
    }

    #[test]
    fn test_create_players_as_p2() {
        let (us, them) = create_players("$$$ exec p2 : [bot]");
        assert_eq!(us.number, 2);
        assert_eq!(us.symbols, ('s', '$'));
        assert_eq!(them.number, 1);
    }

    #[test]
    fn test_owns_cell() {
        let player = Player {
            number: 1,
            symbols: ('a', '@'),
            cells_captured: 0,
        };

        assert!(player.owns_cell(&'a'));
        assert!(player.owns_cell(&'@'));
        assert!(!player.owns_cell(&'s'));
        assert!(!player.owns_cell(&'.'));
    }
}
