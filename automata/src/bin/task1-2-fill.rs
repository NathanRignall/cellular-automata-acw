fn main() {
    // open a csv file and write the header
    let mut wtr = csv::Writer::from_path("output/task1-2-fill.csv").unwrap();
    wtr.write_record(["x", "y"]).unwrap();

    // parameters for the simulation
    let grid_size = 100;
    let iterations = 1000000;
    let start_position = (10, 10);
    let end_position = (90, 90);

    // simulate the movement of the cells
    let positions =
        automata::simulate::simulate(grid_size, start_position, end_position, iterations, true);

    // write the positions to the csv file
    for position in positions {
        wtr.write_record(&[position.0.to_string(), position.1.to_string()])
            .unwrap();
    }
}
