//! Filter design for delay filters.
//!
//! This module provides filter designs that delay the input signal by a given amount of samples,
//! which can be any positive real number.

use log::warn;
use std::cmp::min;

use crate::filter::design::window::{hamming, hann};
use crate::utilities::sinc;
use super::FirCoeffs;

/// The time resolution for the delay time. If the delay is smaller than this value, it is
/// considered to be zero.
const EPSILON: f32 = 1e-6;
/// The maximum sinc half width that can be used for the sinc interpolation filter.
const MAX_SINC_HALF_WIDTH: usize = 32;

/// Delay filter design using linear interpolation.
///
/// Linear interpolation is the simplest way to implement fractional delay, with the cost of
/// attenuation of high frequencies. Also, the group delay is not constant and is greater than the
/// desired delay in the higher frequencies.
///
/// # Arguments
///
/// * `delay` - The desired delay in samples. Can be any positive real number.
///
/// # Returns
///
/// The FIR filter coefficients object.
///
/// # Panics
///
/// * If `delay` is negative.
pub fn linear_interpolation(delay: f32) -> FirCoeffs {
    assert!(delay >= 0.0, "The delay must not be negative");
    if delay < EPSILON {
        return FirCoeffs { b: vec![1.0] };
    }

    let num_taps = delay.ceil() as usize + 1;
    let mut coeffs = vec![0.0; num_taps];

    let n1 = delay.floor() as usize;
    let n2 = n1 + 1;
    let frac = delay - n1 as f32;

    coeffs[n1] = 1.0 - frac;
    coeffs[n2] = frac;

    FirCoeffs { b: coeffs }
}

/// Delay filter design using (windowed) sinc interpolation.
///
/// This delay filter is composed of a normal integer delay (delta function) and a fractional
/// delay filter implemented with sinc function. Although the fractional delay filter loses the
/// linear phase property of the normal sinc lowpass filters, the group delay is still nearly
/// constant in its wide passband. The main drawback is the higher computational complexity.
///
/// If the given delay is an integer, the filter will be reduced to a delta function.
///
/// # Arguments
///
/// * `delay` - The desired delay in samples. Can be any positive real number.
/// * `sinc_half_width` - The half width of the sinc filter in samples. If not provided, a proper
///   value is automatically chosen. Note that because the sinc filter also introduces a delay,
///   `sinc_half_width` must not be greater than `delay + 0.5`.
/// * `window` - The window function to use for the sinc filter. For the available window
///   functions, see [`window`](super::window). The default window is the Hamming window.
///
/// # Returns
///
/// The FIR filter coefficients object.
///
/// # Panics
///
/// * If `delay` is negative.
/// * If `sinc_half_width` is not greater than 0.
pub fn sinc_interpolation(delay: f32, sinc_half_width: Option<usize>, window: Option<&str>) -> FirCoeffs {
    // Check the delay
    assert!(delay >= 0.0, "The delay must not be negative");
    if delay < EPSILON {
        return FirCoeffs { b: vec![1.0] };
    }

    // Factorize the delay
    let nearest_integer_delay = delay.round() as usize;
    let fractional_delay = delay - nearest_integer_delay as f32;

    if fractional_delay.abs() < EPSILON {
        let mut coeffs = vec![0.0; nearest_integer_delay + 1];
        coeffs[nearest_integer_delay] = 1.0;
        return FirCoeffs { b: coeffs };
    }

    // Determine the (half) width of the sinc filter
    let sinc_half_width = sinc_half_width.unwrap_or(
        min(nearest_integer_delay, MAX_SINC_HALF_WIDTH)
    );
    assert!(sinc_half_width > 0, "The half width of the sinc filter must be greater than 0");

    if sinc_half_width > nearest_integer_delay {
        warn!(concat!(
            "The half width of the sinc filter ({}) was too large for the desired delay ({:.3} ",
            "samples), so it was reduced to {}."
        ), sinc_half_width, delay, sinc_half_width);
    }

    // Determine the delay introduced by the delta function (integer delay)
    let delta_delay = nearest_integer_delay - sinc_half_width;

    // Construct the filter coefficients
    let sinc_width = sinc_half_width * 2 + 1;
    let mut coeffs = vec![0.0; delta_delay + sinc_width];
    let sinc_coeffs = sinc_fractional_delay(sinc_half_width, fractional_delay, window);
    coeffs[delta_delay..delta_delay + sinc_width].copy_from_slice(&sinc_coeffs);

    FirCoeffs { b: coeffs }
}

