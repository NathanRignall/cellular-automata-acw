fn available_positions(position: (usize, usize), grid: &Vec<Vec<bool>>) -> Vec<(usize, usize)> {
    let (x, y) = position;
    let mut positions = Vec::new();
    
    // check up down left right
    if x > 0 && !grid[x - 1][y] {
        positions.push((x - 1, y));
    }
    if x < grid.len() - 1 && !grid[x + 1][y] {
        positions.push((x + 1, y));
    }
    if y > 0 && !grid[x][y - 1] {
        positions.push((x, y - 1));
    }
    if y < grid[0].len() - 1 && !grid[x][y + 1] {
        positions.push((x, y + 1));
    }

    positions
}

fn main() {
    // open a csv file and write the header
    let mut wtr = csv::Writer::from_path("output/task1-1.csv").unwrap();
    wtr.write_record(&["x", "y"]).unwrap();

    // parameters for the simulation
    let grid_size = 100;
    let iterations = 100;

    // create a grid of cells 100x100 which are bool and set to false
    let mut grid = vec![vec![false; grid_size]; grid_size];
    grid[grid_size / 2][grid_size / 2] = true;
    let mut position = (grid_size / 2, grid_size / 2);

    // loop for n iterations
    for _ in 0..iterations {
        // make a new grid of cells
        let mut new_grid = vec![vec![false; grid_size]; grid_size];

        // move the cell
        let available_positions = available_positions(position, &grid);
        let random_position = rand::random::<usize>() % available_positions.len();
        let new_position = available_positions[random_position];
        new_grid[new_position.0][new_position.1] = true;

        // update the grid and position
        grid = new_grid;
        position = new_position;

        // write the position to the csv file
        wtr.write_record(&[position.0.to_string(), position.1.to_string()]).unwrap();
    }
}