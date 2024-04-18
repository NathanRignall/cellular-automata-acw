pub fn move_cell(position: &mut (usize, usize), grid: &mut Vec<Vec<f64>>, diagonal: bool) {
    // move the cell
    let available_positions = {
        if diagonal {
            crate::position::available_diagonal(*position, grid)
        } else {
            crate::position::available_normal(*position, grid)
        }
    };
    let random_position = rand::random::<usize>() % available_positions.len();
    *position = available_positions[random_position];
    grid[position.0][position.1] = 1.0;
}

pub fn simulate(
    grid_size: usize,
    start_position: (usize, usize),
    end_position: (usize, usize),
    max_iterations: usize,
    diagonal: bool,
    finish: bool,
) -> Vec<(usize, usize)> {
    // create a grid of cells 100x100 which are bool and set to false
    let mut grid = vec![vec![0.0; grid_size]; grid_size];
    let mut position = start_position;
    grid[position.0][position.1] = 1.0;

    // loop for n iterations
    let mut positions = Vec::new();
    for _ in 0..max_iterations {
        // simulate the movement of the cells
        move_cell(&mut position, &mut grid, diagonal);

        // store the position
        positions.push(position);

        // stop if the cell is at the end position
        if position == end_position && finish {
            break;
        }
    }
    positions
}

pub fn simulate_distance(
    grid_size: usize,
    start_position: (usize, usize),
    distance: f64,
    max_iterations: usize,
    diagonal: bool,
    finish: bool,
) -> Vec<(usize, usize)> {
    // create a grid of cells 100x100 which are bool and set to false
    let mut grid = vec![vec![0.0; grid_size]; grid_size];
    let mut position = start_position;
    grid[position.0][position.1] = 1.0;

    // loop for n iterations
    let mut positions = Vec::new();
    for _ in 0..max_iterations {
        // simulate the movement of the cells
        move_cell(&mut position, &mut grid, diagonal);

        // store the position
        positions.push(position);

        // stop if the cell is at distance from the start position
        if crate::position::distance(start_position, position) >= distance && finish {
            break;
        }
    }
    positions
}