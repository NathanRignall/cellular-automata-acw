fn main() {
    // open a csv file and write the header
    let mut wtr = csv::Writer::from_path("output/task1-2.csv").unwrap();
    wtr.write_record(["x", "y"]).unwrap();

    // parameters for the simulation
    let grid_size = 100;
    let iterations = 100;

    // create a grid of cells 100x100 which are bool and set to false
    let mut grid = vec![vec![0.0; grid_size]; grid_size];
    let mut position = (grid_size / 2, grid_size / 2);
    grid[position.0][position.1] = 1.0;

    // loop for n iterations
    for _ in 0..iterations {
        // simulate the movement of the cells
        automata::simulate::move_cell(&mut position, &mut grid, true);

        // write the position to the csv file
        wtr.write_record(&[position.0.to_string(), position.1.to_string()])
            .unwrap();
    }
}
