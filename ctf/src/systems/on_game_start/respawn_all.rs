use specs::*;

use server::component::flag::*;
use server::systems::handlers::game::on_join::AllJoinHandlers;
use server::types::systemdata::PlayerRespawner;
use server::*;

use component::*;
use systems::timer::GameStart;

use super::RespawnAllUnspec;

/// Respawn all players at the start of
/// the game.
#[derive(Default)]
pub struct RespawnAll {
	reader: Option<OnGameStartReader>,
}

#[derive(SystemData)]
pub struct RespawnAllData<'a> {
	channel: Read<'a, OnGameStart>,
	respawner: PlayerRespawner<'a>,

	entities: Entities<'a>,
	is_player: ReadStorage<'a, IsPlayer>,
}

impl<'a> System<'a> for RespawnAll {
	type SystemData = RespawnAllData<'a>;

	fn setup(&mut self, res: &mut Resources) {
		Self::SystemData::setup(res);

		self.reader = Some(res.fetch_mut::<OnGameStart>().register_reader());
	}

	fn run(&mut self, mut data: Self::SystemData) {
		for _ in data.channel.read(self.reader.as_mut().unwrap()) {
			let players = (&*data.entities, data.is_player.mask())
				.join()
				.map(|(ent, ..)| ent);

			data.respawner.respawn_players(players);
		}
	}
}

impl SystemInfo for RespawnAll {
	type Dependencies = (
		// We need to depend on RespawnAllUnspec
		// since PlayerRespawner will only
		// send out the PlayerRespawn packet
		// if the player is not also spectating.
		// While in most cases that is the desired
		// behaviour, it doesn't work when we're
		// planning to drop all players out of spec
		// too unless they're dropped out of spec
		// first, which is why this dependency is here.
		RespawnAllUnspec,
		// PlayerRespawn accesses position by entity,
		// there's a race condition in ordering here
		// if a player joins exactly as a game is starting.
		AllJoinHandlers,
		// We want to run in the same frame as the
		// GameStart event is triggered.
		GameStart
	);

	fn name() -> &'static str {
		concat!(module_path!(), "::", line!())
	}

	fn new() -> Self {
		Self::default()
	}
}