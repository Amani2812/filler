use crate::model::{Board, Piece, Player};

pub fn can_place(board: &Board, piece: &Piece, player: Player, top_x: isize, top_y: isize) -> bool {
    let own = player.own_chars();
    let opp = player.opp_chars();
    let mut overlap_own = 0usize;

    for py in 0..piece.height {
        for px in 0..piece.width {
            if piece.cells[py][px] != '#' && piece.cells[py][px] != 'O' && piece.cells[py][px] != '*' {
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

pub fn find_move(board: &Board, piece: &Piece, player: Player) -> (isize, isize) {
    for y in 0..board.height as isize {
        for x in 0..board.width as isize {
            if can_place(board, piece, player, x, y) {
                return (x, y);
            }
        }
    }
    (0, 0)
}

pub fn format_move(x: isize, y: isize) -> String {
    format!("{x} {y}")
}
