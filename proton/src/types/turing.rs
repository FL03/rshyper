/*
    Appellation: direction <module>
    Contrib: @FL03
*/
use rstm::{Head, Tail};

pub type RuleSet<Q = usize, S = usize> = std::collections::HashMap<Head<Q, S>, Tail<Q, S>>;
