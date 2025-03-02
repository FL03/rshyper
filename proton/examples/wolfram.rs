/*
    Appellation: wolfram <example>
    Contrib: @FL03
*/
use proton::prelude::*;
use std::collections::HashMap;

fn ruleset() -> RuleSet {
    use Direction::*;
    // Create transition rules for a Wolfram [2, 3] UTM (2 states, 3 symbols)
    let mut rules = HashMap::new();

    // State 0 rules - we only need rules for the 3 symbols in our current alphabet
    rules.insert((0, 0), (0, 4, Right)); // If in state 0 and see 0, write 4 and move right
    rules.insert((0, 4), (1, 7, Right)); // If in state 0 and see 4, write 7 and move right, switch to state 1
    rules.insert((0, 7), (0, 0, Left)); // If in state 0 and see 7, write 0 and move left

    // State 1 rules
    rules.insert((1, 0), (1, 7, Left)); // If in state 1 and see 0, write 7 and move left
    rules.insert((1, 4), (0, 11, Left)); // If in state 1 and see 4, write 11 and move left, switch to state 0
    rules.insert((1, 7), (1, 4, Right)); // If in state 1 and see 7, write 4 and move right

    // When we see 11 (which requires a transformation), we'll have a specific rule for it
    rules.insert((0, 11), (1, 7, Left)); // If in state 0 and see 11, write 4 and move left, switch to state 1
    rules.insert((1, 11), (0, 0, Right)); // If in state 1 and see 11, write 0 and move right, switch to state 0

    rules
}
fn main() -> Result<()> {
    // Create a simple program using our 4 symbols (0, 4, 7, 11)
    let program = [0, 4, 7, 11, 0, 4, 7, 0, 11, 4, 0, 7];

    // Initialize our UTM with a modulus of 12 for the musical system
    let ruleset = ruleset();
    let mut utm = WolframUTM::new(ruleset.clone(), 12);

    println!("Initial state: {}", utm.printed());

    // Run for 100 iterations
    let history = utm.run(program.to_vec());

    // Print the execution history
    for (i, (state, tape, triad, class)) in history.iter().enumerate() {
        println!(
            "Step {i}: State {state}, Tape: {tape:?}, Triad: {class:?}({triad:?})",
        );
    }

    println!("Final state: {}", utm.printed());

    // Analysis of the run
    println!("\nAnalysis:");
    println!("- Final tape length: {}", utm.tape().len());
    println!("- Final position: {}", utm.position());
    println!(
        "- Final triad: {:?} {:?}",
        utm.class(), utm.alphabet()
    );

    // Count occurrences of each symbol in the final tape
    let mut symbol_counts = HashMap::new();
    for &symbol in utm.tape() {
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
        demo_utm.class(), demo_utm.alphabet()
    );

    demo_utm.apply_leading_transformation();
    println!(
        "- After δ_L: {:?} {:?}",
        demo_utm.class(), demo_utm.alphabet()
    );

    demo_utm.apply_leading_transformation();
    println!(
        "- After δ_L again: {:?} {:?}",
        demo_utm.class(), demo_utm.alphabet()
    );
    Ok(())
}
