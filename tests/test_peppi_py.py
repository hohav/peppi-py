from math import isclose
from pathlib import Path
from peppi_py import read_slippi, read_peppi
from peppi_py.game import *

def test_basic_game():
	game = read_slippi(Path(__file__).parent.joinpath('data/game.slp').as_posix())

	assert game.metadata == {
		'startAt': '2018-06-22T07:52:59Z',
		'lastFrame': 5085,
		'playedOn': 'dolphin',
		'players': {
			'0': {'characters': {'18': 5209}}, # Marth
			'1': {'characters': {'1': 5209}}, # Fox
		},
	}

	assert game.start == Start(
		slippi=Slippi(
			version=(1, 0, 0),
		),
		bitfield=(50, 1, 134, 76),
		is_raining_bombs=False,
		is_teams=False,
		item_spawn_frequency=-1,
		self_destruct_score=-1,
		stage=8, # Yoshi's Story
		timer=480,
		item_spawn_bitfield=(255, 255, 255, 255, 255),
		damage_ratio=1.0,
		players=(
			Player(
				port=Port.P1,
				character=9, # Marth
				type=PlayerType.HUMAN,
				stocks=4,
				costume=3,
				team=None,
				handicap=9,
				bitfield=192,
				cpu_level=None,
				offense_ratio=1.0,
				defense_ratio=1.0,
				model_scale=1.0,
				ucf=Ucf(
					dash_back=None,
					shield_drop=None,
				),
				name_tag=None,
				netplay=None,
			),
			Player(
				port=Port.P2,
				character=2, # Fox
				type=PlayerType.CPU,
				stocks=4,
				costume=0,
				team=None,
				handicap=9,
				bitfield=64,
				cpu_level=1,
				offense_ratio=1.0,
				defense_ratio=1.0,
				model_scale=1.0,
				ucf=Ucf(
					dash_back=None,
					shield_drop=None,
				),
				name_tag=None,
				netplay=None,
			),
		),
		random_seed=3803194226,
		is_pal=None,
		is_frozen_ps=None,
		scene=None,
		language=None,
		match=None,
	)

	assert game.end == End(
		method=EndMethod.RESOLVED,
		lras_initiator=None,
		players=None,
	)

	assert len(game.frames.id) == 5209
	p1 = game.frames.ports[0].leader.pre
	p2 = game.frames.ports[1].leader.pre
	assert p1.position.x[1000].as_py() == 56.818748474121094
	assert p1.position.y[1000].as_py() == -18.6373291015625
	assert p2.position.x[1000].as_py() == 42.195167541503906
	assert p2.position.y[1000].as_py() == 9.287015914916992
