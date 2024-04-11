fn move_cell(position: &mut (usize, usize), grid: &mut Vec<Vec<f64>>, diagonal: bool) {
    // move the cell
    let available_positions = {
        if diagonal {
            automata::position::available_diagonal(*position, grid)
        } else {
            automata::position::available_normal(*position, grid)
        }
    };
    let random_position = rand::random::<usize>() % available_positions.len();
    *position = available_positions[random_position];
    grid[position.0][position.1] = 1.0;
}

fn simulate(
    grid_size: usize,
    start_position: (usize, usize),
    end_position: (usize, usize),
    max_iterations: usize,
    diagonal: bool,
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
        if position == end_position {
            break;
        }
    }
    positions
}

fn generate(
    start_position: (usize, usize),
    end_position: (usize, usize),
    grid_size: usize,
    count: usize,
    iterations: usize,
) {
    // open a csv file and write the header
    let path = format!(
        "output/bulk/task1-1-{}-{}--{}-{}.csv",
        start_position.0, start_position.1, end_position.0, end_position.1
    );
    let mut wtr = csv::Writer::from_path(path).unwrap();
    wtr.write_record(["length"]).unwrap();

    // run the simulation
    for _ in 0..count {
        let positions = simulate(grid_size, start_position, end_position, iterations, false);

        // write the length of the path to the csv file
        wtr.write_record(&[positions.len().to_string()]).unwrap();
    }

    // open a csv file and write the header
    let path = format!(
        "output/bulk/task1-2-{}-{}--{}-{}.csv",
        start_position.0, start_position.1, end_position.0, end_position.1
    );
    let mut wtr = csv::Writer::from_path(path).unwrap();
    wtr.write_record(["length"]).unwrap();

    // run the simulation
    for _ in 0..count {
        let positions = simulate(grid_size, start_position, end_position, iterations, true);

        // write the length of the path to the csv file
        wtr.write_record(&[positions.len().to_string()]).unwrap();
    }
}

fn main() {
    // parameters for the simulation
    let grid_size = 100;
    let iterations = 1000000;
    let count = 100000;

    // generate array of start and end positions in random range 20-80 (100)
    let map_positions = {
        let mut map_positions = Vec::new();
        for _ in 0..100 {
            let start_position = (
                rand::random::<usize>() % 60 + 20,
                rand::random::<usize>() % 60 + 20,
            );
            let end_position = (
                rand::random::<usize>() % 60 + 20,
                rand::random::<usize>() % 60 + 20,
            );
            map_positions.push((start_position, end_position));
        }
        map_positions
    };

    // array to store threads
    let mut threads = Vec::new();

    // loop over map_positions and run the simulation
    for (start_position, end_position) in map_positions {
        // create a thread to run the simulation
        let thread = std::thread::spawn(move || {
            generate(start_position, end_position, grid_size, count, iterations);
        });
        threads.push(thread);
    }

    // wait for all threads to finish
    for thread in threads {
        thread.join().unwrap();
    }
}
