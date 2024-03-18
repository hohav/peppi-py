from dataclasses import dataclass
from enum import IntEnum
from .frame import Frame
from .util import _repr

class Port(IntEnum):
	P1 = 0
	P2 = 1
	P3 = 2
	P4 = 3

class PlayerType(IntEnum):
	HUMAN = 0
	CPU = 1
	DEMO = 2

class Language(IntEnum):
	JAPANESE = 0
	ENGLISH = 1

class DashBack(IntEnum):
	UCF = 1
	ARDUINO = 2

class ShieldDrop(IntEnum):
	UCF = 1
	ARDUINO = 2

class EndMethod(IntEnum):
	UNRESOLVED = 0
	TIME = 1
	GAME = 2
	RESOLVED = 3
	NO_CONTEST = 7

@dataclass(slots=True)
class Scene:
	__repr__ = _repr
	major: int
	minor: int

@dataclass(slots=True)
class Match:
	__repr__ = _repr
	id: str
	game: int
	tiebreaker: int

@dataclass(slots=True)
class Slippi:
	__repr__ = _repr
	version: tuple[int, int, int]

@dataclass(slots=True)
class Netplay:
	__repr__ = _repr
	name: str
	code: str
	suid: str | None = None

@dataclass(slots=True)
class Team:
	__repr__ = _repr
	color: int
	shade: int

@dataclass(slots=True)
class Ucf:
	__repr__ = _repr
	dash_back: DashBack | None
	shield_drop: ShieldDrop | None

@dataclass(slots=True)
class Player:
	__repr__ = _repr
	port: Port
	character: int
	type: PlayerType
	stocks: int
	costume: int
	team: Team | None
	handicap: int
	bitfield: int
	cpu_level: int | None
	offense_ratio: float
	defense_ratio: float
	model_scale: float
	ucf: Ucf | None = None
	name_tag: str | None = None
	netplay: Netplay | None = None

@dataclass(slots=True)
class Start:
	__repr__ = _repr
	slippi: Slippi
	bitfield: tuple[int, int, int, int]
	is_raining_bombs: bool
	is_teams: bool
	item_spawn_frequency: int
	self_destruct_score: int
	stage: int
	timer: int
	item_spawn_bitfield: tuple[int, int, int, int, int]
	damage_ratio: float
	players: tuple[Player, ...]
	random_seed: int
	is_pal: bool | None = None
	is_frozen_ps: bool | None = None
	scene: Scene | None = None
	language: Language | None = None
	match: Match | None = None

@dataclass(slots=True)
class PlayerEnd:
	__repr__ = _repr
	port: Port
	placement: int

@dataclass(slots=True)
class End:
	__repr__ = _repr
	method: EndMethod
	lras_initiator: Port | None = None
	players: tuple[PlayerEnd, ...] | None = None

@dataclass(slots=True)
class Game:
	__repr__ = _repr
	start: Start
	end: End
	metadata: dict
	frames: Frame | None
