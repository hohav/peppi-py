use std::{fs, io};
use std::collections::HashMap;
use arrow::array::{Array, StructArray};
use pyo3::{
	exceptions::PyOSError,
	ffi::Py_uintptr_t,
	prelude::*,
	wrap_pyfunction,
};

use peppi::serde::{collect, de};

mod error;
use error::PyO3ArrowError;

fn to_py_via_json<T: serde::Serialize>(py: Python, json: &PyModule, x: &T) -> Result<PyObject, PyO3ArrowError> {
	Ok(json.call_method1("loads",
		(serde_json::to_string(x)?,)
	)?.to_object(py))
}

fn to_py_via_arrow(py: Python, pyarrow: &PyModule, arr: StructArray) -> Result<PyObject, PyO3ArrowError> {
	let (array_pointer, schema_pointer) = arr.to_raw()?;
	Ok(pyarrow.getattr("Array")?.call_method1("_import_from_c",
		(array_pointer as Py_uintptr_t, schema_pointer as Py_uintptr_t),
	)?.to_object(py))
}

fn _game(py: Python, path: String, parse_opts: de::Opts, collect_opts: collect::Opts) -> Result<PyObject, PyO3ArrowError> {
	let pyarrow = py.import("pyarrow")?;
	let json = py.import("json")?;
	let game = peppi::game(
		&mut io::BufReader::new(fs::File::open(path)?),
		Some(parse_opts),
		Some(collect_opts),
	)?;

	let mut m: HashMap<&str, PyObject> = HashMap::new();

	m.insert("start", to_py_via_json(py, json, &game.start)?);
	m.insert("end", to_py_via_json(py, json, &game.end)?);
	m.insert("metadata", to_py_via_json(py, json, &game.metadata_raw)?);
	if !parse_opts.skip_frames {
		m.insert("frames", to_py_via_arrow(py, &pyarrow, peppi::serde::arrow::frames_to_arrow(&game, None))?);
	}

	Ok(m.to_object(py))
}

#[pyfunction(skip_frames = "false", rollbacks = "false")]
fn game(py: Python, path: String, skip_frames: bool, rollbacks: bool) -> PyResult<PyObject> {
	_game(py, path, de::Opts { skip_frames }, collect::Opts { rollbacks })
		.map_err(|e| PyOSError::new_err(e.to_string()))
}

#[pymodule]
fn peppi_py(_py: Python, m: &PyModule) -> PyResult<()> {
	m.add_function(wrap_pyfunction!(game, m)?)?;
	Ok(())
}
