use pyo3::prelude::*;
use numpy::{
    PyReadonlyArray1,
    PyArray1,
    ToPyArray,
};

use rustafx::filter::{
    Filter,
    LinearInterpDelay,
    SincInterpDelay,
};

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

        fn process<'py>(&mut self, py: Python<'py>, input: PyReadonlyArray1<f32>) -> PyResult<Bound<'py, PyArray1<f32>>> {
            let input: &[f32] = input.as_slice().unwrap();
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

        fn process<'py>(&mut self, py: Python<'py>, input: PyReadonlyArray1<f32>) -> PyResult<Bound<'py, PyArray1<f32>>> {
            let input: &[f32] = input.as_slice().unwrap();
            let output = self.filter.process(input);
            Ok(output.to_pyarray(py))
        }

        fn reset(&mut self) {
            self.filter.reset();
        }
    }
}
