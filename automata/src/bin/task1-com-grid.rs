fn generate(
    start_position: (usize, usize),
    grid_size: usize,
    count: usize,
    iterations: usize,
) {
    // open a csv file and write the header
    let path = format!(
        "output/bulk-g/task1-1-{}.csv",
        grid_size
    );
    let mut wtr = csv::Writer::from_path(path).unwrap();
    wtr.write_record(["length", "time"]).unwrap();

    // run the simulation
    for _ in 0..count {
        let start = std::time::Instant::now();

        let positions = automata::simulate::simulate_full(
            grid_size,
            start_position,
            iterations,
            false,
            true,
        );

        let elapsed = start.elapsed().as_micros();

        // write the length of the path to the csv file
        wtr.write_record(&[positions.len().to_string(), elapsed.to_string()])
            .unwrap();
    }

    // open a csv file and write the header
    let path = format!(
        "output/bulk-g/task1-2-{}.csv",
        grid_size
    );
    let mut wtr = csv::Writer::from_path(path).unwrap();
    wtr.write_record(["length", "time"]).unwrap();
    // run the simulation
    for _ in 0..count {
        let start = std::time::Instant::now();

        let positions = automata::simulate::simulate_full(
            grid_size,
            start_position,
            iterations,
            true,
            true,
        );

        let elapsed = start.elapsed().as_micros();

        // write the length of the path to the csv file
        wtr.write_record(&[positions.len().to_string(), elapsed.to_string()])
            .unwrap();
    }
}

fn main() {
    // parameters for the simulation
    let iterations = 1000000;
    let count = 10000;

    let grid_sizes = vec![(10, (5,5)), (20, (10,10)), (30, (15,15)), (40, (20,20)), (50, (25,25))];

    // array to store threads
    let mut threads = Vec::new();

    // loop over map_positions and run the simulation
    for (grid_size, start_position) in grid_sizes {
        // create a thread to run the simulation
        let thread = std::thread::spawn(move || {
            generate(
                start_position,
                grid_size,
                count,
                iterations,
            );
        });
        threads.push(thread);
    }

    // wait for all threads to finish
    for thread in threads {
        thread.join().unwrap();
    }
}
