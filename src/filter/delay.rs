use crate::filter::{Filter, FirFilter};
use crate::filter::design::delay::{
    linear_interpolation,
    sinc_interpolation,
};

pub trait DelayFilter: Filter {
    fn delay(&self) -> f32;
}

pub struct LinearInterpDelay {
    delay: f32,
    filter: FirFilter,
}

pub struct SincInterpDelay {
    delay: f32,
    filter: FirFilter,
}

impl DelayFilter for LinearInterpDelay {
    fn delay(&self) -> f32 {
        self.delay
    }
}

impl Filter for LinearInterpDelay {
    fn process(&mut self, input: &[f32]) -> Vec<f32> {
        self.filter.process(input)
    }

    fn process_inplace(&mut self, buffer: &mut [f32]) {
        self.filter.process_inplace(buffer);
    }

    fn reset(&mut self) {
        self.filter.reset();
    }
}

impl LinearInterpDelay {
    pub fn new(delay: f32) -> Self {
        let coeffs = linear_interpolation(delay);
        Self {
            delay,
            filter: FirFilter::new(coeffs),
        }
    }
}

impl DelayFilter for SincInterpDelay {
    fn delay(&self) -> f32 {
        self.delay
    }
}

impl Filter for SincInterpDelay {
    fn process(&mut self, input: &[f32]) -> Vec<f32> {
        self.filter.process(input)
    }

    fn process_inplace(&mut self, buffer: &mut [f32]) {
        self.filter.process_inplace(buffer);
    }

    fn reset(&mut self) {
        self.filter.reset();
    }
}

impl SincInterpDelay {
    pub fn new(delay: f32, sinc_half_width: Option<usize>, window_type: Option<&str>) -> Self {
        let coeffs = sinc_interpolation(delay, sinc_half_width, window_type);
        Self {
            delay,
            filter: FirFilter::new(coeffs),
        }
    }
}

