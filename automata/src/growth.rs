pub fn simulate(k: f64, m: f64, dt: f64, initial_n: f64, t_final: usize) -> Vec<f64> {
    // initialize an array to store the number of cells
    let mut n: Vec<f64> = vec![0.0; t_final];
    n[0] = initial_n;
    let mut t_current: usize = 0;

    // euler method to solve the differential equation
    for i in 1..t_final {
        t_current = i;

        // perform the euler method
        let dn_dt = k * n[i - 1] * (m / n[i - 1]).ln();
        n[i] = n[i - 1] + dn_dt * dt;

        // exit early if dn_dt is greater than 99.31% of m
        if n[i] >= 0.9931 * m {
            // println!("Early exit at t = {}", t_current);
            break;
        }
    }
    
    // slice the array to the current time
    n.truncate(t_current);
    n
}

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let k: f64 = 0.006;
        let m: f64 = 10_000_000_000_000.0;
        let dt: f64 = 0.001;
        let initial_n: f64 = 1_000_000_000.0;
        let t_final = (1200.0 / dt) as usize;
        let n = simulate(k, m, dt, initial_n, t_final);
        assert_eq!(n.len(), t_final);
    }
}