/*
    Appellation: wolfram <module>
    Contrib: @FL03
*/

use crate::types::{Direction, Transformation, TriadClass};
use crate::utils::pymod;
use std::collections::HashMap;

// Define the structure for our Wolfram UTM with two states and three symbols
#[derive(Clone, Debug)]
pub struct WolframUTM {
    // Current state (0 or 1)
    pub(crate) state: usize,

    // Tape containing the program symbols
    pub(crate) tape: Vec<usize>,

    // Current position on the tape
    pub(crate) position: usize,

    // Current triad (our active alphabet) - using fixed-size array
    pub(crate) alphabet: [usize; 3],

    // Current triad type (Major, Minor, Augmented, or Diminished)
    pub(crate) class: TriadClass,

    // Transition rules: (current_state, current_symbol) -> (new_state, new_symbol, direction)
    pub(crate) transition_rules: HashMap<(usize, usize), (usize, usize, Direction)>,

    // Modulus for symbol space (12 for standard musical system)
    pub(crate) modulus: usize,
}

impl WolframUTM {
    // Create a new UTM with a specific program
    pub fn new(
        ruleset: HashMap<(usize, usize), (usize, usize, Direction)>,
        modulus: usize,
    ) -> Self {
        // Define our initial C-Major triad [0, 4, 7]
        let initial_triad = [0, 4, 7];

        WolframUTM {
            state: 0,
            tape: Vec::new(),
            position: 0,
            alphabet: initial_triad,
            class: TriadClass::Major, // C-Major is our starting point
            transition_rules: ruleset,
            modulus,
        }
    }

    pub const fn alphabet(&self) -> &[usize; 3] {
        &self.alphabet
    }
    /// get the position of the head on the tape
    #[inline]
    pub fn position(&self) -> usize {
        self.position
    }
    /// returns the current class of the alphabet
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
    pub fn state(&self) -> usize {
        self.state
    }

    pub const fn tape(&self) -> &Vec<usize> {
        &self.tape
    }

    // Calculate interval between two notes in a 12-tone system
    pub fn calculate_interval(&self, a: usize, b: usize) -> usize {
        pymod(b as isize - a as isize, self.modulus as isize) as usize
    }

    // Determine triad type based on the intervals
    pub fn classify_triad(&self, triad: &[usize; 3]) -> TriadClass {
        let interval1 = self.calculate_interval(triad[0], triad[1]);
        let interval2 = self.calculate_interval(triad[1], triad[2]);

        match (interval1, interval2) {
            (4, 3) => TriadClass::Major,      // Major third + Minor third
            (3, 4) => TriadClass::Minor,      // Minor third + Major third
            (4, 4) => TriadClass::Augmented,  // Major third + Major third
            (3, 3) => TriadClass::Diminished, // Minor third + Minor third
            _ => panic!("Invalid triad intervals: {} and {}", interval1, interval2),
        }
    }

    // Apply a Neo-Riemannian transformation
    pub fn apply_transformation(&mut self, transformation: Transformation) {
        match transformation {
            Transformation::Leading => self.apply_leading_transformation(),
            Transformation::Parallel => self.apply_parallel_transformation(),
            Transformation::Relative => self.apply_relative_transformation(),
        }
    }

    // Apply the leading transformation (L)
    pub fn apply_leading_transformation(&mut self) {
        match self.class {
            TriadClass::Major => {
                // Major to Minor transformation:
                // - Move root (x) to fifth position and subtract 1
                // - Result: (y, z, pymod(x-1))
                let [x, y, z] = self.alphabet;

                // δ_L((x, y, z)) = (y, z, pymod(x-1))
                let new_triad = [y, z, pymod(x as isize - 1, self.modulus as isize) as usize];

                self.alphabet = new_triad;
                self.class = TriadClass::Minor;
            }
            TriadClass::Minor => {
                // Minor to Major transformation:
                // - Add 1 to the third symbol and move it to front
                // - Result: (pymod(z+1), x, y)
                let [x, y, z] = self.alphabet;

                // δ_L(E-minor triad(4, 7, 11)) -> C-Major (11 + 1, 4, 7)
                let new_triad = [pymod(z as isize + 1, self.modulus as isize) as usize, x, y];

                self.alphabet = new_triad;
                self.class = TriadClass::Major;
            }
            TriadClass::Augmented => {
                // Transforming augmented triads is special since they're symmetric
                let [x, y, z] = self.alphabet;

                // Modify to create a major triad
                let new_triad = [x, y, pymod(z as isize - 1, self.modulus as isize) as usize];

                self.alphabet = new_triad;
                self.class = TriadClass::Major;
            }
            TriadClass::Diminished => {
                // Transforming diminished triads
                let [x, y, z] = self.alphabet;

                // Modify to create a minor triad
                let new_triad = [x, y, pymod(z as isize + 1, self.modulus as isize) as usize];

                self.alphabet = new_triad;
                self.class = TriadClass::Minor;
            }
        }

        // Verify the new triad classification
        let computed_type = self.classify_triad(&self.alphabet);
        if computed_type != self.class {
            panic!(
                "Leading transformation resulted in unexpected type: expected {:?}, got {:?}",
                self.class, computed_type
            );
        }
    }

