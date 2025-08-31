import types, typing
import pyarrow
import dataclasses as dc
import functools
from inflection import underscore
from enum import Enum
from .frame import Data, Frame, PortData, Item

T = typing.TypeVar('T')

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

get_origin = functools.cache(typing.get_origin)
is_dataclass = functools.cache(dc.is_dataclass)
dc_fields = functools.cache(dc.fields)
get_args = functools.cache(typing.get_args)

@functools.cache
def unwrap_union(cls):
	if get_origin(cls) is types.UnionType:
		return get_args(cls)[0]
	else:
		return cls

def field_from_sa(cls: type[T], arr: pyarrow.Array | None) -> T | pyarrow.Array | None:
	if arr is None:
		return None
	cls = unwrap_union(cls)
	if is_dataclass(cls):
		return dc_from_sa(cls, arr)
	elif get_origin(cls) is tuple:
		return tuple_from_sa(cls, arr)
	else:
		return arr

def arr_field(arr, dc_field: dc.Field):
	try:
		return arr.field(dc_field.name)
	except KeyError:
		if dc_field.default is dc.MISSING:
			raise
		else:
			return dc_field.default

def dc_from_sa(cls: type[T], arr: pyarrow.StructArray) -> T:
	return cls(*(field_from_sa(f.type, arr_field(arr, f)) for f in dc_fields(cls)))

def tuple_from_sa(cls: type[tuple], arr: pyarrow.Array) -> tuple:
	return cls((field_from_sa(t, arr.field(str(idx))) for (idx, t) in enumerate(get_args(cls))))

@functools.cache
def unwrap_optional(cls: type) -> type | None:
	if get_origin(cls) is not types.UnionType:
		return cls

	args = get_args(cls)
	assert len(args) == 2
	assert args[1] is types.NoneType
	return args[0]

# Generic recursion on dataclasses
def map_dc(cls: type[T], fn: typing.Callable, *xs: T) -> T:
	unwrapped_cls = unwrap_optional(cls)
	if unwrapped_cls is not None:
		# Optional fields might be missing; if so, don't recurse.
		for x in xs:
			if x is None:
				return None
		cls = unwrapped_cls

	if is_dataclass(cls):
		return cls(*(
				map_dc(f.type, fn, *(getattr(x, f.name) for x in xs))
				for f in dc_fields(cls)
		))
	elif get_origin(cls) is tuple:
		return cls(
				map_dc(t, fn, *(x[idx] for x in xs))
				for (idx, t) in enumerate(get_args(cls))
		)
	else:
		return fn(*xs)

def dc_from_la(cls: type[T], la: pyarrow.ListArray) -> T:
	"""Converts ListArray of Structs into dataclass of ListArrays."""
	dc_sa = dc_from_sa(cls, la.values)
	return map_dc(cls, lambda arr: pyarrow.ListArray.from_arrays(la.offsets, arr), dc_sa)


class RollbackMode(Enum):
	ALL = 'all'  # All frames in the replay.
	FIRST = 'first'  # Only the first frame, as seen by the player
	LAST = 'last'  # Only the finalized frames; the "true" frame sequence.

def frames_from_sa(arrow_frames) -> typing.Optional[Frame]:
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

	# Extract items if available
	items = None
	try:
		items_array = arrow_frames.field('item')
		items = dc_from_la(Item, items_array)
	except KeyError:
		pass

	return Frame(arrow_frames.field('id'), tuple(ports), items)

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
