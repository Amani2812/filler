pub fn short_distance(grid: &Vec<Vec<char>>, distance: f32, play: &Vec<char>, enemy: &Vec<char>) -> f32 {
    
    let mut min_dist = distance; 
    for (yg, row) in grid.iter().enumerate() {
        for (xg, &cell) in row.iter().enumerate() {
            if cell == play[1] {
                for (ye, enemy_row) in grid.iter().enumerate() {
                    for (xe, &enemy_cell) in enemy_row.iter().enumerate() {
                        if enemy.contains(&enemy_cell) {
                            let dist = (((ye as f32) - (yg as f32)).powf(2.) + ((xe as f32) - (xg as f32)).powf(2.)).sqrt();
                            min_dist = min_dist.min(dist);
                        }
                    }
                }
            }
        }
    }
    min_dist
}
pub fn place_piece(grid: &Vec<Vec<char>>, piece: &Vec<Vec<char>>, play: &Vec<char>, enemy: &Vec<char> ) -> (usize, usize) {
    let grid_rows = grid[0].len();
    let piece_rows = piece[0].len();
    let mut distance = ((grid_rows as f32).powf(2.) + (grid.len() as f32).powf(2.)).sqrt(); 
    let mut sol = (0, 0);


    let (mut xmin, mut xmax, mut ymin, mut ymax) = (grid.len(), 0, grid_rows, 0);
    for yg in 0..grid.len() {
        for xg in 0..grid_rows {
            if play.contains(&grid[yg][xg]) {
                xmin = xmin.min(xg);
                xmax = xmax.max(xg);
                ymin = ymin.min(yg);
                ymax = ymax.max(yg);
            }
        }
    }

    let (mut xi, mut xf, mut yi, mut yf) = (
        0,
        grid_rows - piece_rows + 1,
        0,
        grid.len() - piece.len() + 1,
    );

    let temp = xmin as i32 - piece_rows as i32 - 1;
    if temp > 0 {
        xi = xmin - piece_rows + 1
    }
    if (xmax + piece_rows - 1) < grid_rows {
        xf = xmax + 1
    }
    let temp = ymin as i32 - piece.len() as i32 + 1;
    if temp > 0 {
        yi = ymin - piece.len() + 1
    }
    if (ymax + piece.len() - 1) < grid.len() {
        yf = ymax + 1
    }

    for yg in yi..yf {
        for xg in xi..xf {
            if put_piece(grid, piece, play, xg, yg) {
                let min_dist = short_distance(
                    &grid_with_piece(grid, piece, play, (xg, yg)),
                    distance,
                    play,
                    enemy,
                );
                if min_dist < distance {
                    distance = min_dist;
                    sol = (xg, yg);
                }
            }
        }
    }
   
    sol
}
// Function to place a piece on a grid.

pub fn grid_with_piece( grid: &[Vec<char>], piece: &[Vec<char>], play: &[char], coords: (usize, usize) ) -> Vec<Vec<char>> {
    // Create a modifyable copy
    let mut new_grid = grid.to_vec();
    let prows = piece[0].len(); // number of piece rows

    // Replace with valid chars
    for yg in 0..new_grid.len() {
        for xg in 0..new_grid[0].len() {
            if new_grid[yg][xg] == play[1] {
                new_grid[yg][xg] = play[0]
            }
        }
    }

    for yp in 0..piece.len() {
        for xp in 0..prows {
            if piece[yp][xp] != '.' {
                new_grid[yp + coords.1][xp + coords.0] = play[1]
            }
        }
    }
    new_grid
}

//Function to place the piece
pub fn put_piece(grid: &[Vec<char>], piece: &[Vec<char>], play: &[char], xg: usize, yg: usize ) -> bool {
    let mut cross = 0; // Counter for valid intersections

    // Go through each line (row) and column (val) of the piece
    for (yp, row) in piece.iter().enumerate() {
        for (xp, &val) in row.iter().enumerate() {
            // Check if the cell is part of the piece
            if val != '.' {
                let cell = grid.get(yg + yp).and_then(|r| r.get(xg + xp));

                match cell {
                    Some(&c) if play.contains(&c) => cross += 1,
                    Some(&c) if c != '.' => return false,
                    _ => {}
                }

                // Return false if more than one valid intersection is found
                if cross > 1 {
                    return false;
                }
            }
        }
    }
    // Return true if exactly one valid intersection is found
    cross == 1
}
