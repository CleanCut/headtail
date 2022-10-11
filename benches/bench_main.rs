use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    benchmarks::basic::no_args,
    benchmarks::basic::head_only,
    benchmarks::basic::tail_only,
}
