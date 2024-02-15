fn main() {
    // open a csv file and write the header
    let mut wtr = csv::Writer::from_path("output/task2-1.csv").unwrap();
    wtr.write_record(&["time", "cells"]).unwrap();

    // parameters for the simulation
    let k: f64 = 0.006;
    let m: f64 = 10_000_000_000_000.0;
    let initial_n: f64 = 1_000_000_000.0;
    let t_final = 1200;
    let dt = 1;

    // intitalize an array to store the number of cells
    let mut n: Vec<f64> = vec![0.0; t_final as usize];
    n[0] = initial_n;

    // euler method to solve the differential equation
    for i in 1..t_final {
        let dn_dt = k * n[i - 1] * (m / n[i -1]).ln();
        n[i] = n[i - 1] + dn_dt * dt as f64;

        // write the time to the csv file
        wtr.write_record(&[i.to_string(), n[i].to_string()]).unwrap();
    }

}