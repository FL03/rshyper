/*
    Appellation: wolfram <module>
    Contrib: @FL03
*/

use crate::utils::pymod;
use crate::{PyMod, RuleSet, Transformation, TriadClass};

use rstm::{Direction, Head, State, Tail};

/// An implementation of the Wolfram [2, 3] UTM designed to operate on a 12-tone system
#[derive(Clone, Debug, PartialEq)]
pub struct WolframUTM {
    // Current triad (our active alphabet) - using fixed-size array
    pub(crate) alphabet: [usize; 3],
    // Current position on the tape
    pub(crate) position: usize,
    // Transition rules: (current_state, current_symbol) -> (new_state, new_symbol, direction)
    pub(crate) ruleset: RuleSet<usize, usize>,
    // Current state (0 or 1)
    pub(crate) state: State<usize>,
    // Tape containing the program symbols
    pub(crate) tape: Vec<usize>,

    // Current triad type (Major, Minor, Augmented, or Diminished)
    pub(crate) class: TriadClass,
    /// a posisitional coordinate mapping the machine to a layer within the global structure
    pub(crate) modulus: usize,
}

impl WolframUTM {
    // Create a new UTM with a specific program
    pub fn new(alphabet: [usize; 3], State(state): State<usize>) -> Self {
        WolframUTM {
            alphabet,
            modulus: 0,
            position: 0,
            state: State(state),
            tape: Vec::new(),
            ruleset: RuleSet::new(),

            class: TriadClass::Major, // C-Major is our starting point
        }
    }

    pub fn from_alphabet(alphabet: [usize; 3]) -> Self {
        Self {
            alphabet,
            modulus: 0,
            position: 0,
            state: State(0),
            tape: Vec::new(),
            ruleset: RuleSet::new(),

            class: TriadClass::Major, // C-Major is our starting point
        }
    }
    /// returns the alphabet of the machine
    pub const fn alphabet(&self) -> &[usize; 3] {
        &self.alphabet
    }
    /// returns a copy of the machines current position
    #[inline]
    pub fn position(&self) -> usize {
        self.position
    }
    /// returns a copy of the machines current class
    #[inline]
    pub fn class(&self) -> TriadClass {
        self.class
    }
    #[inline]
    /// get the octave of the current machine; a modular, positional coordinate for the
    /// locating the machine in the global structure
    pub fn modulus(&self) -> usize {
        self.modulus
    }

    #[inline]
    pub fn state(&self) -> State<usize> {
        self.state
    }

    pub fn state_mut(&mut self) -> State<&mut usize> {
        self.state.to_mut()
    }

    pub const fn tape(&self) -> &Vec<usize> {
        &self.tape
    }

    pub fn set_alphabet(&mut self, alphabet: [usize; 3]) {
        self.alphabet = alphabet;
    }

    pub fn with_alphabet(self, alphabet: [usize; 3]) -> Self {
        Self { alphabet, ..self }
    }

    pub fn set_modulus(&mut self, modulus: usize) {
        self.modulus = modulus;
    }

    pub fn with_modulus(self, modulus: usize) -> Self {
        Self { modulus, ..self }
    }

    pub fn set_ruleset(&mut self, ruleset: RuleSet<usize, usize>) {
        self.ruleset = ruleset;
    }

    pub fn with_ruleset(self, ruleset: RuleSet<usize, usize>) -> Self {
        Self { ruleset, ..self }
    }

    pub fn set_state(&mut self, State(state): State<usize>) {
        self.state = State(state);
    }

    pub fn with_state(self, State(state): State<usize>) -> Self {
        Self {
            state: State(state),
            ..self
        }
    }

    pub fn head(&self) -> Head<usize, usize> {
        Head::new(self.state, self.tape[self.position])
    }

    // Check if a symbol is in the current triad
    pub fn is_valid_symbol(&self, symbol: usize) -> bool {
        self.alphabet.contains(&symbol)
    }

    // Get the current symbol under the head
    pub fn get_current_symbol(&self) -> usize {
        if self.position < self.tape.len() {
            self.tape[self.position]
        } else {
            // Default to 0 if we're off the tape
            0
        }
    }

    // Step method with deterministic transformations
    pub fn step(&mut self) -> bool {
        let symbol = self.get_current_symbol();

        // Check if the current symbol is in our current triad context
        if !self.is_valid_symbol(symbol) {
            panic!("symbol {} not in any accessible triad context", symbol);
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
            if self.position >= self.tape.len() {
                self.tape.resize(self.position + 1, 0);
            }
            self.tape[self.position] = new_symbol;

            // Update state
            self.state = new_state;

            // Move the head
            match direction {
                Direction::Left => {
                    if self.position > 0 {
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

    // Run the UTM with the given input program
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
    // Convert the machine state to string for visualization
    pub fn printed(&self) -> String {
        let mut result = String::new();

        // Print the tape with the current position marked
        for (i, &symbol) in self.tape.iter().enumerate() {
            if i == self.position {
                result.push_str(&format!("[{}]", symbol));
            } else {
                result.push_str(&format!(" {} ", symbol));
            }
        }

        // Add state and triad information
        result.push_str(&format!(
            " (State: {}, Triad: {:?} {:?})",
            self.state, self.class, self.alphabet
        ));

        result
    }
}

impl core::fmt::Display for WolframUTM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "Wolfram UTM: State {}, Tape: {:?}, Position: {}, Triad: {:?} {:?}",
            self.state, self.tape, self.position, self.class, self.alphabet
        )
    }
}

impl core::ops::Index<usize> for WolframUTM {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tape[index]
    }
}