/// Fractional delay filter that introduces a delay of `sinc_half_width + frac_delay` samples.
fn sinc_fractional_delay(sinc_half_width: usize, frac_delay: f32, window: Option<&str>) -> Vec<f32> {
    assert!(frac_delay >= -0.5 && frac_delay <= 0.5, "The fractional delay must be in the range [-0.5, 0.5]");
    let sinc_width = sinc_half_width * 2 + 1;

    // Determine the window function
    let window: &str = window.unwrap_or("hamming");
    let window_coeffs: Vec<f32> = match window {
        "hamming" => hamming(sinc_width, true),
        "hann" => hann(sinc_width, true),
        _ => panic!("Invalid window function: {}", window),
    };

    // Construct the filter coefficients
    let mut coeffs: Vec<f32> = vec![0.0; sinc_width];
    for n in 0..sinc_width {
        let x = n as f32 - sinc_half_width as f32;
        coeffs[n] = sinc(x - frac_delay) * window_coeffs[n];
    }
    coeffs
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        assert_all_close,
        assert_all_eq,
    };

    mod linear_interpolation {
        use super::*;

        #[test]
        fn zero_delay() {
            let delay = 0.0;
            let coeffs = linear_interpolation(delay);
            assert_all_eq!(coeffs.b, [1.0]);
        }

        #[test]
        fn epsilon_delay() {
            let delay = EPSILON / 10.0;
            let coeffs = linear_interpolation(delay);
            assert_all_eq!(coeffs.b, [1.0]);
        }

        #[test]
        #[should_panic]
        fn negative_delay() {
            let delay = -1.0;
            let _ =linear_interpolation(delay);
        }

        #[test]
        fn case_1() {
            let delay = 3.3;
            let coeffs = linear_interpolation(delay);
            assert_all_close!(coeffs.b, [0.0, 0.0, 0.0, 0.7, 0.3]);
        }
    }

    mod sinc_interpolation {
        use super::*;

        #[test]
        fn zero_delay() {
            let delay = 0.0;
            let coeffs = sinc_interpolation(delay, None, None);
            assert_all_eq!(coeffs.b, [1.0]);
        }

        #[test]
        fn epsilon_delay() {
            let delay = EPSILON / 10.0;
            let coeffs = sinc_interpolation(delay, None, None);
            assert_all_eq!(coeffs.b, [1.0]);
        }

        #[test]
        #[should_panic]
        fn negative_delay() {
            let delay = -1.0;
            let _ = sinc_interpolation(delay, None, None);
        }

        #[test]
        #[should_panic]
        fn zero_sinc_width() {
            let delay = 10.7;
            let sinc_half_width: usize = 0;
            let _ = sinc_interpolation(delay, Some(sinc_half_width), None);
        }

        #[test]
        fn case_1() {
            // The filter should be a delta function if the delay is an integer
            let delay = 7.0;
            let coeffs = sinc_interpolation(delay, None, None);
            assert_all_eq!(coeffs.b, [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0]);
        }

        #[test]
        fn case_2() {
            let delay = 10.7;
            let coeffs = sinc_interpolation(delay, None, Some("hamming"));
            let expected: Vec<f32> = vec![
                 0.00192537, -0.00261854,  0.00452946, -0.00798520,  0.01341051,
                -0.02143884,  0.03317408, -0.05088355,  0.08023462, -0.14041957,
                 0.36102816,  0.85839369, -0.19439978,  0.10378838, -0.06564651,
                 0.04378352, -0.02941852,  0.01939705, -0.01230828,  0.00740796,
                -0.00423724,  0.00246600, -0.00182314
            ];
            assert_all_close!(coeffs.b, expected);
        }

        #[test]
        fn case_3() {
            let delay = 20.7;
            let coeffs = sinc_interpolation(delay, Some(11), Some("hamming"));
            let expected: Vec<f32> = vec![
                 0.00000000,  0.00000000,  0.00000000,  0.00000000,  0.00000000,
                 0.00000000,  0.00000000,  0.00000000,  0.00000000,  0.00000000,
                 0.00192537, -0.00261854,  0.00452946, -0.00798520,  0.01341051,
                -0.02143884,  0.03317408, -0.05088355,  0.08023462, -0.14041957,
                 0.36102816,  0.85839369, -0.19439978,  0.10378838, -0.06564651,
                 0.04378352, -0.02941852,  0.01939705, -0.01230828,  0.00740796,
                -0.00423724,  0.00246600, -0.00182314
            ];
            assert_all_close!(coeffs.b, expected, 2e-6);
        }
    }
}
