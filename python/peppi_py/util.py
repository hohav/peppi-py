import dataclasses as dc
import pyarrow as pa

def _repr(x):
	if isinstance(x, pa.Array):
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

@dc.dataclass(slots=True)
class ListArray[T]:
	__repr__ = _repr
	values: T
	offsets: pa.Int32Array
