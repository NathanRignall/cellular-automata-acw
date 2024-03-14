fn move_cell(position: &mut (usize, usize), grid: &mut Vec<Vec<f64>>) {
    // move the cell
    let available_positions = automata::position::available_normal(*position, &grid);
    let random_position = rand::random::<usize>() % available_positions.len();
    *position = available_positions[random_position];
    grid[position.0][position.1] = 1.0;
}

fn simulate(grid_size: usize, start_position: (usize, usize), end_position: (usize, usize), max_iterations: usize) -> Vec<(usize, usize)> {
    // create a grid of cells 100x100 which are bool and set to false
    let mut grid = vec![vec![0.0; grid_size]; grid_size];
    let mut position = start_position;
    grid[position.0][position.1] = 1.0;

    // loop for n iterations
    let mut positions = Vec::new();
    for _ in 0..max_iterations {
        // simulate the movement of the cells
        move_cell(&mut position, &mut grid);

        // store the position
        positions.push(position);

        // stop if the cell is at the end position
        if position == end_position {
            break;
        }
    }
    positions
}

fn main() {
    // open a csv file and write the header
    let mut wtr = csv::Writer::from_path("output/task1-1-fill.csv").unwrap();
    wtr.write_record(["x", "y"]).unwrap();

    // parameters for the simulation
    let grid_size = 100;
    let iterations = 1000000;
    let start_position = (10,10);
    let end_position = (90,90);

    // simulate the movement of the cells
    let positions = simulate(grid_size, start_position, end_position, iterations);
    
    // write the positions to the csv file
    for position in positions {
        wtr.write_record(&[position.0.to_string(), position.1.to_string()]).unwrap();
    }
}