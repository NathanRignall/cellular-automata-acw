fn generate(
    start_position: (usize, usize),
    end_position: (usize, usize),
    grid_size: usize,
    count: usize,
    iterations: usize,
    visited: std::sync::Arc<std::sync::Mutex<Vec<Vec<u64>>>>,
    visted_diagonal: std::sync::Arc<std::sync::Mutex<Vec<Vec<u64>>>>,
) {
    // open a csv file and write the header
    let path = format!(
        "output/bulk/task1-1-{}-{}--{}-{}.csv",
        start_position.0, start_position.1, end_position.0, end_position.1
    );
    let mut wtr = csv::Writer::from_path(path).unwrap();
    wtr.write_record(["length", "time"]).unwrap();

    // run the simulation
    for _ in 0..count {
        let start = std::time::Instant::now();

        let positions = automata::simulate::simulate(
            grid_size,
            start_position,
            end_position,
            iterations,
            false,
            true,
        );

        let elapsed = start.elapsed().as_micros();

        // write the length of the path to the csv file
        wtr.write_record(&[positions.len().to_string(), elapsed.to_string()])
            .unwrap();

        // put the visited positions in the visited grid
        let mut visited = visited.lock().unwrap();
        for (x, y) in positions {
            if x < grid_size && y < grid_size {
                visited[x][y] += 1;
            } else {
                println!("Out of bounds: {}, {}", x, y);
            }
        }
    }

    // open a csv file and write the header
    let path = format!(
        "output/bulk/task1-2-{}-{}--{}-{}.csv",
        start_position.0, start_position.1, end_position.0, end_position.1
    );
    let mut wtr = csv::Writer::from_path(path).unwrap();
    wtr.write_record(["length", "time"]).unwrap();
    // run the simulation
    for _ in 0..count {
        let start = std::time::Instant::now();

        let positions = automata::simulate::simulate(
            grid_size,
            start_position,
            end_position,
            iterations,
            true,
            true,
        );

        let elapsed = start.elapsed().as_micros();

        // write the length of the path to the csv file
        wtr.write_record(&[positions.len().to_string(), elapsed.to_string()])
            .unwrap();

        // put the visited positions in the visited grid
        let mut visited = visted_diagonal.lock().unwrap();
        for (x, y) in positions {
            if x < grid_size && y < grid_size {
                visited[x][y] += 1;
            } else {
                println!("Out of bounds: {}, {}", x, y);
            }
        }
    }
}

fn main() {
    // parameters for the simulation
    let grid_size = 100;
    let iterations = 1000000;
    let count = 10000;

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

    // create grid to store visited positions
    let visited_grid: Vec<Vec<u64>> = vec![vec![0; grid_size]; grid_size];
    let visited_diagonal_grid: Vec<Vec<u64>> = vec![vec![0; grid_size]; grid_size];

    // put grids in mutex
    let visited = std::sync::Arc::new(std::sync::Mutex::new(visited_grid));
    let visited_diagonal = std::sync::Arc::new(std::sync::Mutex::new(visited_diagonal_grid));

    // loop over map_positions and run the simulation
    for (start_position, end_position) in map_positions {
        let visited = visited.clone();
        let visited_diagonal = visited_diagonal.clone();

        // create a thread to run the simulation
        let thread = std::thread::spawn(move || {
            generate(
                start_position,
                end_position,
                grid_size,
                count,
                iterations,
                visited,
                visited_diagonal,
            );
        });
        threads.push(thread);
    }

    // wait for all threads to finish
    for thread in threads {
        thread.join().unwrap();
    }

    // write the visited positions to a csv file
    let mut wtr = csv::Writer::from_path("output/bulk/task1-visited.csv").unwrap();
    wtr.write_record(["x", "y", "count"]).unwrap();

    // get the lock for visited positions and write the positions to the csv file
    let visited = visited.lock().unwrap();
    for (x, row) in visited.iter().enumerate() {
        for (y, &value) in row.iter().enumerate() {
            if value > 0 {
                wtr.write_record(&[x.to_string(), y.to_string(), value.to_string()])
                    .unwrap();
            }
        }
    }

    // close the csv file
    wtr.flush().unwrap();

    // write the visited positions to a csv file
    let mut wtr = csv::Writer::from_path("output/bulk/task1-visited-diagonal.csv").unwrap();
    wtr.write_record(["x", "y", "count"]).unwrap();

    // get the lock for visited positions and write the positions to the csv file
    let visited = visited_diagonal.lock().unwrap();
    for (x, row) in visited.iter().enumerate() {
        for (y, &value) in row.iter().enumerate() {
            if value > 0 {
                wtr.write_record(&[x.to_string(), y.to_string(), value.to_string()])
                    .unwrap();
            }
        }
    }

    // close the csv file
    wtr.flush().unwrap();
}
