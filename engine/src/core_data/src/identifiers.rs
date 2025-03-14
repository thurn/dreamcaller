use schemars::gen::SchemaGenerator;
use schemars::schema::{InstanceType, Schema, SchemaObject};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use slotmap::{new_key_type, Key, KeyData};
use uuid::Uuid;

/// A User ID
#[derive(
    Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub struct UserId(pub Uuid);

/// A Battle ID
#[derive(
    Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub struct BattleId(pub Uuid);

new_key_type! {
    /// Identifies a card or card-like object such as:
    ///
    /// - A normal card
    /// - A copy of a card on the stack
    /// - A token or copy of a card in play
    pub struct CardId;
}

impl JsonSchema for CardId {
    fn schema_name() -> String {
        "CardId".to_string()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        let mut schema =
            SchemaObject { instance_type: Some(InstanceType::Object.into()), ..Default::default() };
        let obj = schema.object();
        obj.required.insert("idx".to_owned());
        obj.required.insert("version".to_owned());
        obj.properties.insert("idx".to_owned(), <u32>::json_schema(gen));
        obj.properties.insert("version".to_owned(), <u32>::json_schema(gen));
        schema.into()
    }
}

impl CardId {
    /// Converts an opaque number received from [Self::to_int] into a card
    /// id
    pub fn from_int(value: u64) -> Self {
        KeyData::from_ffi(value).into()
    }

    /// Returns an opaque number which can later be converted back into a card
    /// id
    pub fn to_int(&self) -> u64 {
        self.data().as_ffi()
    }
}
