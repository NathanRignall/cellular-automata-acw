pub fn simulate(k: f64, m: f64, dt: f64, initial_n: f64, t_final: usize) -> Vec<f64> {
    // intitalize an array to store the number of cells
    let mut n: Vec<f64> = vec![0.0; t_final];
    n[0] = initial_n;

    // euler method to solve the differential equation
    for i in 1..t_final {
        let dn_dt = k * n[i - 1] * (m / n[i -1]).ln();
        n[i] = n[i - 1] + dn_dt * dt;
    }
    n
}


extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() {
        let k: f64 = 0.006;
        let m: f64 = 10_000_000_000_000.0;
        let dt: f64 = 0.001;
        let initial_n: f64 = 1_000_000_000.0;
        let t_final = (1200.0 / dt )as usize;
        let n = simulate(k, m, dt, initial_n, t_final);
        assert_eq!(n.len(), t_final);
    }

    #[bench]
    fn bench_growth(b: &mut Bencher) {
        b.iter(|| {
            _ = simulate(0.006, 10_000_000_000_000.0, 0.001, 1_000_000_000.0, (1200.0 / 0.001 )as usize);
        });
    }

    #[bench]
    fn bench_xor_1000_ints(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(1000);

            let k: f64 = 0.006;
            let m: f64 = 10_000_000_000_000.0;
            let dt: f64 = 0.001;
            let initial_n: f64 = 1_000_000_000.0;
            let t_final = (1200.0 / dt )as usize;

            // run the simulation 1000 times using n
            (0..n).for_each(|_| {
                simulate(k, m, dt, initial_n, t_final);
            });
        });
    }
}