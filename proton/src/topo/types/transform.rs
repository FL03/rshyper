/*
    Appellation: transform <module>
    Contrib: @FL03
*/
use crate::topo::{Triad, TriadClass};

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
    Leading, // L: Maps major to minor and vice versa
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

    /// Apply a transformation to a triad
    pub fn apply_transform(&self, triad: &Triad) -> Triad {
        use crate::ops::PyMod;
        let &Triad { class, pitches } = triad;

        match class {
            TriadClass::Major => match self {
                Transformation::Leading => {
                    let [x, y, z] = pitches;
                    Triad::new(
                        [y, z, (x as isize - 1).pymod(12) as usize],
                        TriadClass::Minor,
                    )
                }
                Transformation::Parallel => {
                    let [x, y, z] = pitches;
                    Triad::new(
                        [x, (y as isize - 1).pymod(12) as usize, z],
                        TriadClass::Minor,
                    )
                }
                Transformation::Relative => {
                    let [x, y, z] = pitches;
                    Triad::new(
                        [x, (y as isize + 2).pymod(12) as usize, z],
                        TriadClass::Minor,
                    )
                }
            },
            TriadClass::Minor => match self {
                Transformation::Leading => {
                    let [x, y, z] = pitches;
                    Triad::new(
                        [(z as isize + 1).pymod(12) as usize, x, y],
                        TriadClass::Major,
                    )
                }
                Transformation::Parallel => {
                    let [x, y, z] = pitches;
                    Triad::new(
                        [y, z, (x as isize + 1).pymod(12) as usize],
                        TriadClass::Major,
                    )
                }
                Transformation::Relative => {
                    let [x, y, z] = pitches;
                    Triad::new(
                        [y, z, (x as isize - 2).pymod(12) as usize],
                        TriadClass::Major,
                    )
                }
            },
            _ => triad.clone(),
        }
    }
}

impl From<usize> for Transformation {
    fn from(value: usize) -> Self {
        match value {
            0 => Transformation::Leading,
            1 => Transformation::Parallel,
            2 => Transformation::Relative,
            _ => Transformation::Leading,
        }
    }
}
