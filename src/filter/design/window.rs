use std::f32::consts::PI;

/// Return the coefficients of a Hamming window.
///
/// # Arguments
///
/// * `n` - The number of points in the window.
/// * `sym` - Whether the window is symmetric. If not symmetric, the window will be periodic.
pub fn hamming(n: usize, sym: bool) -> Vec<f32> {
    let a0 = 0.54;
    let a1 = 0.46;
    let two_pi = 2.0 * PI;
    let denom: f32 = if sym {
        n as f32 - 1.0
    } else {
        n as f32
    };
    (0..n)
        .map(|i| a0 - a1 * (two_pi * i as f32 / denom).cos())
        .collect()
}

/// Return the coefficients of a Hann window.
pub fn hann(n: usize, sym: bool) -> Vec<f32> {
    let a0 = 0.5;
    let a1 = 0.5;
    let two_pi = 2.0 * PI;
    let denom: f32 = if sym {
        n as f32 - 1.0
    } else {
        n as f32
    };
    (0..n)
        .map(|i| a0 - a1 * (two_pi * i as f32 / denom).cos())
        .collect()
}
