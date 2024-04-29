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
                    black_box(automata::growth::simulate(k, m, dt, initial_n, t_final));
                });
            },
        );
    }
    group.finish();
}

// performance test for different simulation distances
fn simlate_distance(c: &mut Criterion) {
    let mut group = c.benchmark_group("simulate_distance");

    // create a vector of lengths from 5 to 95 at intervals of 5
    let lengths = (1..=50).step_by(1).collect::<Vec<_>>();

    // generate array of start and end positions in random range 20-80
    let map_positions = {
        let mut map_positions = Vec::new();
        for length in lengths.iter() {
            let start_position = (
                rand::random::<usize>() % 60 + 20,
                rand::random::<usize>() % 60 + 20,
            );
            map_positions.push((start_position, length));
        }
        map_positions
    };

    for (start_position, length) in map_positions.iter() {
        group.bench_with_input(
            criterion::BenchmarkId::new("length", length),
            length,
            |b, &length| {
                b.iter(|| {
                    let grid_size = 100;
                    let iterations = 1000000;
                    let positions =
                        automata::simulate::simulate_distance(grid_size, *start_position, *length as f64, iterations, true, true);
                    black_box(positions);
                });
            },
        );
    }

    group.finish();
}

// performance test for different simulation grid sizes
fn simulate_grid_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("simulate_grid_size");

    // create a vector of grid sizes from 50 to 150 at intervals of 10
    let grid_sizes = (100..=1000).step_by(25).collect::<Vec<_>>();

    for grid_size in grid_sizes.iter() {
        group.bench_with_input(
            criterion::BenchmarkId::new("grid_size", grid_size),
            grid_size,
            |b, &grid_size| {
                // start position is middle of the grid
                let start_position = (grid_size / 2, grid_size / 2);
                let length = 50.0;

                b.iter(|| {
                    let iterations = 1000000;
                    let positions =
                        automata::simulate::simulate_distance(grid_size, start_position, length, iterations, true, true);
                    black_box(positions);
                });
            },
        );
    }

    group.finish();
}

fn move_cell(c: &mut Criterion) {
    let mut group = c.benchmark_group("move_cell");

    let grid_size = 100;
    let mut grid = vec![vec![0.0; grid_size]; grid_size];
    let mut position = (50, 50);

    // test with diagonal on and off
    group.bench_function("diagonal_on", |b| {
        b.iter(|| {
            automata::simulate::move_cell(&mut position, &mut grid, true);
        });
    });

    group.bench_function("diagonal_off", |b| {
        b.iter(|| {
            automata::simulate::move_cell(&mut position, &mut grid, false);
        });
    });

    group.finish();
}

criterion_group!(benches, growth_dt, simlate_distance, simulate_grid_size, move_cell);
criterion_main!(benches);
