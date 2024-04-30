fn main() {
    // open a csv file and write the header
    let mut wtr = csv::Writer::from_path("output/task2-1.csv").unwrap();
    wtr.write_record(["time", "cells"]).unwrap();

    // parameters for the simulation
    let k: f64 = 0.006;
    let m: f64 = 10_000_000_000_000.0;
    let dt: f64 = 0.001;
    let initial_n: f64 = 1_000_000_000.0;
    let t_final = (2000.0 / dt) as usize;

    // initialize an array to store the number of cells
    let mut n: Vec<f64> = vec![0.0; t_final];
    n[0] = initial_n;

    // euler method to solve the differential equation
    n = automata::growth::simulate(k, m, dt, initial_n, t_final, false);

    // write the results to the csv file
    for (i, cells) in n.iter().enumerate() {
        wtr.write_record(&[i.to_string(), cells.to_string()])
            .unwrap();
    }
}
