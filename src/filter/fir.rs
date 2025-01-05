use crate::filter::Filter;
use crate::filter::design::FirCoeffs;

pub struct FirFilter {
    coeffs: FirCoeffs,
    /// FIFO buffer for storing the input samples. The length will be restricted to powers of 2.
    buffer: Vec<f32>,
    /// Index of the next sample to be written to the buffer.
    buffer_index: usize,
}

// TODO Utilize SIMD for processing

impl Filter for FirFilter {
    fn process_inplace(&mut self, buffer: &mut [f32]) {
        let b = &self.coeffs.b;
        let buffer_mask = self.buffer.len() - 1; // For wrapping around the buffer index

        for n in 0..buffer.len() {
            // Push the new sample into the buffer
            self.buffer[self.buffer_index] = buffer[n];

            // Compute the output
            let mut y = 0.0;
            let mut idx = self.buffer_index;
            for coeff in b.iter() {
                y += coeff * self.buffer[idx];
                idx = (idx + buffer_mask) & buffer_mask; // Wrap around (idx - 1)
            }

            buffer[n] = y;
            self.buffer_index = (self.buffer_index + 1) & buffer_mask;
        }
    }

    fn process(&mut self, buffer: &[f32]) -> Vec<f32> {
        let mut output = buffer.to_vec();
        self.process_inplace(&mut output);
        output
    }

    fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.buffer_index = 0;
    }
}

impl FirFilter {
    pub fn new(coeffs: FirCoeffs) -> Self {
        let size = coeffs.b.len().next_power_of_two();
        Self {
            coeffs,
            buffer: vec![0.0; size],
            buffer_index: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buffer_length() {
        // Buffer length should be the smallest power of 2 greater than the number of coefficients
        let coeffs = FirCoeffs::new(vec![1.0; 10]);
        let filter = FirFilter::new(coeffs);
        assert_eq!(filter.buffer.len(), 16);

        let coeffs = FirCoeffs::new(vec![1.0; 8]);
        let filter = FirFilter::new(coeffs);
        assert_eq!(filter.buffer.len(), 8);

        let coeffs = FirCoeffs::new(vec![1.0; 100]);
        let filter = FirFilter::new(coeffs);
        assert_eq!(filter.buffer.len(), 128);
    }

    mod process {
        use super::*;

        #[test]
        fn case_1() {
            let coeffs = FirCoeffs::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
            let mut filter = FirFilter::new(coeffs);
            let input: Vec<f32> = vec![1.0, 0.0, 0.0, 0.0, 0.0];
            let output = filter.process(&input);
            assert_eq!(input, [1.0, 0.0, 0.0, 0.0, 0.0]);
            assert_eq!(output, [1.0, 2.0, 3.0, 4.0, 5.0]);
        }
    }

    mod process_inplace {
        use super::*;

        #[test]
        fn case_1() {
            let coeffs = FirCoeffs::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
            let mut filter = FirFilter::new(coeffs);
            let mut buffer: Vec<f32> = vec![1.0, 0.0, 0.0, 0.0, 0.0];
            filter.process_inplace(&mut buffer);
            assert_eq!(buffer, [1.0, 2.0, 3.0, 4.0, 5.0]);
        }
    }

    mod reset {
        use super::*;

        #[test]
        fn case_1() {
            let coeffs = FirCoeffs::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
            let mut filter = FirFilter::new(coeffs);
            let input: Vec<f32> = vec![1.0, 0.0];
            let output = filter.process(&input);
            assert_eq!(output, [1.0, 2.0]);

            filter.reset();
            let input: Vec<f32> = vec![0.0, 0.0, 0.0];
            let output = filter.process(&input);
            assert_eq!(output, [0.0, 0.0, 0.0]);
        }
    }
}
