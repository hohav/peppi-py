use peppi::io::Error as PeppiError;
use pyo3::prelude::PyErr;
use std::{error, fmt, io};

#[derive(Debug)]
pub enum PyO3ArrowError {
	ArrowError(arrow2::error::Error),
	IoError(io::Error),
	PeppiError(PeppiError),
	PeppiPyError(&'static str),
	PythonError(PyErr),
	JsonError(serde_json::Error),
}

impl fmt::Display for PyO3ArrowError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		use PyO3ArrowError::*;
		match *self {
			ArrowError(ref e) => e.fmt(f),
			IoError(ref e) => e.fmt(f),
			PeppiError(ref e) => e.fmt(f),
			PeppiPyError(ref e) => e.fmt(f),
			PythonError(ref e) => e.fmt(f),
			JsonError(ref e) => e.fmt(f),
		}
	}
}

impl error::Error for PyO3ArrowError {
	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		use PyO3ArrowError::*;
		match *self {
			ArrowError(ref e) => Some(e),
			IoError(ref e) => Some(e),
			PeppiError(ref e) => Some(e),
			PeppiPyError(_) => None,
			PythonError(ref e) => Some(e),
			JsonError(ref e) => Some(e),
		}
	}
}

impl From<arrow2::error::Error> for PyO3ArrowError {
	fn from(err: arrow2::error::Error) -> PyO3ArrowError {
		PyO3ArrowError::ArrowError(err)
	}
}

impl From<io::Error> for PyO3ArrowError {
	fn from(err: io::Error) -> PyO3ArrowError {
		PyO3ArrowError::IoError(err)
	}
}

impl From<PyErr> for PyO3ArrowError {
	fn from(err: PyErr) -> PyO3ArrowError {
		PyO3ArrowError::PythonError(err)
	}
}

impl From<PeppiError> for PyO3ArrowError {
	fn from(err: PeppiError) -> PyO3ArrowError {
		PyO3ArrowError::PeppiError(err)
	}
}

impl From<&'static str> for PyO3ArrowError {
	fn from(err: &'static str) -> PyO3ArrowError {
		PyO3ArrowError::PeppiPyError(err)
	}
}

impl From<serde_json::Error> for PyO3ArrowError {
	fn from(err: serde_json::Error) -> PyO3ArrowError {
		PyO3ArrowError::JsonError(err)
	}
}
