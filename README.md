# peppi-py

Python bindings for the [peppi](https://github.com/hohav/peppi) replay parser for Slippi, built using [Apache Arrow](https://arrow.apache.org/) and [PyO3](https://pyo3.rs/).

## Installation

```sh
pip install peppi-py
```

## Usage

There are two functions in `peppi_py`:

- `read_slippi(path, skip_frames = True)`
- `read_peppi(path, skip_frames = True)`

Both of these parse a replay file (`.slp` or `.slpp` respectively), and return an object with these attributes:

- `start`
- `end`
- `metadata`
- `hash`
- `frames`

The first three are regular dicts, but `frames` is an Arrow `StructArray` object. This can be treated as an array of dicts, but thanks to Arrow's columnar format you can do many other things such as convert columns to numpy arrays. See the [pyarrow docs](https://arrow.apache.org/docs/python/) for more, particularly [StructArray](https://arrow.apache.org/docs/python/generated/pyarrow.StructArray.html) and [StructScalar](https://arrow.apache.org/docs/python/generated/pyarrow.StructScalar.html) (which is what you get when you subscript a `StructArray`).

Also see the [Slippi replay spec](https://github.com/project-slippi/slippi-wiki/blob/master/SPEC.md) for detailed information about the available fields and their meanings.

```python
>>> import peppi_py
>>> game = peppi_py.read_slippi('game.slp')
>>> game.metadata
{'lastFrame': 11238,
 'playedOn': 'dolphin',
 'players': {'0': {'characters': {'18': 11469},
                   'names': {'code': 'AAAA#123', 'netplay': 'abbott'}},
             '1': {'characters': {'17': 11469},
                   'names': {'code': 'BBBB#456', 'netplay': 'costello'}}},
 'startAt': '2020-08-01T19:41:19Z'}
>>> f = game.frames[0]
>>> f['ports'][0]['leader']['post']['position']
<pyarrow.StructScalar: {'x': -42.0, 'y': 26.600000381469727}>
```
