use criterion::{black_box, BatchSize, Criterion};
use criterion::{criterion_group, criterion_main};
use std::collections::HashMap;

use cellular_automata_video_benchmarks::ivec3::IVec3;
use cellular_automata_video_benchmarks::util;
pub const SIZE: usize = 10;

fn hashmap_iter(values_hashmap: HashMap<IVec3, u8>) {
    for x in values_hashmap.iter() {
        let _ = black_box(x);
    }
}

fn array_3d_iter(values_array: [[[u8; SIZE]; SIZE]; SIZE]) {
    for x in values_array.iter() {
        let _ = black_box(x);
    }
}

fn vec_flat_iter(flat_vec: [u8; SIZE * SIZE * SIZE]) {
    for x in flat_vec.iter() {
        let _ = black_box(x);
    }
}

fn hashmap_iter_by_index(values_hashmap: HashMap<IVec3, u8>) {
    for z in 0..SIZE as i32 {
        for y in 0..SIZE as i32 {
            for x in 0..SIZE as i32 {
                let _ = black_box(values_hashmap[&IVec3(x, y, z)]);
            }
        }
    }
}

fn array_3d_iter_by_index(values_array: [[[u8; SIZE]; SIZE]; SIZE]) {
    for z in 0..SIZE {
        for y in 0..SIZE {
            for x in 0..SIZE {
                let _ = black_box(values_array[x][y][z]);
            }
        }
    }
}

fn vec_flat_iter_by_index(flat_vec: [u8; SIZE * SIZE * SIZE]) {
    for z in 0..SIZE {
        for y in 0..SIZE {
            for x in 0..SIZE {
                let index = x + y * SIZE + z * SIZE * SIZE;
                let _ = black_box(flat_vec[index]);
            }
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("hashmap_VS_vec");

    // BY ITER
    group.bench_function("hashmap_iter", |b| {
        b.iter_batched(
            || util::prepare_hashmap(SIZE),
            |values_hashmap| {
                hashmap_iter(values_hashmap);
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("3d_array_iter", |b| {
        b.iter_batched(
            || [[[0u8; SIZE]; SIZE]; SIZE],
            |values_array| {
                array_3d_iter(values_array);
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("vec_flat_iter", |b| {
        b.iter_batched(
            || [0u8; SIZE * SIZE * SIZE],
            |vec_data| {
                vec_flat_iter(vec_data);
            },
            BatchSize::SmallInput,
        )
    });

    // BY index
    group.bench_function("hashmap_iter_by_index", |b| {
        b.iter_batched(
            || util::prepare_hashmap(SIZE),
            |values_hashmap| {
                hashmap_iter_by_index(values_hashmap);
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("array_3d_iter_by_index", |b| {
        b.iter_batched(
            || [[[0u8; SIZE]; SIZE]; SIZE],
            |values_array| {
                array_3d_iter_by_index(values_array);
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("vec_flat_iter_by_index", |b| {
        b.iter_batched(
            || [0u8; SIZE * SIZE * SIZE],
            |vec_data| {
                vec_flat_iter_by_index(vec_data);
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
