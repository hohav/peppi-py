from dataclasses import dataclass
from pyarrow.lib import Int8Array, Int16Array, Int32Array, Int64Array, UInt8Array, UInt16Array, UInt32Array, UInt64Array, FloatArray, DoubleArray
from .util import _repr

@dataclass(slots=True)
class End:
	__repr__ = _repr
	latest_finalized_frame: Int32Array | None = None

@dataclass(slots=True)
class Position:
	__repr__ = _repr
	x: FloatArray
	y: FloatArray

@dataclass(slots=True)
class Start:
	__repr__ = _repr
	random_seed: UInt32Array
	scene_frame_counter: UInt32Array | None = None

@dataclass(slots=True)
class TriggersPhysical:
	__repr__ = _repr
	l: FloatArray
	r: FloatArray

@dataclass(slots=True)
class Velocities:
	__repr__ = _repr
	self_x_air: FloatArray
	self_y: FloatArray
	knockback_x: FloatArray
	knockback_y: FloatArray
	self_x_ground: FloatArray

@dataclass(slots=True)
class Velocity:
	__repr__ = _repr
	x: FloatArray
	y: FloatArray

@dataclass(slots=True)
class Item:
	__repr__ = _repr
	type: UInt16Array
	state: UInt8Array
	direction: FloatArray
	velocity: Velocity
	position: Position
	damage: UInt16Array
	timer: FloatArray
	id: UInt32Array
	misc: tuple[UInt8Array, UInt8Array, UInt8Array, UInt8Array] | None = None
	owner: Int8Array | None = None

@dataclass(slots=True)
class Post:
	__repr__ = _repr
	character: UInt8Array
	state: UInt16Array
	position: Position
	direction: FloatArray
	percent: FloatArray
	shield: FloatArray
	last_attack_landed: UInt8Array
	combo_count: UInt8Array
	last_hit_by: UInt8Array
	stocks: UInt8Array
	state_age: FloatArray | None = None
	state_flags: tuple[UInt8Array, UInt8Array, UInt8Array, UInt8Array, UInt8Array] | None = None
	misc_as: FloatArray | None = None
	airborne: UInt8Array | None = None
	ground: UInt16Array | None = None
	jumps: UInt8Array | None = None
	l_cancel: UInt8Array | None = None
	hurtbox_state: UInt8Array | None = None
	velocities: Velocities | None = None
	hitlag: FloatArray | None = None
	animation_index: UInt32Array | None = None

@dataclass(slots=True)
class Pre:
	__repr__ = _repr
	random_seed: UInt32Array
	state: UInt16Array
	position: Position
	direction: FloatArray
	joystick: Position
	cstick: Position
	triggers: FloatArray
	buttons: UInt32Array
	buttons_physical: UInt16Array
	triggers_physical: TriggersPhysical
	raw_analog_x: Int8Array | None = None
	percent: FloatArray | None = None
	raw_analog_y: Int8Array | None = None

@dataclass(slots=True)
class Data:
	__repr__ = _repr
	pre: Pre
	post: Post

@dataclass(slots=True)
class PortData:
	__repr__ = _repr
	leader: Data
	follower: Data | None = None

@dataclass(slots=True)
class Frame:
	__repr__ = _repr
	id: object
	ports: tuple[PortData]
