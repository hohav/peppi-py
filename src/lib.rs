use arrow2::array::{Array, StructArray};
use pyo3::{
	exceptions::PyOSError,
	ffi::Py_uintptr_t,
	prelude::*,
	types::PyDict,
	wrap_pyfunction,
};
use std::{fs, io};
use std::io::{BufReader, ErrorKind};
use peppi::io::slippi::de;
use byteorder::ReadBytesExt;
use std::fs::File;
use arrow2::array::MutableArray;
// use pyo3::types::PyIterator;
// use peppi::io::Error as PeppiError;
// use pyo3::types::PyAny;
// use peppi::io::slippi::de::ParseState;
// use peppi::io::Result;



use peppi::frame::PortOccupancy;
use peppi::game::{Start, ICE_CLIMBERS};
use peppi::io::peppi::de::Opts as PeppiOpts;
use peppi::io::slippi::de::Opts as SlippiOpts;

mod error;
use error::PyO3ArrowError;

fn to_py_via_json<T: serde::Serialize>(
	json: &Bound<PyModule>,
	x: &T,
) -> Result<Py<PyDict>, PyO3ArrowError> {
	Ok(json
		.call_method1("loads", (serde_json::to_string(x)?,))?
		.extract()?)
}

fn to_py_via_arrow(
	py: Python,
	pyarrow: &Bound<PyModule>,
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
) -> Result<Bound<Game>, PyO3ArrowError> {
	let pyarrow = py.import_bound("pyarrow")?;
	let json = py.import_bound("json")?;
	let game = peppi::io::slippi::read(
		&mut io::BufReader::new(fs::File::open(path)?),
		Some(&parse_opts),
	)?;

	println!("{:?}", game.hash);
	Ok(Bound::new(
		py,
		Game {
			start: to_py_via_json(&json, &game.start)?,
			end: to_py_via_json(&json, &game.end)?,
			metadata: to_py_via_json(&json, &game.metadata)?,
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
) -> Result<Bound<Game>, PyO3ArrowError> {
	let pyarrow = py.import_bound("pyarrow")?;
	let json = py.import_bound("json")?;
	let game = peppi::io::peppi::read(
		&mut io::BufReader::new(fs::File::open(path)?),
		Some(&parse_opts),
	)?;

	Ok(Bound::new(
		py,
		Game {
			start: to_py_via_json(&json, &game.start)?,
			end: to_py_via_json(&json, &game.end)?,
			metadata: to_py_via_json(&json, &game.metadata)?,
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
fn read_slippi(py: Python, path: String, skip_frames: bool) -> PyResult<Bound<Game>> {
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
fn read_peppi(py: Python, path: String, skip_frames: bool) -> PyResult<Bound<Game>> {
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

// #[pyfunction]
// #[pyo3(signature = (path))]
// fn read_slippi_stream(py: Python, path: String) -> PyResult<PyObject> {
// let r = BufReader::new(File::open(path)?);
//     let mut r = std::io::BufReader::new(r);
//     let size = de::parse_header(&mut r, None).unwrap() as usize;
//     let mut state = de::parse_start(&mut r, None).unwrap();

//     // Return a generator
//     pyo3::types::PyIter::from_fn(py, move || {
//         loop {
//             match de::parse_event(&mut r, &mut state, None) {
//                 Ok(event) if event == de::Event::GameEnd as u8 => {
//                     return Ok(None); // Stop the generator
//                 }
//                 Ok(event) => {
//                     // Convert event to something Python can work with
//                     let py_event = event_to_py_object(py, event);
//                     return Ok(Some(py_event));
//                 }
//                 Err(PeppiError::Io(ref io_err)) if io_err.kind() == ErrorKind::UnexpectedEof => {
// 					std::thread::sleep(std::time::Duration::from_millis(200));
// 					continue;
// 				}
// 				Err(err) => {
// 					return Err(err.into());
// 				}
// 						}
//         }
//     })
//     .map(|iter| iter.into_py(py))
// }

// /// Converts the event (u8 or Event) to a PyObject. Customize this for your use case.
// fn event_to_py_object(py: Python, event: u8) -> PyObject {
//     event.into_py(py)
// }

#[pymodule]
#[pyo3(name = "_peppi")]
fn peppi_py(m: &Bound<PyModule>) -> PyResult<()> {
	m.add_class::<Game>()?;
	m.add_function(wrap_pyfunction!(read_slippi, m)?)?;
	m.add_function(wrap_pyfunction!(read_peppi, m)?)?;
	m.add_function(wrap_pyfunction!(read_slippi_stream, m)?)?;
	m.add_class::<SlippiStreamReader>()?;
	Ok(())
}

#[pyfunction]
#[pyo3(signature = (path))]
fn read_slippi_stream(py: Python, path: String) -> () {
	let mut r = BufReader::new(fs::File::open(path).unwrap());
    let size = de::parse_header(&mut r, None).unwrap() as usize;
    let mut state = de::parse_start(&mut r, None).unwrap();
	println!("READ SLIPPI STREAM");
	println!("size: {:?}", size);
	let mut y = 80000;
	let mut ii = 0;
	while true {
		match de::parse_event(&mut r, &mut state, None) {
			Ok(event) if event == de::Event::GameEnd as u8 => {
				return;
			}
			Ok(event) => {
				let mut frame_number = &state.frames().id.values().last();
				let mut pre = &state.frames().ports[0].leader.pre;
				let char_state = &pre.state;
				let buttons = &pre.buttons;
				let x_coord = &pre.joystick.x;
				let y_coord = &pre.joystick.y;
				println!(
					"{:?}:{:?}:{:?}:{:?}:{:?}",
					frame_number,
					char_state.values().last(),
					buttons.values().last(),
					x_coord.values().last(),
					y_coord.values().last()
				);
			}
			Err(err) => {
				std::thread::sleep(std::time::Duration::from_millis(200));
			}
		}
	} 

	// if r.read_u8().unwrap() == 0x55 {
	// 	de::parse_metadata(&mut r, &mut state, None).unwrap()
	// }
	// de::parse_event(&mut r, &mut state, None).unwrap();
	// match state.frames().id.iter().last() {
	// 	Some(Some(i)) => *i as u8,
	// 	_ => 0,
	// }
	// state.frames().id.iter().last()
}


//// A Python-exposed iterator to read Slippi stream events.
#[pyclass]
pub struct SlippiStreamReader {
    reader: BufReader<File>,
	start: de::ParseState,
    size: usize,
}


#[pymethods]
impl SlippiStreamReader {
    #[new]
    fn new(path: String) -> PyResult<Self> {
		println!("HEY LOUDER");
        let mut reader = BufReader::new(fs::File::open(path).unwrap());
        let size = de::parse_header(&mut reader, None).unwrap() as usize;
        let mut start = de::parse_start(&mut reader, None).unwrap();
		println!("HEY");
        Ok(Self { reader, start, size })
    }

    // fn __iter__(slf: PyRefMut<Self>) -> PyRefMut<Self> {
    //     slf
    // }

    // fn __next__(mut slf: PyRefMut<Self>) -> Option<PyResult<String>> {
    //     loop {
    // 		let mut state = slf.start;
    //         match de::parse_event(&mut slf.reader,&mut state, None) {
    //             Ok(event) if event == de::Event::GameEnd as u8 => {
    //                 return None; // Stop iteration
    //             }
    //             Ok(event) => {
    //                 return Some(Ok(format!("Event: {:?}", event)));
    //             }
    //             Err(err)=> {
    //                 // Sleep and retry in case of EOF (waiting for more data
	// 				// Err(err) => return Some(Ok(format!("Error: {:?}", err)))
    //                 std::thread::sleep(std::time::Duration::from_millis(200));
    //                 continue;
    //             }
				
	// 		}
    //     }
    // }

}