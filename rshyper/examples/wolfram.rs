use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    use tm::WolframUTM;
    // Create a simple program using our 4 symbols (0, 4, 7, 11)
    let program = vec![0, 4, 7, 11, 0, 4, 7, 0, 11, 4, 0, 7];

    // Initialize our UTM with a modulus of 12 for the musical system
    let mut utm = WolframUTM::new(program, 12);

    println!("Initial state: {}", utm.to_string());

    // Run for 100 iterations
    let history = utm.run(100);

    // Print the execution history
    for (i, (state, tape, triad, triad_type)) in history.iter().enumerate() {
        println!(
            "Step {}: State {}, Tape: {:?}, Triad: {:?} {:?}",
            i, state, tape, triad_type, triad
        );
    }

    println!("Final state: {}", utm.to_string());

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
    let mut demo_utm = WolframUTM::new(vec![0], 12);
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

    // Defines the classification of a triad
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum TriadType {
        Major,
        Minor,
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

        // Current triad type (Major or Minor)
        pub(crate) triad_type: TriadType,

        // Transition rules: (current_state, current_symbol) -> (new_state, new_symbol, direction)
        pub(crate) transition_rules: HashMap<(usize, usize), (usize, usize, Direction)>,

        // Modulus for symbol space
        pub(crate) modulus: usize,
    }

    impl WolframUTM {
        // Create a new UTM with a specific program
        pub fn new(program: Vec<usize>, modulus: usize) -> Self {
            // Define our initial C-Major triad [0, 4, 7]
            let initial_triad = [0, 4, 7];

            // Create transition rules - these are arbitrary for now
            let mut rules = HashMap::new();

            // State 0 rules
            rules.insert((0, 0), (0, 4, Direction::Right)); // If in state 0 and see 0, write 4 and move right
            rules.insert((0, 4), (1, 7, Direction::Right)); // If in state 0 and see 4, write 7 and move right, switch to state 1
            rules.insert((0, 7), (0, 0, Direction::Left)); // If in state 0 and see 7, write 0 and move left
            rules.insert((0, 11), (1, 4, Direction::Left)); // If in state 0 and see 11, write 4 and move left, switch to state 1

            // State 1 rules
            rules.insert((1, 0), (1, 7, Direction::Left)); // If in state 1 and see 0, write 7 and move left
            rules.insert((1, 4), (0, 11, Direction::Left)); // If in state 1 and see 4, write 11 and move left, switch to state 0
            rules.insert((1, 7), (1, 4, Direction::Right)); // If in state 1 and see 7, write 4 and move right
            rules.insert((1, 11), (0, 0, Direction::Right)); // If in state 1 and see 11, write 0 and move right, switch to state 0

            WolframUTM {
                state: 0,
                tape: program,
                position: 0,
                current_triad: initial_triad,
                triad_type: TriadType::Major, // C-Major is our starting point
                transition_rules: rules,
                modulus: modulus,
            }
        }

        // Calculate interval between two notes in a 12-tone system
        pub fn calculate_interval(&self, a: usize, b: usize) -> usize {
            pymod(b as isize - a as isize, self.modulus as isize) as usize
        }

        // Determine if a triad is major or minor based on the intervals
        pub fn classify_triad(&self, triad: &[usize; 3]) -> TriadType {
            let interval1 = self.calculate_interval(triad[0], triad[1]);
            let interval2 = self.calculate_interval(triad[1], triad[2]);

            // In a 12-tone system:
            // Major third = 4 semitones, Minor third = 3 semitones
            // Major triad: Major third (4) followed by Minor third (3)
            // Minor triad: Minor third (3) followed by Major third (4)
            if interval1 == 4 && interval2 == 3 {
                TriadType::Major
            } else if interval1 == 3 && interval2 == 4 {
                TriadType::Minor
            } else {
                panic!("Invalid triad intervals: {} and {}", interval1, interval2);
            }
        }

        // Apply the leading transformation to the current triad
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
            }

            // Verify the new triad classification matches what we expect
            let computed_type = self.classify_triad(&self.current_triad);
            if computed_type != self.triad_type {
                panic!(
                    "Triad transformation resulted in unexpected type: expected {:?}, got {:?}",
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

        // Update the step method in the WolframUTM implementation
        pub fn step(&mut self) -> bool {
            let symbol = self.get_current_symbol();

            // Check if the current symbol is in our alphabet (current triad)
            // If not, apply the leading transformation to switch to the other triad
            if !self.is_valid_symbol(symbol) {
                self.apply_leading_transformation();

                // If still not valid after transformation, symbol is outside of both triads
                if !self.is_valid_symbol(symbol) {
                    // We could handle this differently, but for now we'll proceed with the invalid symbol
                    panic!("Warning: Symbol {} is not in either triad alphabet", symbol);
                }
            }

            // Find the transition rule
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
        // Run the UTM for a specified number of iterations
        pub fn run(
            &mut self,
            iterations: usize,
        ) -> Vec<(usize, Vec<usize>, [usize; 3], TriadType)> {
            let mut history = Vec::new();

            for _ in 0..iterations {
                // Record the current state, tape, and triad information
                history.push((
                    self.state,
                    self.tape.clone(),
                    self.current_triad,
                    self.triad_type,
                ));

                // Step the UTM forward
                if !self.step() {
                    break;
                }
            }

            history
        }

        // Convert the machine state to string for visualization
        pub fn to_string(&self) -> String {
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
}

// For testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triad_classification() {
        let utm = WolframUTM::new(vec![0], 12);

        // C-Major [0, 4, 7]
        assert_eq!(utm.classify_triad(&vec![0, 4, 7]), TriadType::Major);

        // E-Minor [4, 7, 11]
        assert_eq!(utm.classify_triad(&vec![4, 7, 11]), TriadType::Minor);

        // G-Major [7, 11, 2]
        assert_eq!(utm.classify_triad(&vec![7, 11, 2]), TriadType::Major);
    }

    #[test]
    fn test_major_to_minor_transformation() {
        let mut utm = WolframUTM::new(vec![0], 12);

        // Initial C-Major [0, 4, 7]
        assert_eq!(utm.triad_type, TriadType::Major);
        assert_eq!(utm.current_triad, vec![0, 4, 7]);

        // Apply transformation
        utm.apply_leading_transformation();

        // Should be E-Minor [4, 7, 11]
        assert_eq!(utm.triad_type, TriadType::Minor);
        assert_eq!(utm.current_triad, vec![4, 7, 11]);
    }

    #[test]
    fn test_minor_to_major_transformation() {
        let mut utm = WolframUTM::new(vec![0], 12);

        // Transform to E-Minor first
        utm.apply_leading_transformation();
        assert_eq!(utm.current_triad, vec![4, 7, 11]);
        assert_eq!(utm.triad_type, TriadType::Minor);

        // Now apply transformation again
        utm.apply_leading_transformation();

        // Should go back to C-Major [0, 4, 7]
        assert_eq!(utm.triad_type, TriadType::Major);
        assert_eq!(utm.current_triad, vec![0, 4, 7]);
    }

    #[test]
    fn test_full_transformation_cycle() {
        let mut utm = WolframUTM::new(vec![0], 12);

        // Start with C-Major [0, 4, 7]
        let initial_triad = utm.current_triad.clone();

        // Apply leading transformation 6 times
        for _ in 0..6 {
            utm.apply_leading_transformation();
        }

        // After 6 transformations, we should be back at C-Major
        assert_eq!(utm.current_triad, initial_triad);
        assert_eq!(utm.triad_type, TriadType::Major);
    }
}
