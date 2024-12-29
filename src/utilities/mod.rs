use std::f32::consts::PI;

/// The tolerance for floating point comparisons.
const EPSILON: f64 = 1e-6;

/// Macro for checking if all the values of two sequences are equal.
#[doc(hidden)]
#[macro_export]
macro_rules! assert_all_eq {
    ($a:expr, $b:expr) => {
        assert_eq!($a.len(), $b.len(), "Lengths of sequences do not match");
        for i in 0..$a.len() {
            assert_eq!(
                $a[i], $b[i],
                "Values at index {} are not equal. Left: {}, Right: {}",
                i, $a[i], $b[i]
            );
        }
    };
}

/// Macro for checking if all the values of two sequences are equal within a tolerance.
#[doc(hidden)]
#[macro_export]
macro_rules! assert_all_close {
    ($a:expr, $b:expr) => {
        assert_eq!($a.len(), $b.len(), "Lengths of sequences do not match");
        for i in 0..$a.len() {
            assert!(
                (($a[i] - $b[i]).abs() as f64) < 1e-6,
                "Values at index {} are not close. Left: {}, Right: {}",
                i, $a[i], $b[i]
            );
        }
    };
    ($a:expr, $b:expr, $tol:expr) => {
        assert_eq!($a.len(), $b.len(), "Lengths of sequences do not match");
        for i in 0..$a.len() {
            assert!(
                (($a[i] - $b[i]).abs() as f64) < $tol as f64,
                "Values at index {} are not close. Left: {}, Right: {}",
                i, $a[i], $b[i]
            );
        }
    };
}

/// The normalized sinc function for digital signal processing.
pub fn sinc(x: f32) -> f32 {
    if x.abs() < EPSILON as f32 {
        return 1.0;
    }
    (PI * x).sin() / (PI * x)
}
