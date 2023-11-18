use std::{fs, io};
use std::collections::HashMap;
use arrow2::array::{Array, StructArray};
use pyo3::{
	exceptions::PyOSError,
	ffi::Py_uintptr_t,
	prelude::*,
	wrap_pyfunction,
};

use peppi::serde::de;
use peppi::model::frame::PortOccupancy;

mod error;
use error::PyO3ArrowError;

fn to_py_via_json<T: serde::Serialize>(py: Python, json: &PyModule, x: &T) -> Result<PyObject, PyO3ArrowError> {
	Ok(json.call_method1("loads",
		(serde_json::to_string(x)?,)
	)?.to_object(py))
}

fn to_py_via_arrow(py: Python, pyarrow: &PyModule, arr: StructArray) -> Result<PyObject, PyO3ArrowError> {
	let data_type = arr.data_type().clone();
	let array_ptr = &arrow2::ffi::export_array_to_c(arr.boxed()) as *const _;
	let schema_ptr = &arrow2::ffi::export_field_to_c(
		&arrow2::datatypes::Field::new("frames", data_type, false)
	) as *const _;
	Ok(pyarrow.getattr("Array")?.call_method1("_import_from_c",
		(array_ptr as Py_uintptr_t, schema_ptr as Py_uintptr_t),
	)?.to_object(py))
}

fn _game(py: Python, path: String, parse_opts: de::Opts) -> Result<PyObject, PyO3ArrowError> {
	let pyarrow = py.import("pyarrow")?;
	let json = py.import("json")?;
	let game = peppi::game(
		&mut io::BufReader::new(fs::File::open(path)?),
		Some(&parse_opts),
	)?;

	let mut m: HashMap<&str, PyObject> = HashMap::new();

	m.insert("start", to_py_via_json(py, json, &game.start)?);
	m.insert("end", to_py_via_json(py, json, &game.end)?);
	m.insert("metadata", to_py_via_json(py, json, &game.metadata)?);
	if !parse_opts.skip_frames {
		let ports: Vec<_> = game.start.players.iter().map(|p| PortOccupancy {
			port: p.port,
			follower: p.character == 14,
		}).collect();
		println!("{:?}", ports);
		m.insert("frames", to_py_via_arrow(py, &pyarrow,
			game.frames.into_struct_array(game.start.slippi.version, &ports))?);
	}

	Ok(m.to_object(py))
}

#[pyfunction]
#[pyo3(signature = (path, skip_frames = false))]
fn game(py: Python, path: String, skip_frames: bool) -> PyResult<PyObject> {
	_game(py, path, de::Opts { skip_frames, debug: None })
		.map_err(|e| PyOSError::new_err(e.to_string()))
}

#[pymodule]
fn peppi_py(_py: Python, m: &PyModule) -> PyResult<()> {
	m.add_function(wrap_pyfunction!(game, m)?)?;
	Ok(())
}
