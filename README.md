# peppi-py

[![](https://img.shields.io/pypi/v/peppi-py)](https://pypi.org/project/peppi-py/)
[![](https://img.shields.io/readthedocs/peppi-py)](https://peppi-py.readthedocs.io/en/latest/)

Python bindings for the [peppi](https://github.com/hohav/peppi) Slippi replay parser, built using [Apache Arrow](https://arrow.apache.org/) and [PyO3](https://pyo3.rs/).

## Installation

```sh
pip install peppi-py
```

To build from source instead, first install [Rust](https://rustup.rs/). Then:

```sh
pip install maturin
maturin develop
```

## Usage

peppi-py exposes two functions:

- `read_slippi(path, skip_frames=False)`
- `read_peppi(path, skip_frames=False)`

Both of these parse a replay file (`.slp` or `.slpp` respectively) into a [Game](https://peppi-py.readthedocs.io/en/latest/source/peppi_py.game.html#peppi_py.game.Game) object.

Frame data is stored as a [struct-of-arrays](https://en.wikipedia.org/wiki/AoS_and_SoA) for performance, using [Arrow](https://arrow.apache.org/). So to get the value of an attribute "foo.bar" for the `n`th frame of the game, you'd write `game.frames.foo.bar[n]` instead of `game.frames[n].foo.bar`. See the code example below.

You can do many other things with Arrow arrays, such as converting them to [numpy](https://numpy.org/) arrays. See the [pyarrow docs](https://arrow.apache.org/docs/python/) for more, particularly the various primitive array types such as [Int8Array](https://arrow.apache.org/docs/python/generated/pyarrow.Int8Array.html).

Also see the [Slippi replay spec](https://github.com/project-slippi/slippi-wiki/blob/master/SPEC.md) for detailed information about the available fields and their meanings.

```python
>>> from peppi_py import read_slippi, read_peppi
>>> game = read_slippi('tests/data/game.slp')
>>> game.metadata
{'startAt': '2018-06-22T07:52:59Z', 'lastFrame': 5085, 'players': {'1': {'characters': {'1': 5209}}, '0': {'characters': {'18': 5209}}}, 'playedOn': 'dolphin'}
>>> game.start
Start(slippi=Slippi(version=(1, 0, 0)), ...)
>>> game.end
End(method=<EndMethod.RESOLVED: 3>, lras_initiator=None, players=None)
>>> game.frames.ports[0].leader.post.position.x[0]
<pyarrow.FloatScalar: -42.0>
```
