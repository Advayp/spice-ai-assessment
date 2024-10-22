use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize, Default)]
pub struct ScoreInfo {
    pub id: u64,
    // Store as string for now, convert later to date/time if necessary
    pub created_at: String,
    pub name: String,
    pub score: u64,
    pub notes: String,
}
