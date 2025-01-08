use core_data::numerics::Energy;
use serde::{Deserialize, Serialize};

use crate::predicate::Predicate;

/// Any action a player must take in order to play a card or activate an
/// ability, such as paying energy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Cost {
    None,
    Energy(Energy),
    BanishCardsFromYourVoid(u64),
    AbandonCharacter(Predicate),
    DiscardHand,
}
