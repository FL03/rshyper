/*
    Appellation: traid <module>
    Contrib: @FL03
*/
use crate::{LPR, PyMod, TriadClass};

use super::Factors;

/// A triad is a particular chord composed of three notes that satify particular intervallic
/// constrains with each other. Here, the triad materializes the facet of a hyperedge within a
/// cluster of triads persisted in the Tonnetz. The triad is a fundamental entity in the
/// substrate used to represent the _headspace_ of a plant. Each plant relies on these objects
/// to transverse the surface of the tonnetz so that it may gaurantee the completion of a task.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde_derive::Deserialize, serde_derive::Serialize)
)]
pub struct Triad {
    /// The type of triad (Major, Minor, Augmented, Diminished)
    pub(crate) class: TriadClass,
    /// The pitch classes forming this triad
    pub(crate) notes: [usize; 3],
}

impl Triad {
    pub fn new(notes: [usize; 3], class: TriadClass) -> Self {
        if !class.validate(&notes) {
            panic!("Invalid triad pitches for class {notes:?}");
        }
        Triad { notes, class }
    }
    /// Create a new triad from a root pitch and class
    pub fn from_root(root: usize, class: TriadClass) -> Self {
        let [a, .., c] = class.intervals();
        let third = (root + a).pymod(12);
        let fifth = (root + c).pymod(12);
        Triad {
            notes: [root, third, fifth],
            class,
        }
    }
    /// creates a new augmented triad from the given root
    pub fn augmented(root: usize) -> Self {
        Self::from_root(root, TriadClass::Augmented)
    }
    /// creates a new diminished triad from the given root
    pub fn diminished(root: usize) -> Self {
        Self::from_root(root, TriadClass::Diminished)
    }
    /// Create a new major triad from the given root
    pub fn major(root: usize) -> Self {
        Self::from_root(root, TriadClass::Major)
    }
    /// creates a new minor triad from the given root
    pub fn minor(root: usize) -> Self {
        Self::from_root(root, TriadClass::Minor)
    }
    /// returns a copy of the class of the triad
    pub fn class(&self) -> TriadClass {
        self.class
    }
    /// returns an immutable reference to the notes of the triad
    pub const fn notes(&self) -> &[usize; 3] {
        &self.notes
    }
    /// returns a mutable reference to the notes of the triad
    pub fn notes_mut(&mut self) -> &mut [usize; 3] {
        &mut self.notes
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
    /// check if the triad contains a given pitch class
    pub fn contains<Q>(&self, pitch: &Q) -> bool
    where
        Q: core::borrow::Borrow<usize>,
    {
        self.notes().contains(pitch.borrow())
    }
    /// returns true if the pitches within the triad match its classification
    pub fn is_valid(&self) -> bool {
        self.class().validate(self.notes())
    }
    /// Apply the leading transformation to the triad
    pub fn leading(&self) -> Self {
        self.transform(LPR::Leading)
    }
    /// Apply the parallel transformation to the triad
    pub fn parallel(&self) -> Self {
        self.transform(LPR::Parallel)
    }
    /// Apply the relative transformation to the triad
    pub fn relative(&self) -> Self {
        self.transform(LPR::Relative)
    }
    /// Apply a transformation to a triad
    pub fn transform(&self, transform: LPR) -> Self {
        transform.apply(self)
    }
}

impl Default for Triad {
    fn default() -> Self {
        Triad {
            class: TriadClass::Major,
            notes: [0, 4, 7],
        }
    }
}

impl core::convert::AsRef<[usize; 3]> for Triad {
    fn as_ref(&self) -> &[usize; 3] {
        &self.notes
    }
}

impl core::convert::AsMut<[usize; 3]> for Triad {
    fn as_mut(&mut self) -> &mut [usize; 3] {
        &mut self.notes
    }
}

impl core::ops::Index<Factors> for Triad {
    type Output = usize;
    fn index(&self, index: Factors) -> &Self::Output {
        &self.notes[index as usize]
    }
}

impl core::ops::IndexMut<Factors> for Triad {
    fn index_mut(&mut self, index: Factors) -> &mut Self::Output {
        &mut self.notes[index as usize]
    }
}

impl core::ops::Mul<LPR> for Triad {
    type Output = Self;

    fn mul(self, rhs: LPR) -> Self::Output {
        rhs.apply(&self)
    }
}

impl core::ops::MulAssign<LPR> for Triad {
    fn mul_assign(&mut self, rhs: LPR) {
        *self = rhs.apply(self);
    }
}
