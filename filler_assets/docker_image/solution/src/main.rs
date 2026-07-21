use std::io::{self, BufRead, Write};

use filler_bot::logic::{find_move, format_move};
use filler_bot::model::Player;
use filler_bot::parser::{parse_board, parse_board_header, parse_piece, parse_piece_header, parse_player};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut player = Player::One;

    while let Some(Ok(line)) = lines.next() {
        if let Some(p) = parse_player(&line) {
            player = p;
            continue;
        }

        if let Some((board_w, board_h)) = parse_board_header(&line) {
            let mut board_lines = Vec::with_capacity(board_h + 1);
            for _ in 0..(board_h + 1) {
                match lines.next() {
                    Some(Ok(l)) => board_lines.push(l),
                    _ => return,
                }
            }
            let board = match parse_board(&board_lines, board_w, board_h) {
                Some(b) => b,
                None => {
                    println!("0 0");
                    let _ = io::stdout().flush();
                    continue;
                }
            };

            let piece_header = match lines.next() {
                Some(Ok(h)) => h,
                _ => return,
            };
            let (piece_w, piece_h) = match parse_piece_header(&piece_header) {
                Some(v) => v,
                None => {
                    println!("0 0");
                    let _ = io::stdout().flush();
                    continue;
                }
            };

            let mut piece_lines = Vec::with_capacity(piece_h);
            for _ in 0..piece_h {
                match lines.next() {
                    Some(Ok(l)) => piece_lines.push(l),
                    _ => return,
                }
            }
            let piece = match parse_piece(&piece_lines, piece_w, piece_h) {
                Some(p) => p,
                None => {
                    println!("0 0");
                    let _ = io::stdout().flush();
                    continue;
                }
            };

            let (x, y) = find_move(&board, &piece, player);
            println!("{}", format_move(x, y));
            let _ = io::stdout().flush();
        }
    }
}
