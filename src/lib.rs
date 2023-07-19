pub mod creature;
pub mod item;
pub mod tools;

pub mod prelude {
    use super::*;

    pub use creature::{Move, MovementPlugin, Player};
    pub use item::{AttributeSet, ItemType};
    pub use tools::ConsolePlugin;
}
