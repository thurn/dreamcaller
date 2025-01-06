// Copyright (c) dreamcaller 2025-present
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use serde::{Deserialize, Serialize};

use crate::activated_ability::ActivatedAbility;
use crate::effect::Effect;
use crate::static_ability::StaticAbility;
use crate::triggered_ability::TriggeredAbility;

/// An 'ability' represents a paragraph of text present on a card or a specific
/// keyword which maps to text defined by the game rules. Abilities on cards are
/// evaluated from top to bottom in order to apply their game effects.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Ability {
    /// An event ability happens immediately when an event card is played, and
    /// then the event card is discarded. Character cards cannot have
    /// 'event' abilities.
    Event(Effect),

    /// A static ability represents something which modifies the rules of the
    /// game, either for this specific card or globally. Static abilities do
    /// not 'happen', they're just something that is always true.
    Static(StaticAbility),

    /// An activated ability is present on a character card and allows the
    /// controlling player to pay some cost in order to achieve an effect.
    /// This is written as "> cost: effect".
    Activated(ActivatedAbility),

    /// A triggered ability is an effect which happens when some triggering
    /// event occurs, typically while its card is in play. Indicated in card
    /// text by "When", "Whenever", "At", or by a trigger keyword.
    Triggered(TriggeredAbility),
}
