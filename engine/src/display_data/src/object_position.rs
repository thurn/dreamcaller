use core_data::identifiers::CardId;
use core_data::types::PlayerName;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Represents the position of some object in the UI
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ObjectPosition {
    /// Position category
    pub position: Position,
    /// Sorting key, determines order within the position
    pub sorting_key: u32,
    /// Sub-key, used to break ties in sorting
    pub sorting_sub_key: u32,
}

impl Default for ObjectPosition {
    fn default() -> Self {
        Self { position: Position::Default, sorting_key: 0, sorting_sub_key: 0 }
    }
}

/// Possible types of display positions
#[derive(
    Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Ord, PartialOrd, JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum Position {
    /// Object position used in interface elements like the deck viewer which
    /// don't rely on game positioning.
    Default,

    /// Object is not visible.
    Offscreen,

    /// The user is selecting targets for this card from among characters
    /// controlled by the indicated player.
    SelectingTargets(PlayerName),

    /// Object is on the stack
    OnStack,

    /// Position for cards to be shown to the user immediately after they're
    /// drawn.
    Drawn,

    /// Object is in a player's hand
    InHand(PlayerName),

    /// Object is on top of a player's deck
    OnTopOfDeck(PlayerName),

    /// Object is shuffled into a player's deck
    InDeck(PlayerName),

    /// Object is in a player's void
    InVoid(PlayerName),

    /// Object is in this player's banished zone
    InBanished(PlayerName),

    /// Object is on the battlefield
    OnBattlefield(PlayerName),

    /// Object is in the user's status zone
    InUserStatus(PlayerName),

    /// Object is being displayed in a card browser, e.g. to select from a list
    /// of cards while searching
    Browser,

    /// Object is being displayed in a list of cards available to select in a
    /// card selector.
    CardSelectionChoices,

    /// Object is in a temporary holding space for cards in hand while resolving
    /// some other 'play card' ability.
    HandStorage,

    /// Object is hidden within a card
    HiddenWithinCard(CardId),
}
