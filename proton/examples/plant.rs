/*
    Appellation: wolfram <example>
    Contrib: @FL03
*/
use proton::{LPR, Plant, Triad};
use std::collections::HashMap;

fn main() -> proton::Result<()> {
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

    let mut plant = Plant::new(Triad::major(0));
    plant.set_ruleset(ruleset.clone());

    println!("Initial configuration: {}", plant.utm().pretty_print());

    // Run for 100 iterations
    let history = plant.run(program.to_vec())?;

    // Print the execution history
    for (i, (triad, state, tape)) in history.iter().enumerate() {
        println!("Step {i}: Triad: {triad} State {state}, {tape:?}",);
    }

    println!("Final state: {}", plant.utm().pretty_print());

    // Analysis of the run
    println!("\nAnalysis:");
    println!("- Final tape length: {}", plant.tape().len());
    println!("- Final position: {}", plant.position());
    println!("- Final triad: {:?} {:?}", plant.class(), plant.alphabet());

    // Count occurrences of each symbol in the final tape
    let mut symbol_counts = HashMap::new();
    for &symbol in plant.tape() {
        *symbol_counts.entry(symbol).or_insert(0) += 1;
    }
    println!("- Symbol counts: {:?}", symbol_counts);

    // Detailed information about transformations
    println!("\nMusical Interpretation:");
    println!("- Starting with C-Major triad: [0, 4, 7]");

    // Demonstrate a single transformation
    let headspace = Triad::major(0);
    let plant = Plant::new(headspace);
    println!("- Initial plant: {plant:?}",);

    let next = plant.transform(LPR::Leading);
    println!("- After δ_L: {next:?}");

    let other = plant.transform(LPR::Parallel);
    assert_eq!(other, plant);
    println!(
        "- After δ_L again: {:?} {:?}",
        other.class(),
        other.alphabet()
    );
    Ok(())
}
