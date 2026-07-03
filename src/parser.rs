use crate::model::{Board, Piece, Player};

pub fn parse_player(line: &str) -> Option<Player> {
    if line.contains("$$$ exec p1") {
        Some(Player::One)
    } else if line.contains("$$$ exec p2") {
        Some(Player::Two)
    } else {
        None
    }
}

pub fn parse_board_header(line: &str) -> Option<(usize, usize)> {
    // Example: "Anfield 20 15:"
    let mut parts = line.split_whitespace();
    let tag = parts.next()?;
    if tag != "Anfield" {
        return None;
    }
    let width = parts.next()?.parse::<usize>().ok()?;
    let height_raw = parts.next()?;
    let height = height_raw.trim_end_matches(':').parse::<usize>().ok()?;
    Some((width, height))
}

pub fn parse_piece_header(line: &str) -> Option<(usize, usize)> {
    // Example: "Piece 4 1:"
    let mut parts = line.split_whitespace();
    let tag = parts.next()?;
    if tag != "Piece" {
        return None;
    }
    let width = parts.next()?.parse::<usize>().ok()?;
    let height_raw = parts.next()?;
    let height = height_raw.trim_end_matches(':').parse::<usize>().ok()?;
    Some((width, height))
}

pub fn parse_board(lines: &[String], width: usize, height: usize) -> Option<Board> {
    if lines.len() < height + 1 {
        return None;
    }
    // lines[0] is coordinate guide line
    let mut cells = Vec::with_capacity(height);
    for row in lines.iter().skip(1).take(height) {
        // row example: "000 ...................."
        let mut parts = row.split_whitespace();
        let _idx = parts.next()?;
        let content = parts.next()?;
        if content.chars().count() != width {
            return None;
        }
        cells.push(content.chars().collect::<Vec<char>>());
    }
    Some(Board { width, height, cells })
}

pub fn parse_piece(lines: &[String], width: usize, height: usize) -> Option<Piece> {
    if lines.len() < height {
        return None;
    }
    let mut cells = Vec::with_capacity(height);
    for row in lines.iter().take(height) {
        if row.chars().count() != width {
            return None;
        }
        cells.push(row.chars().collect::<Vec<char>>());
    }
    Some(Piece { width, height, cells })
}
