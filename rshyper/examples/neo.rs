use std::collections::HashMap;

/*
    Appellation: neo <example>
    Contrib: @FL03
*/
#[allow(unused_imports)]
use rshyper::{HyperGraph, Node};
use utm::{demonstrate_tonnetz_and_utm, State, Symbol};

fn main() {
    use utm::Symbol::*;
    // Create a hypergraph of &str to keep the example simple
    let mut graph = HyperGraph::new();

    // Add our "notes" as vertices
    let v0 = graph.add_vertex(C);
    let v1 = graph.add_vertex(E);
    let v2 = graph.add_vertex(G);

    // Here we define a hyperedge as a "facet" linking three symbols (a triad)
    let e1 = graph.add_hyperedge(vec![v0, v1, v2]).expect("triad edge");

    // In a real system, you could store additional data about states & triad
    let facet_data = MachineFacet {
        states: (State::S0, State::S1),
        alphabet: (Symbol::A, Symbol::B, Symbol::C),
    };
    println!("Facet data for hyperedge {e1:?}: {:?}", facet_data);

    // let machine = TM::new(Head { state: State::S0, symbol: A }, vec![A, B, C]).with_rules(sample_rulest());

    // for (head, position) in machine {
    //     println!("Step => head: {head:?}, head_position: {position}");
    // }

    // Demonstrate how this triad serves as a "headspace" of a Wolfram [2, 3] UTM
    let triad = graph
        .get_edge_nodes(e1)
        .expect("triad vertices")
        .iter()
        .map(|&v| v.weight())
        .collect::<Vec<_>>();
    demonstrate_tonnetz_and_utm(triad);
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Head<Q = State, S = Symbol> {
    pub state: Q,
    pub symbol: S,
}
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TM {
    pub head: Head,
    pub tape: Vec<Symbol>,
    pub position: usize,
    pub rules: HashMap<Head, (Head, &'static str)>,
}

impl TM {
    pub fn new(head: Head, tape: Vec<Symbol>) -> Self {
        Self {
            head,
            tape,
            position: 0,
            rules: HashMap::new(),
        }
    }

    pub fn with_rules(self, rules: HashMap<Head, (Head, &'static str)>) -> Self {
        Self { rules, ..self }
    }


    pub fn tape(&self) -> &[Symbol] {
        &self.tape
    }

    pub fn tape_mut(&mut self) -> &mut Vec<Symbol> {
        &mut self.tape
    }
}

impl core::iter::Iterator for TM {
    type Item = (Head, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let &(head, dir) = self.rules.get(&self.head)?;
        self.tape[self.position] = head.symbol;
        self.head = head;

        // Move head (no bounds checking for brevity)
        match dir {
            "L" => self.position = self.position.checked_sub(1)?,
            "R" => self.position += 1,
            _ => {}
        }

        Some((head, self.position))
    }
}

#[derive(Debug)]
pub struct MachineFacet {
    pub states: (State, State),
    pub alphabet: (Symbol, Symbol, Symbol),
}

impl MachineFacet {
    pub fn new(states: (State, State), alphabet: (Symbol, Symbol, Symbol)) -> Self {
        Self { states, alphabet }
    }
}

pub mod utm {
    use super::Head;
    // A basic representation of states for a Wolfram [2, 3] UTM
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[cfg_attr(
        feature = "serde",
        derive(serde_derive::Deserialize, serde_derive::Serialize)
    )]
    pub enum State {
        #[default]
        S0,
        S1,
    }

    // A basic representation of our symbols (three-note triad)
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    #[cfg_attr(
        feature = "serde",
        derive(serde_derive::Deserialize, serde_derive::Serialize)
    )]
    pub enum Symbol {
        #[default]
        A,
        B,
        C,
        D,
        E,
        F,
        G,
    }

    pub fn sample_rulest() -> std::collections::HashMap<Head, (Head, &'static str)> {
        use State::*;
        use Symbol::*;
        let mut rules = std::collections::HashMap::new();
        rules.insert(
            Head { state: S0, symbol: A },
            (Head { state: S1, symbol: B }, "R"),
        );
        rules.insert(
            Head { state: S0, symbol: B },
            (Head { state: S0, symbol: C }, "L"),
        );
        rules.insert(
            Head { state: S0, symbol: C },
            (Head { state: S1, symbol: A }, "R"),
        );
        rules.insert(
            Head { state: S1, symbol: A },
            (Head { state: S0, symbol: C }, "L"),
        );
        rules.insert(
            Head { state: S1, symbol: B },
            (Head { state: S1, symbol: A }, "R"),
        );
        rules.insert(
            Head { state: S1, symbol: C },
            (Head { state: S0, symbol: B }, "L"),
        );
        rules
    }

    // Example ruleset (transition function) for a Wolfram [2, 3] UTM
    // Returns (new_state, new_symbol, direction)
    // For brevity, direction is just a placeholder string here.
    pub fn wolfram_23_rule(state: State, symbol: Symbol) -> (State, Symbol, &'static str) {
        use State::*;
        use Symbol::*;
        match (state, symbol) {
            (S0, A) => (S1, B, "R"),
            (S0, B) => (S0, C, "L"),
            (S0, C) => (S1, A, "R"),
            (S1, A) => (S0, C, "L"),
            (S1, B) => (S1, A, "R"),
            (S1, C) => (S0, B, "L"),
            _ => unreachable!("Invalid state/symbol combination"),
        }
    }

    pub fn demonstrate_tonnetz_and_utm(edge: Vec<&Symbol>) {
        // Initial machine configuration
        let mut current_state = State::S0;
        let mut tape = vec![Symbol::A, Symbol::B, Symbol::C]; // simplistic tape
        let mut head_position = 1; // start in the middle

        println!(
            "Initial state: {:?}, tape: {:?}, head_position: {}",
            current_state, tape, head_position
        );

        // Run a few steps
        for _step in 0..4 {
            let symbol_under_head = tape[head_position];
            let (new_state, new_symbol, dir) = wolfram_23_rule(current_state, symbol_under_head);
            tape[head_position] = new_symbol;
            current_state = new_state;

            // Move head (no bounds checking for brevity)
            match dir {
                "L" => head_position = head_position.checked_sub(1).unwrap_or(0),
                "R" => head_position += 1,
                _ => {}
            }

            println!(
                "Step => state: {:?}, tape: {:?}, head_position: {}",
                current_state, tape, head_position
            );
        }

        println!(
            "\nTriad: {edge:?} can be viewed as a facet with two sides (two states, S0 & S1)."
        );
    }
}
