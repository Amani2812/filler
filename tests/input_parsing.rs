use solution::parsing::{parse_dimensions, parse_grid_rows, parse_piece_rows, parse_player_characters};

#[test]
fn reads_player_one_characters() {
    assert_eq!(parse_player_characters("$$$ exec p1 : [player]").unwrap(), (vec!['@', 'a'], vec!['$', 's']));
}

#[test]
fn reads_anfield_dimensions_and_rows() {
    assert_eq!(parse_dimensions("Anfield 3 2:").unwrap(), (3, 2));
    let rows = vec!["000 .@.".to_string(), "001 ..$".to_string()];
    assert_eq!(parse_grid_rows(&rows, 3, 2).unwrap(), vec![vec!['.', '@', '.'], vec!['.', '.', '$']]);
}

#[test]
fn reads_piece_shape_and_rejects_wrong_width() {
    let rows = vec![".OO".to_string(), "OO.".to_string()];
    assert_eq!(parse_piece_rows(&rows, 3, 2).unwrap(), vec![vec!['.', 'O', 'O'], vec!['O', 'O', '.']]);
    assert!(parse_piece_rows(&rows, 2, 2).is_err());
}
