use solution::place_piece::put_piece;

fn board() -> Vec<Vec<char>> {
    vec![
        vec!['.', '.', '.', '.'],
        vec!['.', '@', '.', '.'],
        vec!['.', '.', '$', '.'],
        vec!['.', '.', '.', '.'],
    ]
}

#[test]
fn accepts_exactly_one_overlap_with_own_cell() {
    assert!(put_piece(&board(), &[vec!['O']], &['@', 'a'], 1, 1));
}

#[test]
fn rejects_two_overlaps_with_own_cells() {
    let grid = vec![vec!['@', '@'], vec!['.', '.']];
    assert!(!put_piece(&grid, &[vec!['O', 'O']], &['@', 'a'], 0, 0));
}

#[test]
fn rejects_an_opponent_overlap() {
    assert!(!put_piece(&board(), &[vec!['O']], &['@', 'a'], 2, 2));
}

#[test]
fn rejects_a_piece_partially_outside_the_grid() {
    assert!(!put_piece(&board(), &[vec!['O', 'O']], &['@', 'a'], 3, 1));
    assert!(!put_piece(&board(), &[vec!['O'], vec!['O']], &['@', 'a'], 1, 3));
}
