/*
    Appellation: classes <module>
    Contrib: @FL03
*/

// Expanded triad types in Neo-Riemannian theory
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
    serde(rename_all = "lowercase")
)]
#[strum(serialize_all = "lowercase")]
pub enum TriadType {
    #[default]
    Major,
    Minor,
    Augmented,
    Diminished,
}

/// Enumerates the available transformations in Neo-Riemannian theory
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
    serde(rename_all = "lowercase")
)]
#[strum(serialize_all = "snake_case")]
pub enum Transformation {
    #[default]
    #[serde(alias = "L", alias = "l")]
    Leading,  // L: Maps major to minor and vice versa
    #[serde(alias = "P", alias = "p")]
    Parallel, // P: Maps major to parallel minor and vice versa
    #[serde(alias = "R", alias = "r")]
    Relative, // R: Maps major to relative minor and vice versa
}