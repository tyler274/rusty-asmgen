mod common;
use criterion::{criterion_group, criterion_main, Criterion, SamplingMode};
use std::path::Path;

use common::{clang_compile, record_asm_bin_output, write_assembly};

// pub fn bench_stage3(c: &mut Criterion) {
//     let program_path = Path::new("progs/stage3-lots-of-ops.bas");
//     write_assembly(program_path).ok();
//     clang_compile(program_path).ok();
//     let mut group = c.benchmark_group("stage3");
//     group.sampling_mode(SamplingMode::Flat).sample_size(1000);
//     group.bench_function(
//         format!(
//             "{} execution time",
//             program_path.file_name().unwrap().to_str().unwrap()
//         ),
//         |b| b.iter(|| record_asm_bin_output(program_path).ok()),
//     );
// }

// pub fn bench_stage4(c: &mut Criterion) {
//     let program_path = Path::new("progs/stage4-lots-of-ops.bas");
//     write_assembly(program_path).ok();
//     clang_compile(program_path).ok();
//     let mut group = c.benchmark_group("stage4");
//     group.sampling_mode(SamplingMode::Flat).sample_size(1000);
//     group.bench_function(
//         format!(
//             "{} execution time",
//             program_path.file_name().unwrap().to_str().unwrap()
//         ),
//         |b| b.iter(|| record_asm_bin_output(program_path).ok()),
//     );
// }

// pub fn bench_stage5(c: &mut Criterion) {
//     let program_path = Path::new("progs/stage5-lots-of-ops.bas");
//     write_assembly(program_path).ok();
//     clang_compile(program_path).ok();
//     let mut group = c.benchmark_group("stage5");
//     group.sampling_mode(SamplingMode::Flat).sample_size(1000);
//     group.bench_function(
//         format!(
//             "{} execution time",
//             program_path.file_name().unwrap().to_str().unwrap()
//         ),
//         |b| b.iter(|| record_asm_bin_output(program_path).ok()),
//     );
// }

// pub fn bench_stage6(c: &mut Criterion) {
//     let program_path = Path::new("progs/stage6-nested.bas");
//     write_assembly(program_path).ok();
//     clang_compile(program_path).ok();
//     let mut group = c.benchmark_group("stage6");
//     group.sampling_mode(SamplingMode::Flat).sample_size(1000);
//     group.bench_function("loops_of_ops execution time", |b| {
//         b.iter(|| record_asm_bin_output(program_path).ok())
//     });
// }

pub fn bench_loops(c: &mut Criterion) {
    let program_path = Path::new("progs/stage7-loops-of-ops.bas");
    write_assembly(program_path).ok();
    clang_compile(program_path).ok();
    let mut group = c.benchmark_group("loops");
    group.sampling_mode(SamplingMode::Flat).sample_size(1000);
    group.bench_function("loops_of_ops execution time", |b| {
        b.iter(|| record_asm_bin_output(program_path).ok())
    });
}

pub fn bench_unhash(c: &mut Criterion) {
    let program_path = Path::new("progs/stage7-unhash.bas");
    write_assembly(program_path).ok();
    clang_compile(program_path).ok();
    let mut group = c.benchmark_group("unhash");
    group.sampling_mode(SamplingMode::Flat).sample_size(1000);
    group.bench_function("unhash execution time", |b| {
        b.iter(|| record_asm_bin_output(program_path).ok())
    });
}

criterion_group!(benches, bench_loops, bench_unhash);
criterion_main!(benches);
