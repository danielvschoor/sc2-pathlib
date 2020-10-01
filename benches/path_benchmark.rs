use criterion::{black_box, criterion_group, criterion_main, Criterion};
use common::get_pathfind;
pub mod common;

fn bench_astar_automaton(c: &mut Criterion) {
    let path_find = get_pathfind("tests/AutomatonLE.txt");
    c.bench_function("find_path_automaton", |b| {
        b.iter(|| {
            path_find.find_path((32, 51), (150, 118), Some(0));
        })
    });
}

fn bench_astar_4x4(c: &mut Criterion) {
    let path_find = get_pathfind("tests/maze4x4.txt");
    // Run bench
    c.bench_function("find_path_4x4", |b| {
        b.iter(|| {
            path_find.find_path((0, 0), (0, 2), Some(0));
        })
    });
}


fn bench_astar_10x10(c: &mut Criterion) {
    let path_find = get_pathfind("tests/empty10x10.txt");
    // Run bench
    c.bench_function("find_path_10x10", |b| {
        b.iter(|| {
            path_find.find_path((0, 0), (8, 9), Some(0));
        })});
}

pub fn bench_norm_influence(c: &mut Criterion) {
    let mut path_find = get_pathfind("tests/AutomatonLE.txt");
    c.bench_function("norm_influence", |b| {
        b.iter(|| {
            path_find.normalize_influence(10)
        })
    });
}
pub fn bench_add_influence(c: &mut Criterion) {
    let mut path_find = get_pathfind("tests/AutomatonLE.txt");
    let mut positions: Vec<(usize, usize)> = Vec::with_capacity(100*100);
    for y in 0..100{
        for x in 0..100{
            positions.push((x,y))
        }
    }
    c.bench_function("add_influence", |b| {
        b.iter(|| {
            path_find.add_influence(positions.clone(),10.0, 10.0);

        })
    });
    c.bench_function("add_influence_flat", |b| {
        b.iter(|| {
            path_find.add_influence_flat(positions.clone(),10.0, 10.0);
        })
    });
    c.bench_function("reset_map", |b| {
        b.iter(|| {
            path_find.reset();
        })
    });
}

criterion_group!(benches, bench_astar_automaton, bench_astar_4x4, bench_astar_10x10, bench_norm_influence, bench_add_influence);
criterion_main!(benches);
