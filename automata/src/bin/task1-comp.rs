fn generate(
    start_position: (usize, usize),
    end_position: (usize, usize),
    grid_size: usize,
    count: usize,
    iterations: usize,
    visited: std::sync::Arc<std::sync::Mutex<Vec<(usize, usize)>>>,
    visited_diagonal: std::sync::Arc<std::sync::Mutex<Vec<(usize, usize)>>>,
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
        let positions = automata::simulate::simulate(
            grid_size,
            start_position,
            end_position,
            iterations,
            false,
        );

        // get the lock for visited positions and store the positions
        let mut visited = visited.lock().unwrap();
        for position in positions.iter() {
            visited.push(*position);
        }

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
        let positions =
            automata::simulate::simulate(grid_size, start_position, end_position, iterations, true);

        // get the lock for visited positions and store the positions
        let mut visted_diagonal = visited_diagonal.lock().unwrap();
        for position in positions.iter() {
            visted_diagonal.push(*position);
        }

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

    // mutex for visited positions
    let visited = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let visited_diagonal = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));

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
    wtr.write_record(["x", "y"]).unwrap();

    // get the lock for visited positions and write the positions to the csv file
    let visited = visited.lock().unwrap();
    for position in visited.iter() {
        wtr.write_record(&[position.0.to_string(), position.1.to_string()])
            .unwrap();
    }

    // write the visited positions to a csv file
    let mut wtr = csv::Writer::from_path("output/bulk/task1-visited-diagonal.csv").unwrap();
    wtr.write_record(["x", "y"]).unwrap();

    // get the lock for visited positions and write the positions to the csv file
    let visited = visited_diagonal.lock().unwrap();
    for position in visited.iter() {
        wtr.write_record(&[position.0.to_string(), position.1.to_string()])
            .unwrap();
    }
}
