use cache_locality_lab::experiments::matrix_blocking::MatrixBlockingExperiment;
use cache_locality_lab::experiments::pointer_layout::PointerLayoutExperiment;
use cache_locality_lab::experiments::simd_scan::SimdScanExperiment;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

fn bench_pointer_layout(c: &mut Criterion) {
    let experiment = PointerLayoutExperiment::default();
    experiment.verify();

    let mut group = c.benchmark_group("pointer_layout");

    for case in experiment.cases() {
        group.throughput(Throughput::Elements(case.work_units));
        group.bench_function(
            BenchmarkId::new(case.variant.name(), experiment.benchmark_input_label()),
            |b| {
                b.iter(|| black_box(experiment.run(case.variant)));
            },
        );
    }

    group.finish();
}

fn bench_matrix_blocking(c: &mut Criterion) {
    let experiment = MatrixBlockingExperiment::default();
    experiment.verify();

    let mut group = c.benchmark_group("matrix_blocking");

    for case in experiment.cases() {
        let mut output = vec![0.0_f32; experiment.output_len()];
        group.throughput(Throughput::Elements(case.work_units));
        group.bench_function(
            BenchmarkId::new(case.variant.name(), experiment.benchmark_input_label()),
            |b| {
                b.iter(|| {
                    let checksum = experiment.run(case.variant, &mut output);
                    black_box(checksum);
                });
            },
        );
    }

    group.finish();
}

fn bench_simd_scan(c: &mut Criterion) {
    let experiment = SimdScanExperiment::default();
    experiment.verify();

    let mut group = c.benchmark_group("simd_scan");

    for case in experiment.cases() {
        group.throughput(Throughput::Elements(case.work_units));
        group.bench_function(
            BenchmarkId::new(case.variant.name(), experiment.benchmark_input_label()),
            |b| {
                b.iter(|| black_box(experiment.run(case.variant)));
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_pointer_layout,
    bench_matrix_blocking,
    bench_simd_scan
);
criterion_main!(benches);
