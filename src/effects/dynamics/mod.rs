
use crate::buffer_view::{BufferView, BufferViewMut};
use crate::effects::Effect;

const MIN_AMPLITUDE: f32 = 1e-10;

pub struct Compressor {
    num_channels: usize,
    sample_rate: f32,
    block_size: usize,

    threshold: f32,
    ratio: f32,
    attack_ms: f32,
    release_ms: f32,
    linking: f32,
    makeup_gain: f32,

    attack_coeff: f32,
    release_coeff: f32,

    left_gain: f32,
    right_gain: f32,
}

impl Effect for Compressor {
    fn prepare(&mut self, sample_rate: f32, block_size: usize) {
        self.sample_rate = sample_rate;
        self.block_size = block_size;

        let samples_per_ms = sample_rate * 0.001;
        self.attack_coeff = (-1.0 / (self.attack_ms * samples_per_ms)).exp();
        self.release_coeff = (-1.0 / (self.release_ms * samples_per_ms)).exp();
    }

    fn reset(&mut self) {
        self.left_gain = 0.0;
        self.right_gain = 0.0;
    }

    fn process_inplace<'outer, 'inner>(
        &mut self,
        buffer: &'outer mut BufferViewMut<'outer, 'inner>,
    ) {
        // Check if the effect is prepared
        if self.sample_rate == 0.0 {
            return;
        }

        debug_assert!(buffer.num_channels() == self.num_channels);
        let num_channels = buffer.num_channels().clamp(1, 2);
        let num_samples = buffer.num_samples();

        // Iterate over samples
        if num_channels == 1 {
            let channel = buffer.channel_mut(0);
            for sample in channel.iter_mut() {
                let target_gain = self.compute_target_gain(*sample);
                self.left_gain = self.smooth_gain(target_gain, self.left_gain);
                *sample *= 10.0f32.powf(self.left_gain / 20.0);
            }
        } else {
            let channels = buffer.channels_mut();
            for n in 0..num_samples {
                let mut left_target_gain = self.compute_target_gain(channels[0][n]);
                let mut right_target_gain = self.compute_target_gain(channels[1][n]);

                if left_target_gain < right_target_gain {
                    right_target_gain = right_target_gain + self.linking * (left_target_gain - right_target_gain);
                } else {
                    left_target_gain = left_target_gain + self.linking * (right_target_gain - left_target_gain);
                }

                self.left_gain = self.smooth_gain(left_target_gain, self.left_gain);
                channels[0][n] *= 10.0f32.powf((self.left_gain + self.makeup_gain) / 20.0);

                self.right_gain = self.smooth_gain(right_target_gain, self.right_gain);
                channels[1][n] *= 10.0f32.powf((self.right_gain + self.makeup_gain) / 20.0);
            }
        }
    }

    fn process(&mut self, input: BufferView) -> Vec<Vec<f32>> {
        let mut output = vec![vec![0.0; input.num_samples()]; input.num_channels()];
        let mut slices: Vec<&mut [f32]> = output.iter_mut().map(|ch| ch.as_mut_slice()).collect();
        self.process_inplace(&mut BufferViewMut::new(&mut slices));
        output
    }
}

impl Default for Compressor {
    fn default() -> Self {
        Self {
            num_channels: 1,
            sample_rate: 0.0,
            block_size: 0,
            threshold: -12.0,
            ratio: 2.0,
            attack_ms: 5.0,
            release_ms: 50.0,
            linking: 1.0,
            makeup_gain: 0.0,
            attack_coeff: 0.0,
            release_coeff: 0.0,
            left_gain: 0.0,
            right_gain: 0.0,
        }
    }
}

impl Compressor {
    pub fn new(num_channels: usize) -> Self {
        assert!((1..=2).contains(&num_channels), "num_channels must be 1 or 2");
        Self {
            num_channels,
            ..Default::default()
        }
    }

    pub fn set_threshold(&mut self, threshold: f32) {
        self.threshold = threshold;
    }

    pub fn set_ratio(&mut self, ratio: f32) {
        self.ratio = ratio;
    }

    pub fn set_attack(&mut self, attack_ms: f32) {
        self.attack_ms = attack_ms;
    }

    pub fn set_release(&mut self, release_ms: f32) {
        self.release_ms = release_ms;
    }

    pub fn set_linking(&mut self, linking: f32) {
        self.linking = linking;
    }

    pub fn set_makeup_gain(&mut self, makeup_gain: f32) {
        self.makeup_gain = makeup_gain;
    }

    fn compute_target_gain(&self, x: f32) -> f32 {
        debug_assert!(self.ratio > 1.0);

        let level = x.abs().clamp(MIN_AMPLITUDE, 1.0).log10() * 20.0;
        let target_gain = {
            if level > self.threshold {
                (self.threshold - level) * (1.0 - 1.0 / self.ratio)
            } else {
                0.0
            }
        };
        debug_assert!(target_gain <= 0.0);
        target_gain
    }

    fn smooth_gain(&self, target_gain: f32, current_gain: f32) -> f32 {
        if target_gain < current_gain {
            target_gain + self.attack_coeff * (current_gain - target_gain)
        } else {
            target_gain + self.release_coeff * (current_gain - target_gain)
        }
    }
}
