use std::io;
use crate::place_piece::place_piece;
mod place_piece;

fn main() {
    let (play, enemy) = get_player_characters();
    
    loop {
        let grid = read_grid();
        let piece = read_piece();
        let (piece_x, piece_y) = place_piece(&grid, &piece, &play, &enemy);
        println!("{} {}", piece_x, piece_y);
    }
}

fn get_player_characters() -> (Vec<char>, Vec<char>) {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let player_number = input.chars().nth(10).unwrap();
    if player_number == '1' {
        (vec!['@', 'a'], vec!['$', 's'])
    } else {
        (vec!['$', 's'], vec!['@', 'a'])
    }
}
fn read_grid() -> Vec<Vec<char>> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let grid_details = input.split_whitespace().collect::<Vec<&str>>()[2];
    let grid_lines = grid_details[..grid_details.len() - 1]
        .parse::<i32>()
        .unwrap();

    let mut grid = Vec::new();
    for i in 0..grid_lines + 1 {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        if i < 1 {
            continue;
        } else {
            let row: Vec<char> = input[4..input.len() - 1].chars().collect();
            grid.push(row);
        }
    }
    grid
}

// Read the piece from the standard input and return it as a 2D vector.
fn read_piece() -> Vec<Vec<char>> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let piece_details = input.split_whitespace().collect::<Vec<&str>>();
    let piece_lines = piece_details[2][..piece_details[2].len() - 1]
        .parse::<i32>()
        .unwrap();

    let mut piece = Vec::new();
    for _ in 0..piece_lines {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let row: Vec<char> = input[..input.len() - 1].chars().collect();
        piece.push(row);
    }
    piece
}

