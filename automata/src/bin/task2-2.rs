fn main() {
    // open a csv file and write the header
    let mut wtr = csv::Writer::from_path("output/task2-2.csv").unwrap();
    wtr.write_record(["x", "y", "cells"]).unwrap();

    // open another csv file and write the header
    let mut wtr2 = csv::Writer::from_path("output/task2-2-total.csv").unwrap();
    wtr2.write_record(["time", "cells"]).unwrap();

    // parameters for the simulation
    let k: f64 = 0.006;
    let m: f64 = 10_000_000_000_000.0;
    let dt: f64 = 0.001;
    let initial_n: f64 = 1_000_000_000.0;
    let t_final = (1200.0 / dt) as usize;

    let grid_size = 100;
    let iterations = 100;

    // create a grid of cells 100x100 which are f64 and set to 0.0
    let mut grid: Vec<Vec<f64>> = vec![vec![initial_n; grid_size]; grid_size];
    let mut position = (grid_size / 2, grid_size / 2);

    // store the total number of cells in the grid as a vector
    let mut running_total: f64 = 0.0;
    let mut total_cells: Vec<f64> = Vec::new();

    // loop for n iterations
    for _ in 0..iterations {
        // move the cell
        let available_positions = automata::position::available_diagonal(position, &grid);
        let random_position = rand::random::<usize>() % available_positions.len();
        let new_position = available_positions[random_position];

        // get the current number of cells
        let current_n = grid[new_position.0][new_position.1];

        // simulate the growth of the cells
        let n = automata::growth::simulate(k, m, dt, current_n, t_final);
        grid[new_position.0][new_position.1] = n[n.len() - 1];

        // update the grid and position
        position = new_position;

        // store the total number of cells in the grid for each iteration in n
        for cells in n.iter() {
            total_cells.push(running_total + cells - current_n);
        }

        // update the running total
        running_total += n[n.len() - 1] - current_n;

        // write the position to the csv file
        wtr.write_record(&[
            position.0.to_string(),
            position.1.to_string(),
            grid[position.0][position.1].to_string(),
        ])
        .unwrap();
    }

    // write the total number of cells to the csv file
    for (i, cells) in total_cells.iter().enumerate() {
        wtr2.write_record(&[i.to_string(), cells.to_string()])
            .unwrap();
    }
}
