use enums::FlagUpdateType;
use types::{Flag, Player, Position};

/// Update position of flag in CTF
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct GameFlag {
	#[serde(rename = "type")]
	pub ty: FlagUpdateType,
	pub flag: Flag,
	pub id: Option<Player>,
	pub pos: Position,
	/// Blue team score
	pub blueteam: u8,
	/// Red team score
	pub redteam: u8,
}