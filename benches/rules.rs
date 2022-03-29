use cellular_automata_video_benchmarks::rule::Rule;
use criterion::{black_box, BatchSize, Criterion};
use criterion::{criterion_group, criterion_main};

pub const SIZE: usize = 10;

fn rule_variant(rule: Rule) {
    let neighbour_count = 5;
    for _z in 0..SIZE as i32 {
        for _y in 0..SIZE as i32 {
            for _x in 0..SIZE as i32 {
                let _rule_applies: bool = black_box(rule.in_range(neighbour_count));
            }
        }
    }
}

fn rule_array(rule_array: [bool; 27]) {
    let neighbour_count = 5;
    for _z in 0..SIZE {
        for _y in 0..SIZE {
            for _x in 0..SIZE {
                let _rule_applies: bool = black_box(rule_array[neighbour_count]);
            }
        }
    }
}

// I wanted to see the cost of wrapping the same fast method inside an enum
#[allow(dead_code)]
#[derive(Clone)]
pub enum RuleOptimizedData {
    Variant1([bool; 27]),
}

impl RuleOptimizedData {
    pub fn in_range(&self, value: u8) -> bool {
        match self {
            Self::Variant1(values) => values[value as usize],
        }
    }
}

// same as using array but the array is stored inside an enum variant
// I'm surprised this is just as slow as the first enum method
fn rule_wrapped_array(rule: RuleOptimizedData) {
    let neighbour_count = 5;
    for _z in 0..SIZE as i32 {
        for _y in 0..SIZE as i32 {
            for _x in 0..SIZE as i32 {
                let _rule_applies: bool = black_box(rule.in_range(neighbour_count));
            }
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("rules");

    group.bench_function("rule_variant", |b| {
        b.iter_batched(
            || Rule::Single(7),
            |values_hashmap| {
                rule_variant(values_hashmap);
            },
            BatchSize::SmallInput,
        )
    });

    #[rustfmt::skip]
    group.bench_function("rule_array", |b| {
        b.iter_batched(
            || {
                // beautiful
                [true,true,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false]
            },
            |vec_data| {
                rule_array(vec_data);
            },
            BatchSize::SmallInput,
        )
    });

    #[rustfmt::skip]
    group.bench_function("rule_wrapped_array", |b| {
        b.iter_batched(
            || {
                // beautiful
                RuleOptimizedData::Variant1([true,true,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false,false])
            },
            |vec_data| {
                rule_wrapped_array(vec_data);
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
