fn generate(t: f64) {
    // open a csv file and write the header
    let path = format!("output/bulk-t/task2-1-{}.csv", t);
    let mut wtr = csv::Writer::from_path(path).unwrap();
    wtr.write_record(["time", "cells"]).unwrap();

    // parameters for the simulation
    let k: f64 = 0.006;
    let m: f64 = 10_000_000_000_000.0;
    let dt: f64 = 0.001;
    let initial_n: f64 = 1_000_000_000.0;
    let t_final = (t / dt) as usize;

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

fn main() {
    // start at 1000 and got to 2000 at intervals of 100
    let dts = (1000..=2000).step_by(200).map(|x| x as f64);

    for dt in dts {
        generate(dt);
    }
}
