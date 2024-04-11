use automata::growth::simulate;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

// performance test for different values of dt
fn growth_dt(c: &mut Criterion) {
    let plot_config = criterion::PlotConfiguration::default().summary_scale(criterion::AxisScale::Logarithmic);

    let mut group = c.benchmark_group("growth_dt");
    group.plot_config(plot_config);

    // iterate over different values of dt from 1 to 0.001 at steps of division by 2

    for dt in [1.0, 0.5, 0.25, 0.125, 0.0625, 0.03125, 0.015625, 0.0078125, 0.00390625, 0.001953125].iter() {
        group.bench_with_input(
            criterion::BenchmarkId::new("dt", dt),
            dt,
            |b, &dt| {
                b.iter(|| {
                    let k: f64 = 0.006;
                    let m: f64 = 10_000_000_000_000.0;
                    let initial_n: f64 = 1_000_000_000.0;
                    let t_final = (1200.0 / dt) as usize;
                    black_box(simulate(k, m, dt, initial_n, t_final));
                });
            },
        );
    

    }
    group.finish();
}

criterion_group!(benches, growth_dt);
criterion_main!(benches);