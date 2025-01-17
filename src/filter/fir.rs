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

        buffer.iter_mut().for_each(|sample| {
            // Push the new sample into the filter buffer
            self.buffer[self.buffer_index] = *sample;

            // Compute the filter output
            let buffer_len = self.buffer.len();
            let y = b.iter()
                .enumerate()
                .map(|(i, &coeff)| {
                    let idx = (self.buffer_index + (buffer_len - i)) & buffer_mask;
                    coeff * self.buffer[idx] // b[i] * x[n - i]
                })
                .sum();

            *sample = y;
            self.buffer_index = (self.buffer_index + 1) & buffer_mask;
        });
    }

    fn process(&mut self, input: &[f32]) -> Vec<f32> {
        let mut output = input.to_vec();
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
