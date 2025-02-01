//! Delay effects, or you can also call it echo effects. Different from the delay filters, the
//! delay effects are optimized for varying delay amount during runtime, and they may have many
//! other features like feedback, mix, etc.

use crate::buffer_view::BufferViewMut;
use crate::effects::Effect;

const MAX_DELAY_TIME: f32 = 1000.0; // ms

const DELAY_TIME_SMOOTHING: f32 = 10.0; // ms

const DEFAULT_DELAY_TIME: f32 = 100.0; // ms
const DEFAULT_FEEDBACK: f32 = 0.2;
const DEFAULT_DRY_GAIN: f32 = 1.0;
const DEFAULT_WET_GAIN: f32 = 0.25; // 25% = -12 dB

/// A simple digital delay effect with feedback and dry/wet gain. Linear interpolation is used for
/// the delay line, and there is no cross-talk between the channels. The channel number is not limited.
///
/// Although it is called digital delay, because of the lowpass characteristics of the linear
/// interpolation, the echoes will get a little darker over time.
pub struct DigitalDelay {
    // Parameters
    sample_rate: f32,
    delay_time: f32,
    feedback: f32,
    dry_gain: f32,
    wet_gain: f32,

    // Dependent parameters
    sample_rate_per_ms: f32,
    /// The delay time in samples.
    delay_samples: f32,
    /// The smoothing factor for the delay time.
    smoothing_factor: f32,
    /// The smoothed delay time in samples.
    smoothed_delay_samples: f32,

    // Internal states
    delay_lines: Vec<Vec<f32>>,
    /// The read index of the delay line.
    read_index: usize,
}

impl Effect for DigitalDelay {
    fn prepare(&mut self, sample_rate: f32, _block_size: usize) {
        assert!(sample_rate > 0.0);
        self.sample_rate = sample_rate;

        // Update the dependent parameters
        self.sample_rate_per_ms = sample_rate / 1000.0;
        self.delay_samples = self.delay_time * self.sample_rate_per_ms;
        self.smoothing_factor = (-1.0 * DELAY_TIME_SMOOTHING * self.sample_rate_per_ms)
            .recip()
            .exp();
        self.smoothed_delay_samples = self.delay_samples;

        // Update the internal states
        self.reset();
        let max_delay_samples = (MAX_DELAY_TIME * self.sample_rate_per_ms).ceil() as usize;
        self.delay_lines.iter_mut().for_each(|channel| {
            channel.resize(max_delay_samples.next_power_of_two(), 0.0);
        });
    }

    fn reset(&mut self) {
        self.smoothed_delay_samples = self.delay_samples;
        self.delay_lines.iter_mut().for_each(|channel| {
            channel.fill(0.0);
        });
        self.read_index = 0;
    }

    // TODO Delay time smoothing

    fn process_inplace<'a>(&mut self, buffer: &'a mut BufferViewMut<'a>) {
        // Check if the effect is prepared
        if self.sample_rate == 0.0 {
            return;
        }

        let num_channels = buffer.num_channels();
        let num_samples = buffer.num_samples();
        let delay_line_len = self.delay_lines[0].len();
        let delay_line_mask = delay_line_len - 1;

        // Iterate over samples
        let channels: &mut [&mut [f32]] = buffer.channels_mut();
        debug_assert_eq!(channels.len(), num_channels);

        for n in 0..num_samples {
            // Smooth the delay time
            self.smoothed_delay_samples = self.delay_samples
                + (self.smoothed_delay_samples - self.delay_samples) * self.smoothing_factor;
            let delay_int = self.smoothed_delay_samples.floor() as usize;
            let delay_frac = self.smoothed_delay_samples - delay_int as f32;
            let write_index1 = (self.read_index + delay_int) & delay_line_mask;
            let write_index2 = (write_index1 + 1) & delay_line_mask;

            // Iterate over each channel
            for (ch, channel) in channels.iter_mut().enumerate() {
                // Read the sample from the delay line
                let y = self.delay_lines[ch][self.read_index];
                self.delay_lines[ch][self.read_index] = 0.0;

                // Write the sample to the delay line
                let x = channel[n] + y * self.feedback;
                self.delay_lines[ch][write_index1] += x * (1.0 - delay_frac);
                self.delay_lines[ch][write_index2] += x * delay_frac;

                // Mix the dry and wet signals
                channel[n] = self.dry_gain * channel[n] + self.wet_gain * y;
            }

            // Update the read index
            self.read_index = (self.read_index + 1) & delay_line_mask;
        }
    }
}

