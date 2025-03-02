use std::collections::HashMap;


fn ruleset() -> tm::RuleSet {
    use tm::Direction::*;
    // Create transition rules for a Wolfram [2, 3] UTM (2 states, 3 symbols)
    let mut rules = HashMap::new();

    // State 0 rules - we only need rules for the 3 symbols in our current alphabet
    rules.insert((0, 0), (0, 4, Right));  // If in state 0 and see 0, write 4 and move right
    rules.insert((0, 4), (1, 7, Right));  // If in state 0 and see 4, write 7 and move right, switch to state 1
    rules.insert((0, 7), (0, 0, Left));   // If in state 0 and see 7, write 0 and move left

    // State 1 rules
    rules.insert((1, 0), (1, 7, Left));   // If in state 1 and see 0, write 7 and move left
    rules.insert((1, 4), (0, 11, Left));  // If in state 1 and see 4, write 11 and move left, switch to state 0
    rules.insert((1, 7), (1, 4, Right));  // If in state 1 and see 7, write 4 and move right
    
    // When we see 11 (which requires a transformation), we'll have a specific rule for it
    rules.insert((0, 11), (1, 7, Left));  // If in state 0 and see 11, write 4 and move left, switch to state 1
    rules.insert((1, 11), (0, 0, Right)); // If in state 1 and see 11, write 0 and move right, switch to state 0
    
    rules
}
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    use tm::WolframUTM;
    // Create a simple program using our 4 symbols (0, 4, 7, 11)
    let program = [0, 4, 7, 11, 0, 4, 7, 0, 11, 4, 0, 7];

    // Initialize our UTM with a modulus of 12 for the musical system
    let ruleset = ruleset();
    let mut utm = WolframUTM::new(ruleset.clone(), 12);

    println!("Initial state: {}", utm.printed());

    // Run for 100 iterations
    let history = utm.run(program.to_vec());

    // Print the execution history
    for (i, (state, tape, triad, triad_type)) in history.iter().enumerate() {
        println!(
            "Step {}: State {}, Tape: {:?}, Triad: {:?} {:?}",
            i, state, tape, triad_type, triad
        );
    }

    println!("Final state: {}", utm.printed());

    // Analysis of the run
    println!("\nAnalysis:");
    println!("- Final tape length: {}", utm.tape.len());
    println!("- Final position: {}", utm.position);
    println!(
        "- Final triad: {:?} {:?}",
        utm.triad_type, utm.current_triad
    );

    // Count occurrences of each symbol in the final tape
    let mut symbol_counts = HashMap::new();
    for &symbol in &utm.tape {
        *symbol_counts.entry(symbol).or_insert(0) += 1;
    }
    println!("- Symbol counts: {:?}", symbol_counts);

    // Detailed information about transformations
    println!("\nMusical Interpretation:");
    println!("- Starting with C-Major triad: [0, 4, 7]");

    // Demonstrate a single transformation
    let mut demo_utm = WolframUTM::new(ruleset, 12);
    println!(
        "- Initial triad: {:?} {:?}",
        demo_utm.triad_type, demo_utm.current_triad
    );

    demo_utm.apply_leading_transformation();
    println!(
        "- After δ_L: {:?} {:?}",
        demo_utm.triad_type, demo_utm.current_triad
    );

    demo_utm.apply_leading_transformation();
    println!(
        "- After δ_L again: {:?} {:?}",
        demo_utm.triad_type, demo_utm.current_triad
    );
    Ok(())
}

// The pymod function you provided
pub fn pymod<T>(lhs: T, rhs: T) -> T
where
    T: Copy + num::Num + PartialOrd,
{
    let r = lhs % rhs;
    if (r < T::zero() && rhs > T::zero()) || (r > T::zero() && rhs < T::zero()) {
        r + rhs
    } else {
        r
    }
}

#[allow(dead_code)]
mod tm {
    use super::pymod;
    use std::collections::HashMap;

