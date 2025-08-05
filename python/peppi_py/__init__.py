#!/usr/bin/env python
from dataclasses import dataclass
from ._peppi import read_peppi as _read_peppi, read_slippi as _read_slippi
from .game import End, Game, Start
from .parse import dc_from_json, frames_from_sa

def read_peppi(path: str, skip_frames: bool=False) -> Game:
	g = _read_peppi(path, skip_frames)
	return Game(dc_from_json(Start, g.start), g.end and dc_from_json(End, g.end), g.metadata, frames_from_sa(g.frames))

def read_slippi(path: str, skip_frames: bool=False) -> Game:
	g = _read_slippi(path, skip_frames)
	return Game(dc_from_json(Start, g.start), g.end and dc_from_json(End, g.end), g.metadata, frames_from_sa(g.frames))
