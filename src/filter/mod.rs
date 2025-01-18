//! This module provides various digital filter implementations for audio processing:
//!
//! General filters:
//! - FIR (Finite Impulse Response) filters through [`FirFilter`]
//!
//! Delay filters are filters of which the only purpose is to introduce a delay to the signal.
//! They implement the [`DelayFilter`] trait:
//! - Linear interpolation delay ([`LinearInterpDelay`])
//! - Sinc interpolation delay ([`SincInterpDelay`])
//!
//! All filters implement the [`Filter`] trait which provides a common interface
//! for processing audio samples.

pub mod fir;
pub mod delay;
mod design;

pub use fir::FirFilter;
pub use delay::{
    DelayFilter,
    LinearInterpDelay,
    SincInterpDelay,
};

/// Common interface for digital audio filters.
///
/// This trait defines the basic operations that all filters must implement:
/// - Processing a slice of samples
/// - In-place processing of samples
/// - Resetting the filter state
pub trait Filter {
    /// Process a slice of input samples and return the filtered output.
    ///
    /// # Arguments
    /// * `input` - Slice of input samples to process
    ///
    /// # Returns
    /// A new vector containing the filtered samples
    fn process(&mut self, input: &[f32]) -> Vec<f32>;

    /// Process samples in-place, modifying the input buffer directly.
    ///
    /// # Arguments
    /// * `buffer` - Mutable slice of samples to process and store results in
    fn process_inplace(&mut self, buffer: &mut [f32]);

    /// Reset the filter's internal state.
    ///
    /// This clears any stored sample history or internal buffers,
    /// returning the filter to its initial state.
    fn reset(&mut self);
}
