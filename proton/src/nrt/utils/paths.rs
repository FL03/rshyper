/*
    Appellation: paths <module>
    Contrib: @FL03
*/
use crate::nrt::{LPR, Triad};

/// Determine if there's a single transformation between two triads
pub fn get_transformation(triad1: &Triad, triad2: &Triad) -> Option<LPR> {
    // Leading transformation
    let leading_result = triad1.transform(LPR::Leading);
    if leading_result == *triad2 {
        return Some(LPR::Leading);
    }

    // Parallel transformation
    let parallel_result = triad1.transform(LPR::Parallel);
    if parallel_result == *triad2 {
        return Some(LPR::Parallel);
    }

    // Relative transformation
    let relative_result = triad1.transform(LPR::Relative);
    if relative_result == *triad2 {
        return Some(LPR::Relative);
    }

    None
}
