use crate::model::{Board, Piece, Player};

fn is_piece_cell(cell: char) -> bool {
    matches!(cell, '#' | 'O' | '*')
}

pub fn can_place(board: &Board, piece: &Piece, player: Player, top_x: isize, top_y: isize) -> bool {
    let own = player.own_chars();
    let opp = player.opp_chars();
    let mut overlap_own = 0usize;

    for py in 0..piece.height {
        for px in 0..piece.width {
            if !is_piece_cell(piece.cells[py][px]) {
                continue;
            }
            let bx = top_x + px as isize;
            let by = top_y + py as isize;

            if bx < 0 || by < 0 || bx >= board.width as isize || by >= board.height as isize {
                return false;
            }

            let c = board.cells[by as usize][bx as usize];
            if opp.contains(&c) {
                return false;
            }
            if own.contains(&c) {
                overlap_own += 1;
                if overlap_own > 1 {
                    return false;
                }
            }
        }
    }

    overlap_own == 1
}

fn apply_move(board: &Board, piece: &Piece, player: Player, top_x: isize, top_y: isize) -> Option<Board> {
    let own = player.own_chars();
    let mut next = board.clone();

    for py in 0..piece.height {
        for px in 0..piece.width {
            if !is_piece_cell(piece.cells[py][px]) {
                continue;
            }
            let bx = top_x + px as isize;
            let by = top_y + py as isize;

            if bx < 0 || by < 0 || bx >= board.width as isize || by >= board.height as isize {
                return None;
            }

            let cell = board.cells[by as usize][bx as usize];
            if cell == own[0] || cell == own[1] {
                continue;
            }
            if cell == '.' {
                next.cells[by as usize][bx as usize] = own[0];
            }
        }
    }

    Some(next)
}

fn count_valid_moves(board: &Board, piece: &Piece, player: Player) -> usize {
    let mut count = 0usize;

    for y in 0..board.height as isize {
        for x in 0..board.width as isize {
            if can_place(board, piece, player, x, y) {
                count += 1;
            }
        }
    }

    count
}

pub fn score_move(board: &Board, piece: &Piece, player: Player, top_x: isize, top_y: isize) -> isize {
    let own = player.own_chars();
    let opp = player.opp_chars();
    let mut overlap_own = 0usize;
    let mut claimed_empty = 0isize;
    let mut adjacent_own = 0isize;
    let mut adjacent_opp = 0isize;
    let mut adjacent_empty = 0isize;

    for py in 0..piece.height {
        for px in 0..piece.width {
            if !is_piece_cell(piece.cells[py][px]) {
                continue;
            }
            let bx = top_x + px as isize;
            let by = top_y + py as isize;

            if bx < 0 || by < 0 || bx >= board.width as isize || by >= board.height as isize {
                return isize::MIN;
            }

            let c = board.cells[by as usize][bx as usize];
            if opp.contains(&c) {
                return isize::MIN;
            }
            if own.contains(&c) {
                overlap_own += 1;
            } else {
                claimed_empty += 1;
            }

            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let nx = bx + dx;
                    let ny = by + dy;
                    if nx < 0 || ny < 0 || nx >= board.width as isize || ny >= board.height as isize {
                        continue;
                    }
                    let neighbor = board.cells[ny as usize][nx as usize];
                    if own.contains(&neighbor) {
                        adjacent_own += 1;
                    } else if opp.contains(&neighbor) {
                        adjacent_opp += 1;
                    } else {
                        adjacent_empty += 1;
                    }
                }
            }
        }
    }

    if overlap_own != 1 {
        return isize::MIN;
    }

    let next = match apply_move(board, piece, player, top_x, top_y) {
        Some(n) => n,
        None => return isize::MIN,
    };
    let future_moves = count_valid_moves(&next, piece, player) as isize;

    let mut score = claimed_empty * 180;
    score += adjacent_own * 60;
    score -= adjacent_opp * 220;
    score += future_moves * 700;
    score += adjacent_empty * 6;

    if adjacent_own > 0 {
        score += 120;
    }
    if adjacent_opp > 0 {
        score -= 40;
    }

    score
}

pub fn find_move(board: &Board, piece: &Piece, player: Player) -> (isize, isize) {
    let mut best_score = isize::MIN;
    let mut best_move = (0, 0);

    for y in 0..board.height as isize {
        for x in 0..board.width as isize {
            if !can_place(board, piece, player, x, y) {
                continue;
            }
            let score = score_move(board, piece, player, x, y);
            if score > best_score {
                best_score = score;
                best_move = (x, y);
            }
        }
    }

    best_move
}

pub fn format_move(x: isize, y: isize) -> String {
    format!("{x} {y}")
}
