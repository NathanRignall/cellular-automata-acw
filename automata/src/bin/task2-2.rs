fn available_positions(position: (usize, usize), grid: &Vec<Vec<f64>>) -> Vec<(usize, usize)> {
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


fn main() {
    // open a csv file and write the header
    let mut wtr = csv::Writer::from_path("output/task2-2.csv").unwrap();
    wtr.write_record(&["x", "y", "cells"]).unwrap();

    // parameters for the simulation
    let k: f64 = 0.006;
    let m: f64 = 10_000_000_000_000.0;
    let initial_n: f64 = 1_000_000_000.0;
    let t_final: usize = 1200;
    let dt: f64 = 0.001;
    let grid_size = 100;
    let iterations = 100;

    // create a grid of cells 100x100 which are f64 and set to 0.0
    let mut grid: Vec<Vec<f64>> = vec![vec![0.0; grid_size]; grid_size];
    grid[grid_size / 2][grid_size / 2] = initial_n;
    let mut position = (grid_size / 2, grid_size / 2);

    // loop for n iterations
    for _ in 0..iterations {
        // make a new grid of cells
        let mut new_grid: Vec<Vec<f64>> = vec![vec![0.0; grid_size]; grid_size];

        // move the cell
        let available_positions = available_positions(position, &grid);
        let random_position = rand::random::<usize>() % available_positions.len();
        let new_position = available_positions[random_position];
        new_grid[new_position.0][new_position.1] = initial_n;

        // update the grid and position
        grid = new_grid;
        position = new_position;

        // write the position to the csv file
        wtr.write_record(&[position.0.to_string(), position.1.to_string(), grid[position.0][position.1].to_string()]).unwrap();
    }

}