    // Apply the parallel transformation (P)
    pub fn apply_parallel_transformation(&mut self) {
        match self.class {
            TriadClass::Major => {
                // Major to parallel minor: lower the middle note by a semitone
                let [x, y, z] = self.alphabet;

                // C-Major [0, 4, 7] -> C-Minor [0, 3, 7]
                let new_triad = [x, pymod(y as isize - 1, self.modulus as isize) as usize, z];

                self.alphabet = new_triad;
                self.class = TriadClass::Minor;
            }
            TriadClass::Minor => {
                // Minor to parallel major: raise the middle note by a semitone
                let [x, y, z] = self.alphabet;

                // C-Minor [0, 3, 7] -> C-Major [0, 4, 7]
                let new_triad = [x, pymod(y as isize + 1, self.modulus as isize) as usize, z];

                self.alphabet = new_triad;
                self.class = TriadClass::Major;
            }
            TriadClass::Augmented | TriadClass::Diminished => {
                // For augmented and diminished, we'll convert to major first
                let root = self.alphabet[0];

                if self.class == TriadClass::Augmented {
                    self.alphabet = [
                        root,
                        pymod(root as isize + 4, self.modulus as isize) as usize,
                        pymod(root as isize + 7, self.modulus as isize) as usize,
                    ];
                } else {
                    self.alphabet = [
                        root,
                        pymod(root as isize + 4, self.modulus as isize) as usize,
                        pymod(root as isize + 7, self.modulus as isize) as usize,
                    ];
                }
                self.class = TriadClass::Major;
            }
        }

        // Verify the new triad classification
        let computed_type = self.classify_triad(&self.alphabet);
        if computed_type != self.class {
            panic!(
                "Parallel transformation resulted in unexpected type: expected {:?}, got {:?}",
                self.class, computed_type
            );
        }
    }

    // Apply the relative transformation (R)
    pub fn apply_relative_transformation(&mut self) {
        match self.class {
            TriadClass::Major => {
                // Major to relative minor: lower the root by a minor third
                let [x, y, ..] = self.alphabet;

                // C-Major [0, 4, 7] -> A-Minor [9, 0, 4]
                let new_root = pymod(x as isize - 3, self.modulus as isize) as usize;
                let new_triad = [new_root, x, y];

                self.alphabet = new_triad;
                self.class = TriadClass::Minor;
            }
            TriadClass::Minor => {
                // Minor to relative major: raise the fifth by a minor third
                let [y, z, ..] = self.alphabet;

                // A-Minor [9, 0, 4] -> C-Major [0, 4, 7]
                let new_fifth = pymod(z as isize + 3, self.modulus as isize) as usize;
                let new_triad = [y, z, new_fifth];

                self.alphabet = new_triad;
                self.class = TriadClass::Major;
            }
            TriadClass::Augmented | TriadClass::Diminished => {
                // For augmented and diminished, we'll convert to major first
                let root = self.alphabet[0];

                if self.class == TriadClass::Augmented {
                    self.alphabet = [
                        root,
                        pymod(root as isize + 4, self.modulus as isize) as usize,
                        pymod(root as isize + 7, self.modulus as isize) as usize,
                    ];
                } else {
                    self.alphabet = [
                        root,
                        pymod(root as isize + 4, self.modulus as isize) as usize,
                        pymod(root as isize + 7, self.modulus as isize) as usize,
                    ];
                }
                self.class = TriadClass::Major;
                // Then apply R transformation
                self.apply_relative_transformation();
            }
        }

        // Verify the new triad classification
        let computed_type = self.classify_triad(&self.alphabet);
        if computed_type != self.class {
            panic!(
                "Relative transformation resulted in unexpected type: expected {:?}, got {:?}",
                self.class, computed_type
            );
        }
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
            // Apply only the Leading transformation (since we're just switching
            // between C-Major [0,4,7] and E-Minor [4,7,11])
            self.apply_leading_transformation();

            // Verify the symbol is now accessible
            if !self.is_valid_symbol(symbol) {
                // If the symbol is still not valid, it's outside our defined alphabet
                println!(
                    "Warning: Symbol {} not in any accessible triad context",
                    symbol
                );
            }
        }

        // Find the transition rule for the current state and symbol
        if let Some(&(new_state, new_symbol, direction)) =
            self.transition_rules.get(&(self.state, symbol))
        {
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
    pub fn run(&mut self, input: Vec<usize>) -> Vec<(usize, Vec<usize>, [usize; 3], TriadClass)> {
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