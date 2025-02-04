//! Module for audio effects. The interface similar to the one of a filter, but an audio effect
//! accepts multiple channels of input/output and is generally more complex.
//!
//! The effects are designed to provide only the core DSP functionality, the more complex features
//! that are specific to the use cases are not provided here, including but not limited to:
//!
//! - Thread safety: effects are not thread-safe and should be manually protected in multi-threaded
//!   applications.
//! - Parameter range checking
//!
//! Those features are left to the users or higher-level frameworks like `nih-plug`.

use crate::buffer_view::{BufferView, BufferViewMut};

mod delay;

pub use delay::DigitalDelay;

/// An effect is like a module that processes audio signals.
pub trait Effect {
    // TODO Allow setting channel number (with `prepare` or a new method?)
    // TODO Sample rate, block size, channel number getters

    /// Prepare the effect for processing. This method must be called before processing any audio
    /// data. The expensive operations depending on the sample rate and block size, e.g. memory
    /// allocations, should be done here.
    fn prepare(&mut self, sample_rate: f32, block_size: usize);

    /// Reset the effect to its initial state.
    fn reset(&mut self);

    /// Process the input signal and return the output signal.
    fn process(&mut self, input: BufferView) -> Vec<Vec<f32>> {
        let mut output: Vec<Vec<f32>> = input.to_vec();
        let mut output_slices: Vec<&mut [f32]> = output
            .iter_mut()
            .map(|ch| ch.as_mut_slice())
            .collect();
        let mut view = BufferViewMut::new(&mut output_slices);
        self.process_inplace(&mut view);
        output
    }

    /// Process the input signal in place.
    fn process_inplace<'outer, 'inner>(
        &mut self,
        buffer: &'outer mut BufferViewMut<'outer, 'inner>,
    );
}
