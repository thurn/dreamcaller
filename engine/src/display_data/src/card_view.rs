use core_data::types::{CardFacing, Url};
use serde::{Deserialize, Serialize};
use specta::Type;

use crate::object_position::ObjectPosition;

/// Represents the visual state of a card or ability in a game
#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct CardView {
    /// Identifier for this card
    pub id: ClientCardId,

    /// Position of this card in the UI
    pub position: ObjectPosition,

    /// Card back image
    pub card_back: Url,

    /// If this card is revealed to the viewer, contains information on the
    /// revealed face of the card.
    pub revealed: Option<RevealedCardView>,

    /// True if this card is in a hidden zone but known to one or more opponents
    pub revealed_to_opponents: bool,

    /// Face up/face down state for this card
    pub card_facing: CardFacing,

    /// Optionally, a position at which to create this card.
    ///
    /// If this card does not already exist, it will be created at this position
    /// before being animated to [Self::position].
    pub create_position: Option<ObjectPosition>,

    /// Optionally, a position at which to destroy this card.
    ///
    /// If provided, the card will be animated to this position before being
    /// destroyed.
    pub destroy_position: Option<ObjectPosition>,
}

/// Identifies a card in client code
///
/// Client-opaque serialized value.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum ClientCardId {
    CardId(String),
    ActivatedAbilityId(String),
    TriggeredAbilityId(String),
}

/// Visual state of a revealed card
#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct RevealedCardView {
    /// Image URL for this card
    pub image: Url,

    /// Name of this card
    pub name: String,

    /// Rules text to display for this face
    pub rules_text: String,

    /// Visual status of this card
    pub status: Option<RevealedCardStatus>,

    /// True if this card represents an ability
    pub is_ability: bool,

    /// True if this card represents a token
    pub is_token: bool,

    /// True if this card can be dragged by the player.
    ///
    /// The set of valid drag targets is set on the GameView. All draggable
    /// cards can be dragged to and reordered within any valid target.
    pub can_drag: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub enum RevealedCardStatus {
    Selected,
    CanSelect,
    CanPlay,
}
