/*
    Appellation: traid <module>
    Contrib: @FL03
*/
use crate::{PyMod, Transformation, TriadClass};

/// Data associated with a hyperedge (triad) in the Tonnetz
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Triad {
    /// The pitch classes forming this triad
    pub pitches: [usize; 3],
    /// The type of triad (Major, Minor, Augmented, Diminished)
    pub class: TriadClass,
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
    /// returns a copy of the class of the triad
    pub fn class(&self) -> TriadClass {
        self.class
    }
    /// returns an immutable reference to the pitches of the triad
    pub const fn pithes(&self) -> &[usize; 3] {
        &self.pitches
    }

    /// Apply a transformation to a triad
    pub fn apply_transform(&self, transform: Transformation) -> Self {
        transform.apply_transform(self)
    }
    /// Check if the triad contains a given pitch class
    pub fn contains<Q>(&self, pitch: &Q) -> bool
    where
        Q: core::borrow::Borrow<usize>,
    {
        self.pitches.contains(pitch.borrow())
    }
    /// Apply the leading transformation to the triad
    pub fn leading(&self) -> Self {
        self.apply_transform(Transformation::Leading)
    }
    /// Apply the parallel transformation to the triad
    pub fn parallel(&self) -> Self {
        self.apply_transform(Transformation::Parallel)
    }
    /// Apply the relative transformation to the triad
    pub fn relative(&self) -> Self {
        self.apply_transform(Transformation::Relative)
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
