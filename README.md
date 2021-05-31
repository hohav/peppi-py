# peppi-py

Python bindings for the [peppi](https://github.com/hohav/peppi) replay parser for Slippi, built using [Apache Arrow](https://arrow.apache.org/) and [PyO3](https://pyo3.rs/).

## Installation

```python
pip install peppi_py
```

## Usage

There is only one function, `peppi_py.game(path)`, which parses a replay file and returns a dict with these keys:

- `metadata`
- `start`
- `end`
- `frames`

The first three are regular dicts, but `frames` is an Arrow `StructArray` object. This can be treated as an array of dicts, but thanks to Arrow's columnar format you can do many other things such as convert columns to numpy arrays. See the [pyarrow docs](https://arrow.apache.org/docs/python/) for more, particularly [StructArray](https://arrow.apache.org/docs/python/generated/pyarrow.StructArray.html) and [StructScalar](https://arrow.apache.org/docs/python/generated/pyarrow.StructScalar.html) (which is what you get when you subscript a `StructArray`).

Also see the [Slippi replay spec](https://github.com/project-slippi/slippi-wiki/blob/master/SPEC.md) for detailed information about the available fields and their meanings.

```python
>>> import peppi_py
>>> game = peppi_py.game("game.slp")
>>> game['metadata']
{'date': '2018-06-22T07:52:59Z',
 'duration': 5209,
 'platform': 'dolphin',
 'players': [{'characters': {'18': 5209}, 'port': 'P1'},
             {'characters': {'1': 5209}, 'port': 'P2'}]}
>>> f = game['frames'][0]
>>> f['ports']['0']['leader']['post']['position']
<pyarrow.StructScalar: {'x': -42.0, 'y': 26.600000381469727}>
```