    pub type RuleSet<Q = usize, S = usize> = HashMap<(Q, S), (Q, S, Direction)>;

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
    )]
    #[cfg_attr(
        feature = "serde",
        derive(serde_derive::Deserialize, serde_derive::Serialize)
    )]
    pub enum Direction {
        Left = -1,
        Right = 1,
        #[default]
        Stay = 0,
    }

    // Neo-Riemannian transformation types
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Transformation {
        Leading,  // L: Maps major to minor and vice versa
        Parallel, // P: Maps major to parallel minor and vice versa
        Relative, // R: Maps major to relative minor and vice versa
    }

    // Expanded triad types in Neo-Riemannian theory
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum TriadType {
        Major,
        Minor,
        Augmented,
        Diminished,
    }

    // Define the structure for our Wolfram UTM with two states and three symbols
    #[derive(Debug, Clone)]
    pub struct WolframUTM {
        // Current state (0 or 1)
        pub(crate) state: usize,

        // Tape containing the program symbols
        pub(crate) tape: Vec<usize>,

        // Current position on the tape
        pub(crate) position: usize,

        // Current triad (our active alphabet) - using fixed-size array
        pub(crate) current_triad: [usize; 3],

        // Current triad type (Major, Minor, Augmented, or Diminished)
        pub(crate) triad_type: TriadType,

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
                current_triad: initial_triad,
                triad_type: TriadType::Major, // C-Major is our starting point
                transition_rules: ruleset,
                modulus,
            }
        }

        // Calculate interval between two notes in a 12-tone system
        pub fn calculate_interval(&self, a: usize, b: usize) -> usize {
            pymod(b as isize - a as isize, self.modulus as isize) as usize
        }

        // Determine triad type based on the intervals
        pub fn classify_triad(&self, triad: &[usize; 3]) -> TriadType {
            let interval1 = self.calculate_interval(triad[0], triad[1]);
            let interval2 = self.calculate_interval(triad[1], triad[2]);

            match (interval1, interval2) {
                (4, 3) => TriadType::Major,      // Major third + Minor third
                (3, 4) => TriadType::Minor,      // Minor third + Major third
                (4, 4) => TriadType::Augmented,  // Major third + Major third
                (3, 3) => TriadType::Diminished, // Minor third + Minor third
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
            match self.triad_type {
                TriadType::Major => {
                    // Major to Minor transformation:
                    // - Move root (x) to fifth position and subtract 1
                    // - Result: (y, z, pymod(x-1))
                    let x = self.current_triad[0];
                    let y = self.current_triad[1];
                    let z = self.current_triad[2];

                    // δ_L((x, y, z)) = (y, z, pymod(x-1))
                    let new_triad = [y, z, pymod(x as isize - 1, self.modulus as isize) as usize];

                    self.current_triad = new_triad;
                    self.triad_type = TriadType::Minor;
                }
                TriadType::Minor => {
                    // Minor to Major transformation:
                    // - Add 1 to the third symbol and move it to front
                    // - Result: (pymod(z+1), x, y)
                    let x = self.current_triad[0];
                    let y = self.current_triad[1];
                    let z = self.current_triad[2];

                    // δ_L(E-minor triad(4, 7, 11)) -> C-Major (11 + 1, 4, 7)
                    let new_triad = [pymod(z as isize + 1, self.modulus as isize) as usize, x, y];

                    self.current_triad = new_triad;
                    self.triad_type = TriadType::Major;
                }
                TriadType::Augmented => {
                    // Transforming augmented triads is special since they're symmetric
                    let x = self.current_triad[0];
                    let y = self.current_triad[1];
                    let z = self.current_triad[2];

                    // Modify to create a major triad
                    let new_triad = [x, y, pymod(z as isize - 1, self.modulus as isize) as usize];

                    self.current_triad = new_triad;
                    self.triad_type = TriadType::Major;
                }
                TriadType::Diminished => {
                    // Transforming diminished triads
                    let x = self.current_triad[0];
                    let y = self.current_triad[1];
                    let z = self.current_triad[2];

                    // Modify to create a minor triad
                    let new_triad = [x, y, pymod(z as isize + 1, self.modulus as isize) as usize];

                    self.current_triad = new_triad;
                    self.triad_type = TriadType::Minor;
                }
            }

            // Verify the new triad classification
            let computed_type = self.classify_triad(&self.current_triad);
            if computed_type != self.triad_type {
                panic!(
                    "Leading transformation resulted in unexpected type: expected {:?}, got {:?}",
                    self.triad_type, computed_type
                );
            }
        }

        // Apply the parallel transformation (P)
        pub fn apply_parallel_transformation(&mut self) {
            match self.triad_type {
                TriadType::Major => {
                    // Major to parallel minor: lower the middle note by a semitone
                    let x = self.current_triad[0];
                    let y = self.current_triad[1];
                    let z = self.current_triad[2];

                    // C-Major [0, 4, 7] -> C-Minor [0, 3, 7]
                    let new_triad = [x, pymod(y as isize - 1, self.modulus as isize) as usize, z];

                    self.current_triad = new_triad;
                    self.triad_type = TriadType::Minor;
                }
                TriadType::Minor => {
                    // Minor to parallel major: raise the middle note by a semitone
                    let x = self.current_triad[0];
                    let y = self.current_triad[1];
                    let z = self.current_triad[2];

                    // C-Minor [0, 3, 7] -> C-Major [0, 4, 7]
                    let new_triad = [x, pymod(y as isize + 1, self.modulus as isize) as usize, z];

                    self.current_triad = new_triad;
                    self.triad_type = TriadType::Major;
                }
                TriadType::Augmented | TriadType::Diminished => {
                    // For augmented and diminished, we'll convert to major first
                    let root = self.current_triad[0];

                    if self.triad_type == TriadType::Augmented {
                        self.current_triad = [
                            root,
                            pymod(root as isize + 4, self.modulus as isize) as usize,
                            pymod(root as isize + 7, self.modulus as isize) as usize,
                        ];
                    } else {
                        self.current_triad = [
                            root,
                            pymod(root as isize + 4, self.modulus as isize) as usize,
                            pymod(root as isize + 7, self.modulus as isize) as usize,
                        ];
                    }
                    self.triad_type = TriadType::Major;
                }
            }

            // Verify the new triad classification
            let computed_type = self.classify_triad(&self.current_triad);
            if computed_type != self.triad_type {
                panic!(
                    "Parallel transformation resulted in unexpected type: expected {:?}, got {:?}",
                    self.triad_type, computed_type
                );
            }
        }

        // Apply the relative transformation (R)
        pub fn apply_relative_transformation(&mut self) {
            match self.triad_type {
                TriadType::Major => {
                    // Major to relative minor: lower the root by a minor third
                    let x = self.current_triad[0];
                    let y = self.current_triad[1];
                    let z = self.current_triad[2];

                    // C-Major [0, 4, 7] -> A-Minor [9, 0, 4]
                    let new_root = pymod(x as isize - 3, self.modulus as isize) as usize;
                    let new_triad = [new_root, x, y];

                    self.current_triad = new_triad;
                    self.triad_type = TriadType::Minor;
                }
                TriadType::Minor => {
                    // Minor to relative major: raise the fifth by a minor third
                    let x = self.current_triad[0];
                    let y = self.current_triad[1];
                    let z = self.current_triad[2];

                    // A-Minor [9, 0, 4] -> C-Major [0, 4, 7]
                    let new_fifth = pymod(z as isize + 3, self.modulus as isize) as usize;
                    let new_triad = [y, z, new_fifth];

                    self.current_triad = new_triad;
                    self.triad_type = TriadType::Major;
                }
                TriadType::Augmented | TriadType::Diminished => {
                    // For augmented and diminished, we'll convert to major first
                    let root = self.current_triad[0];

                    if self.triad_type == TriadType::Augmented {
                        self.current_triad = [
                            root,
                            pymod(root as isize + 4, self.modulus as isize) as usize,
                            pymod(root as isize + 7, self.modulus as isize) as usize,
                        ];
                    } else {
                        self.current_triad = [
                            root,
                            pymod(root as isize + 4, self.modulus as isize) as usize,
                            pymod(root as isize + 7, self.modulus as isize) as usize,
                        ];
                    }
                    self.triad_type = TriadType::Major;
                    // Then apply R transformation
                    self.apply_relative_transformation();
                }
            }

            // Verify the new triad classification
            let computed_type = self.classify_triad(&self.current_triad);
            if computed_type != self.triad_type {
                panic!(
                    "Relative transformation resulted in unexpected type: expected {:?}, got {:?}",
                    self.triad_type, computed_type
                );
            }
        }

        // Check if a symbol is in the current triad
        pub fn is_valid_symbol(&self, symbol: usize) -> bool {
            self.current_triad.contains(&symbol)
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

            // Check if the current symbol is in our alphabet (current triad)
            // If not, find the appropriate transformation to access it
            if !self.is_valid_symbol(symbol) {
                // Try Leading transformation first
                self.apply_transformation(Transformation::Leading);

                // If still not valid, try Parallel transformation
                if !self.is_valid_symbol(symbol) {
                    self.apply_transformation(Transformation::Parallel);

                    // Finally try Relative transformation
                    if !self.is_valid_symbol(symbol) {
                        self.apply_transformation(Transformation::Relative);

                        // If still not valid, we can't reach this symbol with simple transformations
                        if !self.is_valid_symbol(symbol) {
                            // For demonstration purposes, we'll just print a message and continue
                            // In a real system, we might need a more complex transformation strategy
                            println!(
                                "Warning: Symbol {} requires complex transformation path",
                                symbol
                            );
                        }
                    }
                }
            }

            // Find the transition rule (rest of the method remains the same)
            if let Some(&(new_state, new_symbol, direction)) =
                self.transition_rules.get(&(self.state, symbol))
            {
                // Update the tape
                if self.position >= self.tape.len() {
                    self.tape.resize(self.position + 1, 0);
                }
                self.tape[self.position] = new_symbol;

                // Update the state
                self.state = new_state;

                // Move the head
                match direction {
                    Direction::Left => {
                        if self.position > 0 {
                            self.position -= 1;
                        } else {
                            // If at the left edge, add a new cell
                            self.tape.insert(0, 0);
                        }
                    }
                    Direction::Right => {
                        self.position += 1;
                        if self.position >= self.tape.len() {
                            self.tape.push(0);
                        }
                    }
                    Direction::Stay => {
                        self.tape[self.position] = new_symbol;
                    }
                }

                true
            } else {
                // No transition rule found
                false
            }
        }

        // Run the UTM with the given input program
        // Run the UTM with the given input program until it halts
    pub fn run(&mut self, input: Vec<usize>) -> Vec<(usize, Vec<usize>, [usize; 3], TriadType)> {
        // Initialize the tape with the input
        self.tape = input;
        self.position = 0;
        
        let mut history = Vec::new();
        
        // Record initial state
        history.push((
            self.state,
            self.tape.clone(),
            self.current_triad,
            self.triad_type,
        ));
        
        // Run until the machine halts (i.e., step() returns false)
        while self.step() {
            // Record each state after a step
            history.push((
                self.state,
                self.tape.clone(),
                self.current_triad,
                self.triad_type,
            ));
            
            // Optional: Add a safety limit to prevent infinite loops in development
            if history.len() > 10000 {
                println!("Warning: Machine reached 10,000 steps without halting. Stopping execution.");
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
                self.state, self.triad_type, self.current_triad
            ));

            result
        }
    }

    impl core::fmt::Display for WolframUTM {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            write!(
                f,
                "Wolfram UTM: State {}, Tape: {:?}, Position: {}, Triad: {:?} {:?}",
                self.state, self.tape, self.position, self.triad_type, self.current_triad
            )
        }
    }
}

