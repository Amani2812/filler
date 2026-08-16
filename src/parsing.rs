pub fn parse_player_characters(input: &str) -> Result<(Vec<char>, Vec<char>), String> {
    let player = input
        .split_whitespace()
        .find(|token| *token == "p1" || *token == "p2")
        .ok_or_else(|| "player identifier not found".to_string())?;

    match player {
        "p1" => Ok((vec!['@', 'a'], vec!['$', 's'])),
        "p2" => Ok((vec!['$', 's'], vec!['@', 'a'])),
        _ => Err("invalid player identifier".to_string()),
    }
}

pub fn parse_dimensions(header: &str) -> Result<(usize, usize), String> {
    let fields: Vec<&str> = header.split_whitespace().collect();
    if fields.len() < 3 {
        return Err("dimension header is incomplete".to_string());
    }

    let width = fields[1]
        .parse::<usize>()
        .map_err(|_| "invalid board width".to_string())?;
    let height = fields[2]
        .trim_end_matches(':')
        .parse::<usize>()
        .map_err(|_| "invalid board height".to_string())?;
    Ok((width, height))
}

pub fn parse_grid_rows(rows: &[String], width: usize, height: usize) -> Result<Vec<Vec<char>>, String> {
    if rows.len() != height {
        return Err("incorrect number of grid rows".to_string());
    }

    rows.iter()
        .map(|line| {
            let row = line
                .split_whitespace()
                .last()
                .ok_or_else(|| "grid row is missing data".to_string())?;
            let cells: Vec<char> = row.chars().collect();
            if cells.len() != width {
                return Err("grid row has incorrect width".to_string());
            }
            Ok(cells)
        })
        .collect()
}

pub fn parse_piece_rows(rows: &[String], width: usize, height: usize) -> Result<Vec<Vec<char>>, String> {
    if rows.len() != height {
        return Err("incorrect number of piece rows".to_string());
    }

    rows.iter()
        .map(|line| {
            let cells: Vec<char> = line.trim().chars().collect();
            if cells.len() != width {
                return Err("piece row has incorrect width".to_string());
            }
            Ok(cells)
        })
        .collect()
}
