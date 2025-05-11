use criterion::{
    criterion_group,
    criterion_main,
    Criterion,
    BenchmarkId,
    Throughput,
};
use rand::prelude::*;

use ruadio::buffer_view::BufferView;
use ruadio::effects::{
    Effect,
    DigitalDelay,
};

const SAMPLE_RATE: f32 = 48000.0;
const NUM_SAMPLES: [usize; 5] = [128, 256, 512, 1024, 2048];

fn create_bench_data(num_channels: usize, num_samples: usize) -> Vec<Vec<f32>> {
    let mut rng = rand::rng();
    (0..num_channels)
        .map(|_| (0..num_samples).map(|_| rng.random_range(-1.0..=1.0)).collect())
        .collect()
}

fn bench_digital_delay(c: &mut Criterion) {
    let num_channels = 2;

    let mut group = c.benchmark_group("DigitalDelay");
    for num_samples in NUM_SAMPLES {
        let mut delay = DigitalDelay::new(num_channels);
        delay.prepare(SAMPLE_RATE, num_samples);
        delay.set_delay_time(5.0); // ms

        let data: Vec<Vec<f32>> = create_bench_data(num_channels, num_samples);
        let slices: Vec<&[f32]> = data
            .iter()
            .map(|ch| ch.as_slice())
            .collect();

        group.throughput(Throughput::Elements(num_samples as u64));
        group.bench_with_input(
            BenchmarkId::new("process", num_samples),
            &slices,
            |b, slices| {
                b.iter(|| delay.process(BufferView::new(slices)))
            },
        );
    }
}


criterion_group!(benches, bench_digital_delay);
criterion_main!(benches);