// For testing
// For testing
#[cfg(test)]
mod tests {
    use super::tm::{Transformation, TriadType, WolframUTM};
    use super::*;

    #[test]
    fn test_triad_classification() {
        let utm = tm::WolframUTM::new(vec![0], 12);

        // C-Major [0, 4, 7]
        assert_eq!(utm.classify_triad(&[0, 4, 7]), tm::TriadType::Major);

        // E-Minor [4, 7, 11]
        assert_eq!(utm.classify_triad(&[4, 7, 11]), tm::TriadType::Minor);

        // G-Major [7, 11, 2]
        assert_eq!(utm.classify_triad(&[7, 11, 2]), tm::TriadType::Major);

        // Augmented [0, 4, 8]
        assert_eq!(utm.classify_triad(&[0, 4, 8]), tm::TriadType::Augmented);

        // Diminished [0, 3, 6]
        assert_eq!(utm.classify_triad(&[0, 3, 6]), tm::TriadType::Diminished);
    }

    #[test]
    fn test_leading_transformation() {
        let mut utm = tm::WolframUTM::new(vec![0], 12);

        // Initial C-Major [0, 4, 7]
        assert_eq!(utm.triad_type, tm::TriadType::Major);
        assert_eq!(utm.current_triad, [0, 4, 7]);

        // Apply Leading transformation
        utm.apply_transformation(tm::Transformation::Leading);

        // Should be E-Minor [4, 7, 11]
        assert_eq!(utm.triad_type, tm::TriadType::Minor);
        assert_eq!(utm.current_triad, [4, 7, 11]);

        // Apply it again to return to Major
        utm.apply_transformation(tm::Transformation::Leading);
        assert_eq!(utm.triad_type, tm::TriadType::Major);
        assert_eq!(utm.current_triad, [0, 4, 7]);
    }

