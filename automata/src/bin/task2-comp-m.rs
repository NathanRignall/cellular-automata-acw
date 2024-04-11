fn generate(power: i32) {
    // open a csv file and write the header
    let path = format!("output/bulk-m/task2-1-{}.csv", power);
    let mut wtr = csv::Writer::from_path(path).unwrap();
    wtr.write_record(["time", "cells"]).unwrap();

    // parameters for the simulation
    let k: f64 = 0.006;
    let m: f64 = 10.0_f64.powi(power);
    let dt: f64 = 0.001;
    let initial_n: f64 = 1_000_000_000.0;
    let t_final = (1200.0 / dt) as usize;

    // intitalize an array to store the number of cells
    let mut n: Vec<f64> = vec![0.0; t_final];
    n[0] = initial_n;

    // euler method to solve the differential equation
    n = automata::growth::simulate(k, m, dt, initial_n, t_final);

    // write the results to the csv file
    for (i, cells) in n.iter().enumerate() {
        wtr.write_record(&[i.to_string(), cells.to_string()])
            .unwrap();
    }
}

fn main() {
    // start at 10^10 and increase by 10^9 until 10^15
    let powers = 9..=17;

    for m in powers {
        generate(m);
    }
}
