/*
    Appellation: wolfram <example>
    Contrib: @FL03
*/
use proton::prelude::*;
use std::collections::HashMap;

fn main() -> Result<()> {
    let ruleset = proton::ruleset! {
        (0, 0) => (Right, 0, 4),
        (0, 4) => (Right, 1, 7),
        (0, 7) => (Left, 0, 0),
        (1, 0) => (Left, 1, 7),
        (1, 4) => (Left, 0, 11),
        (1, 7) => (Right, 1, 4),
        (0, 11) => (Left, 1, 7),
        (1, 11) => (Right, 0, 0)
    };

    // Create a simple program using our 4 symbols (0, 4, 7, 11)
    let program = [0, 4, 7, 11, 0, 4, 7, 0, 11, 4, 0, 7];

    // Initialize our UTM with a modulus of 12 for the musical system
    let mut utm = WolframUTM::new(ruleset.clone(), 12);

    println!("Initial state: {}", utm.printed());

    // Run for 100 iterations
    let history = utm.run(program.to_vec());

    // Print the execution history
    for (i, (state, tape, triad, class)) in history.iter().enumerate() {
        println!("Step {i}: State {state}, Tape: {tape:?}, Triad: {class:?}({triad:?})",);
    }

    println!("Final state: {}", utm.printed());

    // Analysis of the run
    println!("\nAnalysis:");
    println!("- Final tape length: {}", utm.tape().len());
    println!("- Final position: {}", utm.position());
    println!("- Final triad: {:?} {:?}", utm.class(), utm.alphabet());

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
        demo_utm.class(),
        demo_utm.alphabet()
    );

    demo_utm.apply_leading_transformation();
    println!(
        "- After δ_L: {:?} {:?}",
        demo_utm.class(),
        demo_utm.alphabet()
    );

    demo_utm.apply_leading_transformation();
    println!(
        "- After δ_L again: {:?} {:?}",
        demo_utm.class(),
        demo_utm.alphabet()
    );
    Ok(())
}
