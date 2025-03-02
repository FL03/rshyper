/*
    Appellation: traid <module>
    Contrib: @FL03
*/
use crate::TriadClass;
use crate::models::WolframUTM;

/// Data associated with a hyperedge (triad) in the Tonnetz
#[derive(Debug, Clone)]
pub struct Triad {
    /// The pitch classes forming this triad
    pub pitches: [usize; 3],
    /// The type of triad (Major, Minor, Augmented, Diminished)
    pub class: TriadClass,
    /// Optional UTM instance operating on this triad
    pub utm: Option<WolframUTM>,
}
