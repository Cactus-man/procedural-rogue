use super::Id;
use bevy::reflect::{Reflect, TypeUuid};
use serde::Deserialize;

pub use fragments::{Fragment, FragmentKind};

mod fragments {
    use super::*;

    #[derive(Clone, Debug, Deserialize, Reflect)]
    pub enum ComponentGroup {
        Primary,
        Secondary,
        Spell,
    }

    #[derive(Clone, Debug, Deserialize, TypeUuid, Reflect)]
    #[uuid = "9d067ecc-f489-46e8-80f5-83e0dbb33a68"]
    #[serde(deny_unknown_fields)]
    pub struct FragmentKind {
        group: ComponentGroup,
        attributes: Vec<String>,
    }

    #[derive(Clone, Debug, Reflect)]
    pub struct Fragment {
        kind: FragmentKind,
    }
}

#[derive(Clone, Debug, Deserialize, Reflect)]
pub struct WeaponKind {
    composed_by: [Id<FragmentKind>; 4],
}

impl WeaponKind {
    pub fn composition(&self) -> &[Id<FragmentKind>; 4] {
        &self.composed_by
    }
}

#[derive(Clone, Debug)]
pub struct Weapon {
    kind: WeaponKind,
}

impl Weapon {
    pub fn kind(&self) -> &WeaponKind {
        &self.kind
    }
}
