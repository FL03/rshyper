/*
    Appellation: plant <module>
    Contrib: @FL03
*/
use crate::models::WolframUTM;
use crate::topo::{Transformation, Triad, TriadClass};
use rstm::{Direction, Head, State, Tail};

/// An isolated computational entity represented topologically in the Tonnetz using a triad;
/// the triad is used to describe the _headspace_ of the UTM, representing all possible
/// combinations of (state, symbol) pairs
#[derive(Clone, Debug, PartialEq)]
pub struct Plant {
    pub(crate) modulus: usize,
    pub triad: Triad,
    pub utm: WolframUTM,
}

impl Plant {
    pub fn new(triad: Triad) -> Self {
        let utm = WolframUTM::new(triad.pitches, State(0));
        Plant {
            modulus: 0,
            triad,
            utm,
        }
    }

    pub fn set_utm(&mut self, utm: WolframUTM) {
        self.utm = utm;
    }

    pub fn with_utm(self, utm: WolframUTM) -> Self {
        Plant { utm: utm, ..self }
    }

    pub fn apply_transform(&self, transform: Transformation) -> Self {
        let triad = self.triad.transform(transform);
        let utm = self.utm.clone().with_alphabet(triad.pitches);
        Plant {
            modulus: self.modulus,
            triad,
            utm,
        }
    }

    pub fn contains<Q>(&self, pitch: &Q) -> bool
    where
        Q: core::borrow::Borrow<usize>,
    {
        self.triad.contains(pitch)
    }

    // Step method with deterministic transformations
    pub fn step(&mut self) -> bool {
        let position = self.position();
        let symbol = self.get_current_symbol();

        // Check if the current symbol is in our current triad context
        if !self.is_valid_symbol(symbol) {
            for i in [
                Transformation::Leading,
                Transformation::Parallel,
                Transformation::Relative,
            ] {
                if i.apply(&self.triad).contains(&symbol) {
                    self.apply_transform(i);
                    break;
                }
            }
        }
        let head = Head::new(self.state, symbol);
        // Find the transition rule for the current state and symbol
        if let Some(&tail) = self.ruleset.get(&head) {
            let Tail {
                state: new_state,
                symbol: new_symbol,
                direction,
            } = tail;
            // Update the tape
            if position >= self.tape.len() {
                self.tape.resize(position + 1, 0);
            }
            self.tape[position] = new_symbol;

            // Update state
            self.state = new_state;

            // Move the head
            match direction {
                Direction::Left => {
                    if position > 0 {
                        self.position -= 1;
                    } else {
                        self.tape.insert(0, 0);
                    }
                }
                Direction::Right => {
                    self.position += 1;
                    if self.position >= self.tape.len() {
                        self.tape.push(0);
                    }
                }
                Direction::Stay => {}
            }

            true // Step successful
        } else {
            // No rule found - machine halts
            false
        }
    }

    // Run the UTM with the given input program until it halts
    pub fn run(
        &mut self,
        input: Vec<usize>,
    ) -> Vec<(State<usize>, Vec<usize>, [usize; 3], TriadClass)> {
        // Initialize the tape with the input
        self.tape = input;
        self.position = 0;

        let mut history = Vec::new();

        // Record initial state
        history.push((self.state, self.tape.clone(), self.alphabet, self.class));

        // Run until the machine halts (i.e., step() returns false)
        while self.step() {
            // Record each state after a step
            history.push((self.state, self.tape.clone(), self.alphabet, self.class));

            // Optional: Add a safety limit to prevent infinite loops in development
            if history.len() > 10000 {
                println!(
                    "Warning: Machine reached 10,000 steps without halting. Stopping execution."
                );
                break;
            }
        }

        history
    }
}

impl core::ops::Deref for Plant {
    type Target = WolframUTM;

    fn deref(&self) -> &Self::Target {
        &self.utm
    }
}

impl core::ops::DerefMut for Plant {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.utm
    }
}
