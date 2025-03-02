/*
    Appellation: direction <module>
    Contrib: @FL03
*/


pub type RuleSet<Q = usize, S = usize> = std::collections::HashMap<(Q, S), (Q, S, Direction)>;

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    strum::AsRefStr,
    strum::Display,
    strum::EnumIs,
    strum::EnumIter,
    strum::EnumString,
    strum::VariantArray,
    strum::VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Deserialize, serde_derive::Serialize),
)]
#[strum(serialize_all = "lowercase")]
pub enum Direction {
    #[serde(alias = "L", alias = "l", alias = "left", alias = "LEFT")]
    Left = -1,
    #[serde(alias = "R", alias = "r", alias = "right", alias = "RIGHT")]
    Right = 1,
    #[default]
    #[serde(alias = "S", alias = "s", alias = "stay", alias = "STAY")]
    Stay = 0,
}

impl Direction {
    pub fn left() -> Self {
        Direction::Left
    }

    pub fn right() -> Self {
        Direction::Right
    }

    pub fn stay() -> Self {
        Direction::Stay
    }
}

#[allow(unused_macros)]
macro_rules! impl_from {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Direction {
                fn from(value: $t) -> Self {
                    match value % 2 {
                        x if x < 0 => Direction::Left,
                        x if x > 0 => Direction::Right,
                        _ => Direction::Stay,
                    }
                }
            }
        )*
    };
}