use super::Id;
use bevy::{asset::Handle, prelude::*, reflect::TypeUuid};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, Reflect)]
#[serde(deny_unknown_fields)]
pub enum ItemCategory {
    Consumable,
    Lore,
    Generic,
}

#[derive(Clone, Debug, Deserialize, TypeUuid, Reflect)]
#[uuid = "45dcd38b-6b8d-4938-b8c0-338547cc552f"]
#[serde(deny_unknown_fields)]
pub struct ItemType {
    name: Id<()>,
    category: ItemCategory,
}

#[derive(Debug, Deref, DerefMut, Reflect)]
pub struct ItemContainer {
    #[deref]
    contained: Vec<(Handle<ItemType>, u16)>,
}

#[derive(Clone, Component, Debug, Reflect)]
pub struct Collectible {
    item_type: Handle<ItemType>,
    amount: u16,
}
