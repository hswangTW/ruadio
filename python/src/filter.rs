use pyo3::prelude::*;
use numpy::{
    PyReadonlyArray1,
    PyArray1,
    PyArrayMethods,
    ToPyArray,
};

use rustafx::filter::{
    Filter,
    LinearInterpDelay,
    SincInterpDelay,
};

use crate::utilities::convert_to_f32_array;

#[pymodule(name = "filter")]
pub mod py_filter {
    use super::*;

    #[pyclass(name = "LinearInterpDelay")]
    struct PyLinearInterpDelay {
        filter: LinearInterpDelay,
    }


    // TODO Make the methods accept more different dtypes

    #[pymethods]
    impl PyLinearInterpDelay {
        #[new]
        fn new(delay: f32) -> Self {
            Self { filter: LinearInterpDelay::new(delay) }
        }

        fn process<'py>(&mut self, py: Python<'py>, input: Bound<'py, PyAny>) -> PyResult<Bound<'py, PyArray1<f32>>> {
            if let Ok(input_array) = input.downcast::<PyArray1<f32>>() {
                // The input can be directly read as an f32 array (no copy/conversion needed)
                let input_array: PyReadonlyArray1<f32> = input_array.try_readonly().unwrap();
                let input: &[f32] = input_array.as_slice().unwrap();
                let output = self.filter.process(input);
                return Ok(output.to_pyarray(py));
            }

            let input_array = convert_to_f32_array(input)?;
            let input: &[f32] = input_array.as_slice().unwrap();
            let output = self.filter.process(input);
            Ok(output.to_pyarray(py))
        }

        fn reset(&mut self) {
            self.filter.reset();
        }
    }

    #[pyclass(name = "SincInterpDelay")]
    struct PySincInterpDelay {
        filter: SincInterpDelay,
    }

    #[pymethods]
    impl PySincInterpDelay {
        #[new]
        #[pyo3(signature = (delay, sinc_half_width=None, window_type=None))]
        fn new(delay: f32, sinc_half_width: Option<usize>, window_type: Option<&str>) -> Self {
            Self { filter: SincInterpDelay::new(delay, sinc_half_width, window_type) }
        }

        fn process<'py>(&mut self, py: Python<'py>, input: Bound<'py, PyAny>) -> PyResult<Bound<'py, PyArray1<f32>>> {
            if let Ok(input_array) = input.downcast::<PyArray1<f32>>() {
                // The input can be directly read as an f32 array (no copy/conversion needed)
                let input_array: PyReadonlyArray1<f32> = input_array.try_readonly().unwrap();
                let input: &[f32] = input_array.as_slice().unwrap();
                let output = self.filter.process(input);
                return Ok(output.to_pyarray(py));
            }

            let input_array = convert_to_f32_array(input)?;
            let input: &[f32] = input_array.as_slice().unwrap();
            let output = self.filter.process(input);
            Ok(output.to_pyarray(py))
        }

        fn reset(&mut self) {
            self.filter.reset();
        }
    }
}
