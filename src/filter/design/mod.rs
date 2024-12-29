//! Filter design for FIR and IIR filters.

pub mod delay;
pub mod window;

/// FIR filter coefficients.
#[derive(Debug, Clone)]
pub struct FirCoeffs {
    pub(crate) b: Vec<f32>,
}

#[derive(Debug, Clone)]
pub struct IirCoeffs {
    pub(crate) b: Vec<f32>,
    pub(crate) a: Vec<f32>,
}

#[derive(Debug, Clone)]
pub struct SecondOrderSection {
    pub(crate) b0: f32,
    pub(crate) b1: f32,
    pub(crate) b2: f32,
    pub(crate) a1: f32,    // Note: a0 is always 1.0
    pub(crate) a2: f32,
}

#[derive(Debug, Clone)]
pub struct SosCoeffs {
    pub(crate) sections: Vec<SecondOrderSection>,
}
