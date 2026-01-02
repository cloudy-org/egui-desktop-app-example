use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Default, Hash, Clone)]
pub struct Game {
    #[serde(default)]
    pub achievements: Achievements,
}

#[derive(Deserialize, Serialize, Hash, Clone)]
pub struct Achievements {
    #[serde(default = "super::true_default")]
    pub enable: bool
}

impl Default for Achievements {
    fn default() -> Self {
        Self {
            enable: true
        }
    }
}
