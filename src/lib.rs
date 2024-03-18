use arrow2::array::{Array, StructArray};
use pyo3::{
	exceptions::PyOSError,
	ffi::Py_uintptr_t,
	prelude::*,
	types::PyDict,
	wrap_pyfunction,
};
use std::{fs, io};

use peppi::frame::PortOccupancy;
use peppi::game::{Start, ICE_CLIMBERS};
use peppi::io::peppi::de::Opts as PeppiOpts;
use peppi::io::slippi::de::Opts as SlippiOpts;

mod error;
use error::PyO3ArrowError;

fn to_py_via_json<'a, T: serde::Serialize>(
	json: &PyModule,
	x: &T,
) -> Result<Py<PyDict>, PyO3ArrowError> {
	Ok(json
		.call_method1("loads", (serde_json::to_string(x)?,))?
		.extract()?)
}

fn to_py_via_arrow(
	py: Python,
	pyarrow: &PyModule,
	arr: StructArray,
) -> Result<PyObject, PyO3ArrowError> {
	let data_type = arr.data_type().clone();
	let array_ptr = &arrow2::ffi::export_array_to_c(arr.boxed()) as *const _;
	let schema_ptr =
		&arrow2::ffi::export_field_to_c(&arrow2::datatypes::Field::new("frames", data_type, false))
			as *const _;
	Ok(pyarrow
		.getattr("Array")?
		.call_method1(
			"_import_from_c",
			(array_ptr as Py_uintptr_t, schema_ptr as Py_uintptr_t),
		)?
		.to_object(py))
}

#[pyclass(get_all, set_all)]
pub struct Game {
	pub start: Py<PyDict>,
	pub end: Py<PyDict>,
	pub metadata: Py<PyDict>,
	pub hash: Option<String>,
	pub frames: Option<PyObject>,
}

fn port_occupancy(start: &Start) -> Vec<PortOccupancy> {
	start
		.players
		.iter()
		.map(|p| PortOccupancy {
			port: p.port,
			follower: p.character == ICE_CLIMBERS,
		})
		.collect()
}

fn _read_slippi(
	py: Python,
	path: String,
	parse_opts: SlippiOpts,
) -> Result<&PyCell<Game>, PyO3ArrowError> {
	let pyarrow = py.import("pyarrow")?;
	let json = py.import("json")?;
	let game = peppi::io::slippi::read(
		&mut io::BufReader::new(fs::File::open(path)?),
		Some(&parse_opts),
	)?;

	println!("{:?}", game.hash);
	Ok(PyCell::new(
		py,
		Game {
			start: to_py_via_json(json, &game.start)?,
			end: to_py_via_json(json, &game.end)?,
			metadata: to_py_via_json(json, &game.metadata)?,
			hash: game.hash,
			frames: match parse_opts.skip_frames {
				true => None,
				_ => Some(to_py_via_arrow(
					py,
					&pyarrow,
					game.frames
						.into_struct_array(game.start.slippi.version, &port_occupancy(&game.start)),
				)?),
			},
		},
	)?)
}

fn _read_peppi(
	py: Python,
	path: String,
	parse_opts: PeppiOpts,
) -> Result<&PyCell<Game>, PyO3ArrowError> {
	let pyarrow = py.import("pyarrow")?;
	let json = py.import("json")?;
	let game = peppi::io::peppi::read(
		&mut io::BufReader::new(fs::File::open(path)?),
		Some(&parse_opts),
	)?;

	Ok(PyCell::new(
		py,
		Game {
			start: to_py_via_json(json, &game.start)?,
			end: to_py_via_json(json, &game.end)?,
			metadata: to_py_via_json(json, &game.metadata)?,
			hash: game.hash,
			frames: match parse_opts.skip_frames {
				true => None,
				_ => Some(to_py_via_arrow(
					py,
					&pyarrow,
					game.frames
						.into_struct_array(game.start.slippi.version, &port_occupancy(&game.start)),
				)?),
			},
		},
	)?)
}

#[pyfunction]
#[pyo3(signature = (path, skip_frames = false))]
fn read_slippi(py: Python, path: String, skip_frames: bool) -> PyResult<&PyCell<Game>> {
	_read_slippi(
		py,
		path,
		SlippiOpts {
			skip_frames,
			..Default::default()
		},
	)
	.map_err(|e| PyOSError::new_err(e.to_string()))
}

#[pyfunction]
#[pyo3(signature = (path, skip_frames = false))]
fn read_peppi(py: Python, path: String, skip_frames: bool) -> PyResult<&PyCell<Game>> {
	_read_peppi(
		py,
		path,
		PeppiOpts {
			skip_frames,
			..Default::default()
		},
	)
	.map_err(|e| PyOSError::new_err(e.to_string()))
}

#[pymodule]
#[pyo3(name = "_peppi")]
fn peppi_py(_py: Python, m: &PyModule) -> PyResult<()> {
	m.add_class::<Game>()?;
	m.add_function(wrap_pyfunction!(read_slippi, m)?)?;
	m.add_function(wrap_pyfunction!(read_peppi, m)?)?;
	Ok(())
}