    #[test]
    fn test_parallel_transformation() {
        let mut utm = tm::WolframUTM::new(vec![0], 12);

        // Initial C-Major [0, 4, 7]
        assert_eq!(utm.current_triad, [0, 4, 7]);

        // Apply Parallel transformation
        utm.apply_transformation(tm::Transformation::Parallel);

        // Should be C-Minor [0, 3, 7]
        assert_eq!(utm.triad_type, tm::TriadType::Minor);
        assert_eq!(utm.current_triad, [0, 3, 7]);

        // Apply it again to return to Major
        utm.apply_transformation(tm::Transformation::Parallel);
        assert_eq!(utm.triad_type, tm::TriadType::Major);
        assert_eq!(utm.current_triad, [0, 4, 7]);
    }

    #[test]
    fn test_relative_transformation() {
        let mut utm = tm::WolframUTM::new(vec![0], 12);

        // Initial C-Major [0, 4, 7]
        assert_eq!(utm.current_triad, [0, 4, 7]);

        // Apply Relative transformation
        utm.apply_transformation(tm::Transformation::Relative);

        // Should be A-Minor [9, 0, 4]
        assert_eq!(utm.triad_type, tm::TriadType::Minor);
        assert_eq!(utm.current_triad, [9, 0, 4]);

        // Apply it again to return to Major (but not the same major)
        utm.apply_transformation(tm::Transformation::Relative);
        assert_eq!(utm.triad_type, tm::TriadType::Major);
    }

    #[test]
    fn test_composite_transformations() {
        let mut utm = tm::WolframUTM::new(vec![0], 12);

        // Apply LPR sequence
        utm.apply_transformation(tm::Transformation::Leading);
        utm.apply_transformation(tm::Transformation::Parallel);
        utm.apply_transformation(tm::Transformation::Relative);

        // Verify the result is correct
        assert_eq!(utm.triad_type, tm::TriadType::Major);

        // Reset and try RPL
        let mut utm2 = tm::WolframUTM::new(vec![0], 12);
        utm2.apply_transformation(tm::Transformation::Relative);
        utm2.apply_transformation(tm::Transformation::Parallel);
        utm2.apply_transformation(tm::Transformation::Leading);

        // Verify RPL result
        assert_eq!(utm2.triad_type, tm::TriadType::Major);
    }
}
