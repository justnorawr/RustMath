use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rust_math_mcp::tools::{DefaultToolRegistry, ToolRegistry};
use serde_json::json;

/// Benchmark O(1) tool lookup performance
fn bench_tool_lookup(c: &mut Criterion) {
    let registry = DefaultToolRegistry;

    c.bench_function("tool_lookup_add", |b| {
        let args = json!({"numbers": [1.0, 2.0, 3.0]});
        b.iter(|| {
            registry
                .execute_tool(black_box("add"), black_box(&args))
                .unwrap()
        })
    });

    c.bench_function("tool_lookup_mean", |b| {
        let args = json!({"numbers": [1.0, 2.0, 3.0, 4.0, 5.0]});
        b.iter(|| {
            registry
                .execute_tool(black_box("mean"), black_box(&args))
                .unwrap()
        })
    });

    c.bench_function("tool_lookup_quadratic_formula", |b| {
        let args = json!({"a": 1.0, "b": -5.0, "c": 6.0});
        b.iter(|| {
            registry
                .execute_tool(black_box("quadratic_formula"), black_box(&args))
                .unwrap()
        })
    });
}

/// Benchmark array operations with varying sizes
fn bench_array_operations(c: &mut Criterion) {
    let registry = DefaultToolRegistry;
    let mut group = c.benchmark_group("array_operations");

    for size in [10, 100, 1000, 10000].iter() {
        let numbers: Vec<f64> = (0..*size).map(|i| i as f64).collect();
        let args = json!({"numbers": numbers});

        group.bench_with_input(BenchmarkId::new("add", size), &args, |b, args| {
            b.iter(|| {
                registry
                    .execute_tool(black_box("add"), black_box(args))
                    .unwrap()
            })
        });

        group.bench_with_input(BenchmarkId::new("mean", size), &args, |b, args| {
            b.iter(|| {
                registry
                    .execute_tool(black_box("mean"), black_box(args))
                    .unwrap()
            })
        });

        group.bench_with_input(BenchmarkId::new("std_dev", size), &args, |b, args| {
            b.iter(|| {
                registry
                    .execute_tool(black_box("std_dev"), black_box(args))
                    .unwrap()
            })
        });
    }

    group.finish();
}

/// Benchmark mathematical operations
fn bench_math_operations(c: &mut Criterion) {
    let registry = DefaultToolRegistry;

    c.bench_function("sqrt", |b| {
        let args = json!({"number": 16.0});
        b.iter(|| {
            registry
                .execute_tool(black_box("sqrt"), black_box(&args))
                .unwrap()
        })
    });

    c.bench_function("power", |b| {
        let args = json!({"base": 2.0, "exponent": 10.0});
        b.iter(|| {
            registry
                .execute_tool(black_box("power"), black_box(&args))
                .unwrap()
        })
    });

    c.bench_function("factorial", |b| {
        let args = json!({"n": 10.0});
        b.iter(|| {
            registry
                .execute_tool(black_box("factorial"), black_box(&args))
                .unwrap()
        })
    });
}

/// Benchmark tool listing (should be fast with Arc)
fn bench_tool_listing(c: &mut Criterion) {
    let registry = DefaultToolRegistry;

    c.bench_function("get_all_tools", |b| {
        b.iter(|| black_box(registry.get_all_tools()))
    });
}

criterion_group!(
    benches,
    bench_tool_lookup,
    bench_array_operations,
    bench_math_operations,
    bench_tool_listing
);
criterion_main!(benches);
