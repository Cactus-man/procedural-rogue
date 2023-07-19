use bevy::prelude::*;
use bevy::reflect::{Reflect, TypeUuid};
use hashbrown::HashSet;
use serde::Deserialize;

#[derive(Clone, Debug, Default, Deref, DerefMut, Deserialize, Reflect, Resource, TypeUuid)]
#[uuid = "a165fc05-caec-4c71-8e83-5d1fd5891b83"]
#[serde(deny_unknown_fields)]
pub struct AttributeSet {
    namespace: String,
    #[deref]
    attributes: HashSet<String>,
}

impl AttributeSet {
    const PREFILLED: &[&'static str] = &[
        "Weapon.Damage",
        "Weapon.AttackSpeed",
        "Weapon.Range",
        "Weapon.Knock",
        "Magic.Damage",
        "Magic.ManaCost",
        "Magic.Efficacy",
        "Projectile.Velocity",
        "Armor.Protection",
        "Armor.Resistance",
        "Armor.Hardness",
    ];

    pub fn new_prefilled() -> Self {
        Self {
            namespace: "Base".to_owned(),
            attributes: HashSet::from_iter(Self::PREFILLED.into_iter().map(ToString::to_string)),
        }
    }
}

#[derive(Debug, Reflect)]
pub enum Modifier {
    Increase(f32),
    Multiply(f32),
    IncreaseTimed(f32, Timer),
    MultiplyTimed(f32, Timer),
}

#[derive(Debug, Reflect)]
pub struct Attribute {
    base: f32,
    modifiers: Vec<Modifier>,
}
