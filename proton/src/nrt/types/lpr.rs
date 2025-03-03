/*
    Appellation: transform <module>
    Contrib: @FL03
*/
use crate::nrt::{Triad, TriadClass};

/// Enumerates primary available transformations in Neo-Riemannian theory.
///
/// Each transformation is invertible, meaning that consecutive applications of any
/// transformation will return the original triad. Furthermore, LPR transformations may be
/// chained together in discrete or continuous sequences to create complex harmonic
/// progressions.
///
/// The transformations are:
///
/// - Leading (L):
///   - [Major] given a major triad, subtract a semitone from the root and move it to the fifth
///   - [Minor] given a minor triad, add a semitone to the fifth and move it to the root
/// - Parallel (P):
///   - [Major] given a major triad, subtract a semitone from the third
///   - [Minor] given a minor triad, add a semitone to the third
/// - Relative (R):
///   - [Major] given a major triad, add a tone to the fifth and move it to the root
///   - [Minor] given a minor triad, subtract a tone from the root and move it to the fifth
///
/// These transformations can be described categorically as morphisms between various triads.
/// More specifically, they are contravariant functors between categories of triads.
///
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
    strum::EnumCount,
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
pub enum LPR {
    #[default]
    #[serde(alias = "L", alias = "l")]
    Leading,
    /// L: on major, subtracts one from
    #[serde(alias = "P", alias = "p")]
    Parallel, // P: Maps major to parallel minor and vice versa
    #[serde(alias = "R", alias = "r")]
    Relative, // R: Maps major to relative minor and vice versa
}

impl LPR {
    pub fn leading() -> Self {
        LPR::Leading
    }

    pub fn parallel() -> Self {
        LPR::Parallel
    }

    pub fn relative() -> Self {
        LPR::Relative
    }

    /// Apply a transformation to a triad
    pub fn apply(&self, triad: &Triad) -> Triad {
        use crate::ops::PitchMod;

        let [x, y, z] = triad.notes;
        match triad.class() {
            TriadClass::Major => match self {
                LPR::Leading => {
                    Triad::new([y, z, (x as isize - 1).pmod() as usize], TriadClass::Minor)
                }
                LPR::Parallel => {
                    Triad::new([x, (y as isize - 1).pmod() as usize, z], TriadClass::Minor)
                }
                LPR::Relative => Triad::new([(z + 2).pmod(), x, y], TriadClass::Minor),
            },
            TriadClass::Minor => match self {
                LPR::Leading => Triad::new([(z + 1).pmod(), x, y], TriadClass::Major),
                LPR::Parallel => Triad::new([x, (y + 1).pmod(), z], TriadClass::Major),
                LPR::Relative => {
                    Triad::new([y, z, (x as isize - 2).pmod() as usize], TriadClass::Major)
                }
            },
            _ => triad.clone(),
        }
    }
}

impl From<char> for LPR {
    fn from(value: char) -> Self {
        match value.to_ascii_lowercase() {
            'l' => LPR::Leading,
            'p' => LPR::Parallel,
            'r' => LPR::Relative,
            _ => panic!("Invalid LPR transformation; character must be 'L', 'P', or 'R'"),
        }
    }
}

impl From<usize> for LPR {
    fn from(value: usize) -> Self {
        use strum::EnumCount;
        match value % Self::COUNT {
            0 => LPR::Leading,
            1 => LPR::Parallel,
            2 => LPR::Relative,
            _ => unreachable!(),
        }
    }
}
