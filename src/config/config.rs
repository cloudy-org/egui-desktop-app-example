use serde::{Deserialize, Serialize};
use cirrus_config::v1::config::CConfig;

use super::models::game::Game;

#[derive(Serialize, Deserialize, Default, Hash, Clone)]
pub struct Config {
    #[serde(default)]
    pub version: i8,
    #[serde(default)]
    pub game: Game,
}

impl CConfig for Config {}
