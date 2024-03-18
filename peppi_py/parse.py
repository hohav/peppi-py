import sys, types, typing
import pyarrow
import dataclasses as dc
from inflection import underscore
from enum import Enum
from .frame import Data, Frame, PortData

def _repr(x):
	if isinstance(x, pyarrow.Array):
		s = ', '.join(repr(v.as_py()) for v in x[:3])
		if len(x) > 3:
			s += ', ...'
		return f'[{s}]'
	elif isinstance(x, tuple):
		s = ', '.join(_repr(v) for v in x)
		return f'({s})'
	elif dc.is_dataclass(x):
		s = ', '.join(f'{f.name}={_repr(getattr(x, f.name))}' for f in dc.fields(type(x)))
		return f'{type(x).__name__}({s})'
	else:
		return repr(x)

def unwrap_union(cls):
	if typing.get_origin(cls) is types.UnionType:
		return typing.get_args(cls)[0]
	else:
		return cls

def field_from_sa(cls, arr):
	if arr is None:
		return None
	cls = unwrap_union(cls)
	if dc.is_dataclass(cls):
		return dc_from_sa(cls, arr)
	elif typing.get_origin(cls) is tuple:
		return tuple_from_sa(cls, arr)
	else:
		return arr

def arr_field(arr, dc_field):
	try:
		return arr.field(dc_field.name)
	except KeyError:
		if dc_field.default is dc.MISSING:
			raise
		else:
			return dc_field.default

def dc_from_sa(cls, arr):
	return cls(*(field_from_sa(f.type, arr_field(arr, f)) for f in dc.fields(cls)))

def tuple_from_sa(cls, arr):
	return tuple((field_from_sa(t, arr.field(str(idx))) for (idx, t) in enumerate(typing.get_args(cls))))

def frames_from_sa(arrow_frames):
	if arrow_frames is None:
		return None
	ports = []
	port_arrays = arrow_frames.field('ports')
	for p in ('P1', 'P2', 'P3', 'P4'):
		try: port = port_arrays.field(p)
		except KeyError: continue
		leader = dc_from_sa(Data, port.field('leader'))
		try: follower = dc_from_sa(Data, port.field('follower'))
		except KeyError: follower = None
		ports.append(PortData(leader, follower))
	return Frame(arrow_frames.field('id'), tuple(ports))

def field_from_json(cls, json):
	if json is None:
		return None
	cls = unwrap_union(cls)
	if dc.is_dataclass(cls):
		return dc_from_json(cls, json)
	elif typing.get_origin(cls) is tuple:
		return tuple_from_json(cls, json)
	elif issubclass(cls, Enum):
		return cls[underscore(json).upper()]
	else:
		return json

def dc_from_json(cls, json):
	return cls(*(field_from_json(f.type, json.get(f.name)) for f in dc.fields(cls)))

def tuple_from_json(cls, json):
	child_cls = typing.get_args(cls)[0]

	if type(json) is dict:
		items = json.items()
	else:
		items = enumerate(json)

	return tuple((field_from_json(child_cls, val) for (idx, val) in items))