impl DigitalDelay {
    pub fn new(num_channels: usize) -> Self {
        assert!((1..=2).contains(&num_channels));
        Self {
            sample_rate: 0.0,
            delay_time: DEFAULT_DELAY_TIME,
            feedback: DEFAULT_FEEDBACK,
            dry_gain: DEFAULT_DRY_GAIN,
            wet_gain: DEFAULT_WET_GAIN,
            sample_rate_per_ms: 0.0,
            delay_samples: 0.0,
            smoothing_factor: 0.0,
            smoothed_delay_samples: 0.0,
            delay_lines: vec![vec![0.0; 0]; num_channels],
            read_index: 0,
        }
    }

    pub fn set_delay_time(&mut self, delay: f32) {
        assert!(delay > 0.0);
        self.delay_time = delay;
        self.delay_samples = delay * self.sample_rate_per_ms;
    }

    pub fn set_feedback(&mut self, feedback: f32) {
        assert!(feedback >= 0.0);
        self.feedback = feedback;
    }

    pub fn set_dry_gain(&mut self, dry_gain: f32) {
        assert!(dry_gain >= 0.0);
        self.dry_gain = dry_gain;
    }

    pub fn set_wet_gain(&mut self, wet_gain: f32) {
        assert!(wet_gain >= 0.0);
        self.wet_gain = wet_gain;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use crate::assert_all_close;

    #[test]
    fn test_new_delay() {
        let delay = DigitalDelay::new(2);
        assert_eq!(delay.delay_time, DEFAULT_DELAY_TIME);
        assert_eq!(delay.feedback, DEFAULT_FEEDBACK);
        assert_eq!(delay.dry_gain, DEFAULT_DRY_GAIN);
        assert_eq!(delay.wet_gain, DEFAULT_WET_GAIN);
        assert_eq!(delay.delay_lines.len(), 2);
    }

    #[test]
    fn test_parameter_setters() {
        let mut delay = DigitalDelay::new(1);

        delay.set_delay_time(737.0);
        assert_eq!(delay.delay_time, 737.0);

        delay.set_feedback(0.43);
        assert_eq!(delay.feedback, 0.43);

        delay.set_dry_gain(0.29);
        assert_eq!(delay.dry_gain, 0.29);

        delay.set_wet_gain(0.12);
        assert_eq!(delay.wet_gain, 0.12);
    }

    #[test]
    fn test_prepare() {
        let mut delay = DigitalDelay::new(1);
        delay.set_delay_time(100.0);
        delay.prepare(48000.0, 128);

        assert_eq!(delay.sample_rate_per_ms, 48.0);
        // Delay time in samples should be 4800
        assert_eq!(delay.delay_samples, 4800.0);
        assert_eq!(delay.smoothed_delay_samples, 4800.0);

        // Delay line should be power of 2 and large enough
        let min_size = (MAX_DELAY_TIME * 48000.0 / 1000.0).ceil() as usize;
        assert!(delay.delay_lines[0].len() >= min_size);
        assert!(delay.delay_lines[0].len().is_power_of_two());
    }

    #[test]
    fn test_process_dry_only() {
        let mut delay = DigitalDelay::new(1);
        delay.set_wet_gain(0.0);
        delay.set_dry_gain(1.0);
        delay.prepare(48000.0, 128);

        let mut buffer: [f32; 4] = [1.0, 0.5, -0.5, -1.0];
        let mut slices: Vec<&mut [f32]> = vec![&mut buffer];
        let mut view = BufferViewMut::new(&mut slices);
        delay.process_inplace(&mut view);

        // With wet gain = 0, output should equal input
        assert_all_close!(buffer, [1.0, 0.5, -0.5, -1.0]);
    }

    #[test]
    fn test_process_wet_only() {
        let mut delay = DigitalDelay::new(1);
        delay.set_delay_time(11.0);
        delay.set_feedback(0.0);
        delay.set_dry_gain(0.0);
        delay.set_wet_gain(1.0);
        delay.prepare(48000.0, 128);

        let mut buffer: Vec<f32> = vec![1.0, 0.5, -0.5, -1.0];
        buffer.resize(1000, 0.0); // Enough samples to hear the delay
        let mut slices: Vec<&mut [f32]> = vec![&mut buffer];
        let mut view = BufferViewMut::new(&mut slices);
        delay.process_inplace(&mut view);

        // First samples should be zero (dry is 0)
        let expected_delay: usize = 48 * 11;
        for i in 0..expected_delay {
            assert!(buffer[i].abs() < 1e-6, "Expected buffer[{}]: {} to be zero", i, buffer[i]);
        }

        // After delay_time, we should see the signal
        assert_all_close!(buffer[expected_delay..expected_delay + 4], [1.0, 0.5, -0.5, -1.0]);
    }

    #[test]
    fn test_feedback() {
        let delay_time: f32 = 11.0;
        let feedback: f32 = 0.3;

        let mut delay = DigitalDelay::new(1);
        delay.set_delay_time(delay_time);
        delay.set_feedback(feedback);
        delay.set_dry_gain(0.0);
        delay.set_wet_gain(1.0);
        delay.prepare(48000.0, 128);

        // Send an impulse
        let mut buffer: Vec<f32> = vec![1.0];
        buffer.resize(2000, 0.0); // Enough for several echoes
        let mut slices: Vec<&mut [f32]> = vec![&mut buffer];
        let mut view = BufferViewMut::new(&mut slices);
        delay.process_inplace(&mut view);

        // Check the impulse and its echoes
        let expected_delay: usize = 48 * delay_time as usize;
        let mut echo_count = 0;
        for i in 0..buffer.len() {
            if i % expected_delay > 0 || i == 0 {
                assert!(buffer[i].abs() < 1e-6, "Expected buffer[{}]: {} to be zero", i, buffer[i]);
            }
            else {
                let amplitude: f32 = feedback.powi(echo_count);
                assert!((buffer[i] - amplitude).abs() < 1e-6, "Expected buffer[{}]: {} to be {}", i, buffer[i], amplitude);
                echo_count += 1;
            }
        }
    }

    #[test]
    fn test_process_stereo() {
        let delay_time: f32 = 11.0;
        let feedback: f32 = 0.3;

        let mut delay = DigitalDelay::new(2);
        delay.set_delay_time(delay_time);
        delay.set_feedback(feedback);
        delay.set_dry_gain(0.0);
        delay.set_wet_gain(1.0);
        delay.prepare(48000.0, 128);

        // Let right channel be slightly delayed
        let mut buffer: Vec<Vec<f32>> = vec![vec![1.0, 0.0, 0.0], vec![0.0, 0.0, 0.5]];
        buffer.iter_mut().for_each(|channel| {
            channel.resize(2000, 0.0); // Enough samples to hear several echoes
        });
        let mut slices: Vec<&mut [f32]> = buffer.iter_mut().map(|ch| ch.as_mut_slice()).collect();
        let mut view = BufferViewMut::new(&mut slices);
        delay.process_inplace(&mut view);

        // Check the first 3 echoes
        for i in 1..=3 {
            let expected_delay: usize = 48 * delay_time as usize * i;
            let gain: f32 = feedback.powi(i as i32 - 1);
            assert_relative_eq!(buffer[0][expected_delay..expected_delay + 3], [gain * 1.0, 0.0, 0.0]);
            assert_relative_eq!(buffer[1][expected_delay..expected_delay + 3], [0.0, 0.0, gain * 0.5]);
        }
    }
}
