pub mod fir;
pub mod delay;
mod design;

pub use fir::FirFilter;
pub use delay::{
    DelayFilter,
    LinearInterpDelay,
    SincInterpDelay,
};

pub trait Filter {
    fn process(&mut self, input: &[f32]) -> Vec<f32>;
    fn process_inplace(&mut self, buffer: &mut [f32]);
    fn reset(&mut self);
}
