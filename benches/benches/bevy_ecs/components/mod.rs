use bevy_ecs::{bundle::Bundle, entity::Entity, world::World};
use criterion::*;

mod add_remove_big_sparse_set;
mod add_remove_big_table;
mod add_remove_sparse_set;
mod add_remove_table;
mod add_remove_very_big_table;
mod add_remove;
mod archetype_updates;
mod insert_simple;
mod insert_simple_unbatched;

use archetype_updates::*;
use criterion::{criterion_group, Criterion};

criterion_group!(
    benches,
    add_remove,
    add_remove_big,
    add_remove_very_big,
    insert_simple,
    no_archetypes,
    added_archetypes,
);

fn add_remove(c: &mut Criterion) {
    let mut group = c.benchmark_group("add_remove");
    group.warm_up_time(core::time::Duration::from_millis(500));
    group.measurement_time(core::time::Duration::from_secs(4));
    group.bench_function("table", |b| {
        let mut bench = add_remove_table::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("sparse_set", |b| {
        let mut bench = add_remove_sparse_set::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.finish();
}

fn add_remove_big(c: &mut Criterion) {
    let mut group = c.benchmark_group("add_remove_big");
    group.warm_up_time(core::time::Duration::from_millis(500));
    group.measurement_time(core::time::Duration::from_secs(4));
    group.bench_function("table", |b| {
        let mut bench = add_remove_big_table::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("sparse_set", |b| {
        let mut bench = add_remove_big_sparse_set::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.finish();
}

fn add_remove_very_big(c: &mut Criterion) {
    let mut group = c.benchmark_group("add_remove_very_big");
    group.warm_up_time(core::time::Duration::from_millis(500));
    group.measurement_time(core::time::Duration::from_secs(4));
    group.bench_function("table", |b| {
        let mut bench = add_remove_very_big_table::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.finish();
}

fn insert_simple(c: &mut Criterion) {
    let mut group = c.benchmark_group("insert_simple");
    group.warm_up_time(core::time::Duration::from_millis(500));
    group.measurement_time(core::time::Duration::from_secs(4));
    group.bench_function("base", |b| {
        let mut bench = insert_simple::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.bench_function("unbatched", |b| {
        let mut bench = insert_simple_unbatched::Benchmark::new();
        b.iter(move || bench.run());
    });
    group.finish();
}

pub const ENTITY_COUNT: usize = 10_000;

pub fn make_entities<B>(world: &mut World, bundle: B) -> Vec<Entity>
where
    B: Bundle + Clone,
{
    world
        .spawn_batch(core::iter::repeat(bundle).take(ENTITY_COUNT))
        .collect()
}
