/*
    Appellation: traid <module>
    Contrib: @FL03
*/
use crate::{PyMod, Transformation, TriadClass};

use super::Factors;

/// Data associated with a hyperedge (triad) in the Tonnetz
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Deserialize, serde_derive::Serialize)
)]
pub struct Triad {
    /// The type of triad (Major, Minor, Augmented, Diminished)
    pub(crate) class: TriadClass,
    /// The pitch classes forming this triad
    pub(crate) pitches: [usize; 3],
}

impl Triad {
    pub fn new(pitches: [usize; 3], class: TriadClass) -> Self {
        Triad { pitches, class }
    }
    /// Create a new triad from a root pitch and class
    pub fn from_root(root: usize, class: TriadClass) -> Self {
        let [a, .., c] = class.intervals();
        let third = (root + a).pymod(12);
        let fifth = (root + c).pymod(12);
        Triad {
            pitches: [root, third, fifth],
            class,
        }
    }
    /// returns true if the pitches within the triad match its classification
    pub fn is_valid(&self) -> bool {
        let [a, b, c] = self.class.intervals();
        let [root, third, fifth] = self.pitches;
        let x = (third - root).pymod(12);
        let y = (fifth - third).pymod(12);
        let z = (fifth - root).pymod(12);
        x == a && y == b && z == c
    }
    /// returns a copy of the class of the triad
    pub fn class(&self) -> TriadClass {
        self.class
    }
    /// returns an immutable reference to the pitches of the triad
    pub const fn pitches(&self) -> &[usize; 3] {
        &self.pitches
    }
    /// returns a copy of the root pitch of the triad
    pub fn root(&self) -> usize {
        self[Factors::Root]
    }
    /// returns a copy of the third chord factor within the triad
    pub fn third(&self) -> usize {
        self[Factors::Third]
    }
    /// returns a copy of the fifth chord factor within the triad
    pub fn fifth(&self) -> usize {
        self[Factors::Fifth]
    }
    /// Check if the triad contains a given pitch class
    pub fn contains<Q>(&self, pitch: &Q) -> bool
    where
        Q: core::borrow::Borrow<usize>,
    {
        self.pitches().contains(pitch.borrow())
    }
    /// Apply the leading transformation to the triad
    pub fn leading(&self) -> Self {
        self.transform(Transformation::Leading)
    }
    /// Apply the parallel transformation to the triad
    pub fn parallel(&self) -> Self {
        self.transform(Transformation::Parallel)
    }
    /// Apply the relative transformation to the triad
    pub fn relative(&self) -> Self {
        self.transform(Transformation::Relative)
    }
    /// Apply a transformation to a triad
    pub fn transform(&self, transform: Transformation) -> Self {
        transform.apply(self)
    }
}

impl Default for Triad {
    fn default() -> Self {
        Triad {
            class: TriadClass::Major,
            pitches: [0, 4, 7],
        }
    }
}

impl core::ops::Index<Factors> for Triad {
    type Output = usize;
    fn index(&self, index: Factors) -> &Self::Output {
        &self.pitches[index as usize]
    }
}

impl core::ops::IndexMut<Factors> for Triad {
    fn index_mut(&mut self, index: Factors) -> &mut Self::Output {
        &mut self.pitches[index as usize]
    }
}
