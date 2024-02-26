pub fn available_diagonal(position: (usize, usize), grid: &Vec<Vec<f64>>) -> Vec<(usize, usize)> {
    let (x, y) = position;
    let mut positions = Vec::new();
    
    // check up down left right
    if x > 0 {
        positions.push((x - 1, y));
    }
    if x < grid.len() - 1 {
        positions.push((x + 1, y));
    }
    if y > 0 {
        positions.push((x, y - 1));
    }
    if y < grid[0].len() - 1 {
        positions.push((x, y + 1));
    }

    // check diagonals
    if x > 0 && y > 0 {
        positions.push((x - 1, y - 1));
    }
    if x > 0 && y < grid[0].len() - 1 {
        positions.push((x - 1, y + 1));
    }
    if x < grid.len() - 1 && y > 0 {
        positions.push((x + 1, y - 1));
    }
    if x < grid.len() - 1 && y < grid[0].len() - 1 {
        positions.push((x + 1, y + 1));
    }
    positions
}

pub fn available_normal(position: (usize, usize), grid: &Vec<Vec<bool>>) -> Vec<(usize, usize)> {
    let (x, y) = position;
    let mut positions = Vec::new();
    
    // check up down left right
    if x > 0 {
        positions.push((x - 1, y));
    }
    if x < grid.len() - 1 {
        positions.push((x + 1, y));
    }
    if y > 0 {
        positions.push((x, y - 1));
    }
    if y < grid[0].len() - 1 {
        positions.push((x, y + 1));
    }

    positions
}