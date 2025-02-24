use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Represents possible subtypes of 'character' cards.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum CharacterType {
    Warrior,
    Survivor,
    SpiritAnimal,
}
