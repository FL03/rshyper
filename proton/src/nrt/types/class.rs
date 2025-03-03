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
    /// returns the intervals corresponding to the triad type
    pub fn intervals(self) -> [usize; 3] {
        match self {
            TriadClass::Major => [4, 3, 7],
            TriadClass::Minor => [3, 4, 7],
            TriadClass::Augmented => [4, 4, 8],
            TriadClass::Diminished => [3, 3, 6],
        }
    }
    /// returns the interval from the root to the third chord factor; defined by the class
    pub fn root_to_third(self) -> usize {
        self.intervals()[0]
    }
    /// returns the interval from the third to the fifth chord factor; defined by the class
    pub fn third_to_fifth(self) -> usize {
        self.intervals()[1]
    }
    /// returns the interval from the root to the fifth chord factor; defined by the class
    pub fn root_to_fifth(self) -> usize {
        self.intervals()[2]
    }
    /// validate a chord's composition satisfies the requirements of the current class
    pub fn validate(&self, notes: &[usize; 3]) -> bool {
        use crate::ops::PitchMod;
        let [a, b, c] = self.intervals();
        let r = notes[0] as isize;
        let t = notes[1] as isize;
        let f = notes[2] as isize;
        (t - r).pmod() as usize == a && (f - t).pmod() as usize == b && (f - r).pmod() as usize == c
    }
}

impl From<usize> for TriadClass {
    fn from(value: usize) -> Self {
        match value {
            0 => TriadClass::Major,
            1 => TriadClass::Minor,
            2 => TriadClass::Augmented,
            3 => TriadClass::Diminished,
            _ => TriadClass::Major,
        }
    }
}
