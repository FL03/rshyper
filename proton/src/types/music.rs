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
pub enum TriadClass {
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
#[strum(serialize_all = "lowercase")]
pub enum Transformation {
    #[default]
    #[serde(alias = "L", alias = "l")]
    Leading,  // L: Maps major to minor and vice versa
    #[serde(alias = "P", alias = "p")]
    Parallel, // P: Maps major to parallel minor and vice versa
    #[serde(alias = "R", alias = "r")]
    Relative, // R: Maps major to relative minor and vice versa
}

impl Transformation {
    pub fn leading() -> Self {
        Transformation::Leading
    }

    pub fn parallel() -> Self {
        Transformation::Parallel
    }

    pub fn relative() -> Self {
        Transformation::Relative
    }
}

impl TriadClass {
    /// a functional constructor for the major triad type
    pub fn major() -> Self {
        TriadClass::Major
    }
    /// a functional constructor for the minor triad type
    pub fn minor() -> Self {
        TriadClass::Minor
    }
    /// a functional constructor for the augmented triad type
    pub fn augmented() -> Self {
        TriadClass::Augmented
    }
    /// a functional constructor for the diminished triad type
    pub fn diminished() -> Self {
        TriadClass::Diminished
    }
    /// get the relative triad type
    pub fn relative(self) -> Self {
        match self {
            TriadClass::Major => TriadClass::Minor,
            TriadClass::Minor => TriadClass::Major,
            TriadClass::Augmented => TriadClass::Diminished,
            TriadClass::Diminished => TriadClass::Augmented,
        }
    }
}
