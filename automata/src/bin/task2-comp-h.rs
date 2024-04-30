fn generate(dt: i32) {
    // open a csv file and write the header
    let path = format!("output/bulk-h/task2-1-dt{}.csv", dt);
    let mut wtr = csv::Writer::from_path(path).unwrap();
    wtr.write_record(["time", "cells"]).unwrap();

    // parameters for the simulation
    let k: f64 = 0.006;
    let m: f64 = 10_000_000_000_000.0;
    let dt: f64 = 10.0_f64.powi(dt);
    let initial_n: f64 = 1_000_000_000.0;
    let t_final = (1200.0 / dt) as usize;

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
    // start at 10^10 and increase by 10^9 until 10^15
    let dts = -4..=2;

    for dt in dts {
        generate(dt);
    }
}
