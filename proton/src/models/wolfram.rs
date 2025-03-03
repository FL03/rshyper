/*
    Appellation: wolfram <module>
    Contrib: @FL03
*/
use crate::RuleSet;

use rstm::{Direction, Head, State, Tail};

/// An implementation of the Wolfram [2, 3] UTM that consideres two states and three symbols
#[derive(Clone, Debug, PartialEq)]
pub struct WolframUTM {
    // the alphabet of the machine
    pub(crate) alphabet: [usize; 3],
    // current position of the head of the machine
    pub(crate) position: usize,
    // a set of rules mapping one head to another using some shift
    pub(crate) ruleset: RuleSet<usize, usize>,
    // the current state of the machine
    pub(crate) state: State<usize>,
    // the tape, or memory, of the machine
    pub(crate) tape: Vec<usize>,
}

impl WolframUTM {
    // Create a new UTM with a specific program
    pub fn new(alphabet: [usize; 3], State(state): State<usize>) -> Self {
        WolframUTM {
            alphabet,
            position: 0,
            state: State(state),
            tape: Vec::new(),
            ruleset: RuleSet::new(),
        }
    }
    /// create a new instance with the given alphabet and default state
    pub fn from_alphabet(alphabet: [usize; 3]) -> Self {
        Self {
            alphabet,
            position: 0,
            state: State::default(),
            tape: Vec::new(),
            ruleset: RuleSet::new(),
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
    /// returns a copy of the machine's current state
    #[inline]
    pub fn state(&self) -> State<usize> {
        self.state
    }
    /// returns a mutable reference to the machine's state
    pub fn state_mut(&mut self) -> State<&mut usize> {
        self.state.to_mut()
    }
    /// returns an immutable reference to the tape
    pub const fn tape(&self) -> &Vec<usize> {
        &self.tape
    }
    /// returns a mutable reference to the tape
    pub fn tape_mut(&mut self) -> &mut Vec<usize> {
        &mut self.tape
    }

    /// returns the head of the machine
    pub fn head(&self) -> Head<usize, usize> {
        Head::new(self.state, self.get_current_symbol())
    }
    /// check if a symbol is in the alphabet
    pub fn contains(&self, symbol: usize) -> bool {
        self.alphabet.contains(&symbol)
    }
    /// get the current symbol on the tape
    pub fn get_current_symbol(&self) -> usize {
        if self.position < self.tape.len() {
            self.tape[self.position]
        } else {
            // Default to 0 if we're off the tape
            <usize>::default()
        }
    }
    /// execute one step of the machine
    pub fn step(&mut self) -> bool {
        let symbol = self.get_current_symbol();

        // Check if the current symbol is in our current triad context
        if !self.contains(symbol) {
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

    /// process some input
    pub fn run(&mut self, input: Vec<usize>) -> Vec<(State<usize>, Vec<usize>, [usize; 3])> {
        // Initialize the tape with the input
        self.tape = input;
        self.position = 0;

        let mut history = Vec::new();

        // Record initial state
        history.push((self.state(), self.tape().clone(), *self.alphabet()));

        // Run until the machine halts (i.e., step() returns false)
        while self.step() {
            // Record each state after a step
            history.push((self.state(), self.tape().clone(), *self.alphabet()));

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
    /// returns a string representation of the machine
    pub fn pretty_print(&self) -> String {
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
            " (State: {}, Alphabet: {:?})",
            self.state, self.alphabet
        ));

        result
    }
    /// set the alphabet of the machine
    pub fn set_alphabet(&mut self, alphabet: [usize; 3]) {
        self.alphabet = alphabet;
    }
    /// set the ruleset of the machine
    pub fn set_ruleset(&mut self, ruleset: RuleSet<usize, usize>) {
        self.ruleset = ruleset;
    }
    /// set the state of the machine
    pub fn set_state(&mut self, State(state): State<usize>) {
        self.state = State(state);
    }
    /// consumes this instance to create another with the given alphabet
    pub fn with_alphabet(self, alphabet: [usize; 3]) -> Self {
        Self { alphabet, ..self }
    }
    /// consumes this instance to create another with the given ruleset
    pub fn with_ruleset(self, ruleset: RuleSet<usize, usize>) -> Self {
        Self { ruleset, ..self }
    }
    /// consumes this instance to create another with the given state
    pub fn with_state(self, State(state): State<usize>) -> Self {
        Self {
            state: State(state),
            ..self
        }
    }
}

impl core::fmt::Display for WolframUTM {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "Wolfram UTM: Alphabet: {:?} State {}, Tape: {:?}, Position: {}",
            self.alphabet, self.state, self.tape, self.position,
        )
    }
}

impl core::ops::Index<usize> for WolframUTM {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tape[index]
    }
}
