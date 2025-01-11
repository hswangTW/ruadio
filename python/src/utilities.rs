use pyo3::prelude::*;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::types::PyList;
use numpy::{
    PyArray1,
    PyArrayMethods,
};
use numpy::ndarray::Array1;

pub fn convert_to_f32_array<'py>(obj: Bound<'py, PyAny>) -> PyResult<Array1<f32>> {
    if let Ok(array) = obj.downcast::<PyArray1<f32>>() {
        Ok(array.to_owned_array())
    } else if let Ok(array) = obj.downcast::<PyArray1<f64>>() {
        Ok(array.cast::<f32>(false).unwrap().to_owned_array())
    } else if let Ok(array) = obj.downcast::<PyArray1<i32>>() {
        Ok(array.cast::<f32>(false).unwrap().to_owned_array())
    } else if let Ok(array) = obj.downcast::<PyArray1<i64>>() {
        Ok(array.cast::<f32>(false).unwrap().to_owned_array())
    } else if let Ok(list) = obj.downcast::<PyList>() {
        let values: Vec<f32> = list.extract()
            .map_err(|_| PyValueError::new_err(
                "Failed to convert the Python list to a Rust vector of f32 values."
            ))?;
        Ok(Array1::from_vec(values))
    } else {
        Err(PyTypeError::new_err("Input must be a numpy array or list of real-valued 32/64-bit integer/float."))
    }
}
