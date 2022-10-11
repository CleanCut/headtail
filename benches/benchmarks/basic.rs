use criterion::{black_box, criterion_group, Criterion};
use headtail::{headtail, opts::Opts};

pub fn c1(c: &mut Criterion) {
    let o = Opts {
        filename: Some(String::from("tests/files/input.txt")),
        head: 10,
        tail: 10,
        follow: false,
        sleep_interval: 0.025,
        outfile: None,
    };
    c.bench_function("no args", |b| b.iter(|| headtail(black_box(&o))));
}
criterion_group!(no_args, c1);

pub fn c2(c: &mut Criterion) {
    let o = Opts {
        filename: Some(String::from("tests/files/input.txt")),
        head: 10,
        tail: 0,
        follow: false,
        sleep_interval: 0.025,
        outfile: None,
    };
    c.bench_function("head only", |b| b.iter(|| headtail(black_box(&o))));
}
criterion_group!(head_only, c2);

pub fn c3(c: &mut Criterion) {
    let o = Opts {
        filename: Some(String::from("tests/files/input.txt")),
        head: 0,
        tail: 10,
        follow: false,
        sleep_interval: 0.025,
        outfile: None,
    };
    c.bench_function("tail only", |b| b.iter(|| headtail(black_box(&o))));
}
criterion_group!(tail_only, c3);
