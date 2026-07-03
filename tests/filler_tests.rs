use filler_bot::logic::{can_place, format_move};
use filler_bot::model::{Board, Piece, Player};
use filler_bot::parser::{parse_board, parse_piece};

#[test]
fn test_parse_board_and_piece() {
    let board_lines = vec![
        "    0123".to_string(),
        "000 ....".to_string(),
        "001 .@..".to_string(),
        "002 ....".to_string(),
    ];
    let board = parse_board(&board_lines, 4, 3).expect("board should parse");
    assert_eq!(board.width, 4);
    assert_eq!(board.height, 3);
    assert_eq!(board.cells[1][1], '@');

    let piece_lines = vec![".#".to_string(), "##".to_string()];
    let piece = parse_piece(&piece_lines, 2, 2).expect("piece should parse");
    assert_eq!(piece.width, 2);
    assert_eq!(piece.height, 2);
}

#[test]
fn test_valid_placement_exactly_one_overlap() {
    let board = Board {
        width: 4,
        height: 3,
        cells: vec![
            vec!['.', '.', '.', '.'],
            vec!['.', '@', '.', '.'],
            vec!['.', '.', '.', '.'],
        ],
    };
    let piece = Piece {
        width: 2,
        height: 2,
        cells: vec![vec!['#', '.'], vec!['.', '.']],
    };

    assert!(can_place(&board, &piece, Player::One, 1, 1));
}

#[test]
fn test_invalid_when_overlapping_opponent() {
    let board = Board {
        width: 4,
        height: 3,
        cells: vec![
            vec!['.', '.', '.', '.'],
            vec!['.', '$', '.', '.'],
            vec!['.', '.', '.', '.'],
        ],
    };
    let piece = Piece {
        width: 1,
        height: 1,
        cells: vec![vec!['#']],
    };

    assert!(!can_place(&board, &piece, Player::One, 1, 1));
}

#[test]
fn test_boundary_detection() {
    let board = Board {
        width: 3,
        height: 3,
        cells: vec![
            vec!['@', '.', '.'],
            vec!['.', '.', '.'],
            vec!['.', '.', '.'],
        ],
    };
    let piece = Piece {
        width: 2,
        height: 2,
        cells: vec![vec!['#', '#'], vec!['#', '.']],
    };

    assert!(!can_place(&board, &piece, Player::One, 2, 2));
}

#[test]
fn test_format_output() {
    let out = format_move(7, 2);
    assert_eq!(out, "7 2");
}
