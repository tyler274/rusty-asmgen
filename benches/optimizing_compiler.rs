mod common;
use criterion::{criterion_group, criterion_main, Criterion, SamplingMode};
use std::path::Path;

use common::{clang_compile, record_asm_bin_output, write_assembly};

pub fn bench_lots_of_ops(c: &mut Criterion) {
    let program_path = Path::new("progs/stage4-lots-of-ops.bas");
    write_assembly(program_path).ok();
    clang_compile(program_path).ok();
    let mut group = c.benchmark_group("stage4");
    group.sampling_mode(SamplingMode::Flat);
    group.bench_function(
        format!(
            "{} execution time",
            program_path.file_name().unwrap().to_str().unwrap()
        ),
        |b| b.iter(|| record_asm_bin_output(program_path).ok()),
    );
}

pub fn bench_loops_of_ops(c: &mut Criterion) {
    let program_path = Path::new("progs/stage7-loops-of-ops.bas");
    write_assembly(program_path).ok();
    clang_compile(program_path).ok();
    let mut group = c.benchmark_group("stage7");
    group.sampling_mode(SamplingMode::Flat);
    group.bench_function("loops_of_ops execution time", |b| {
        b.iter(|| record_asm_bin_output(program_path).ok())
    });
}

criterion_group!(benches, bench_lots_of_ops, bench_loops_of_ops);
criterion_main!(benches);
