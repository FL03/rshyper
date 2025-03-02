/*
    Appellation: learn <example>
    Contrib: @FL03
*/
use proton::models::learn::{Goal, RLearningUTM};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize a learning UTM with more aggressive learning parameters
    let mut learner = RLearningUTM::new(12).with_params(0.2, 0.8, 0.4);

    println!("UTM Goal-Based Learning Demonstration");
    println!("===================================\n");

    // Define a transformation goal: convert [0,4,7] to [7,4,0]
    let input = vec![0, 4, 7];
    let target = vec![7, 4, 0];
    let goal = Goal::Transform(input.clone(), target.clone());

    println!("Goal: Transform {:?} into {:?}", input, target);

    // Train the UTM on this goal
    let start = Instant::now();
    learner.train(goal, 2000);
    let duration = start.elapsed();
    println!("Training completed in {:.2?}", duration);

    // Print the learned rules
    learner.print_rules();

    // Test the learned rules
    println!("\nTesting the learned rules...");
    learner.set_input(input.clone());

    // Record initial state
    println!("Initial state:");
    let (state, tape, position, alphabet, triad_class) = learner.get_state();
    println!("  State: {}", state);
    println!("  Tape: {:?}", tape);
    println!("  Head position: {}", position);
    println!("  Alphabet: {:?} ({:?})", alphabet, triad_class);

    // Run for a few steps to see what it does
    println!("\nExecution Steps:");
    let history = learner.run(input);

    for (i, (state, tape, alphabet, triad_class)) in history.iter().enumerate() {
        println!("Step {}:", i);
        println!("  State: {}", state);
        println!("  Tape: {:?}", tape);
        println!(
            "  Head position: {}",
            if i < history.len() - 1 {
                learner.utm.position()
            } else {
                learner.utm.position()
            }
        );
        println!("  Alphabet: {:?} ({:?})", alphabet, triad_class);
    }

    // Check if goal was achieved
    println!("\nFinal tape: {:?}", learner.utm.tape());
    if learner.utm.tape() == &target {
        println!("✓ Goal achieved! The UTM successfully learned to transform the input!");
    } else {
        println!("✗ Goal not fully achieved. Further training may be needed.");

        // Calculate similarity to target
        let min_len = learner.utm.tape().len().min(target.len());
        let mut matches = 0;
        for i in 0..min_len {
            if learner.utm[i] == target[i] {
                matches += 1;
            }
        }

        println!(
            "  Similarity to target: {:.0}%",
            100.0 * matches as f64 / target.len() as f64
        );
    }

    // Demonstrate other learning goals
    println!("\nAdditional Goal Learning Examples:");

    // Goal 1: Move head to position 2
    println!("\n1. Teaching UTM to move head to position 2");
    let mut head_learner = RLearningUTM::new(12).with_params(0.2, 0.8, 0.4);
    head_learner.train(Goal::HeadPosition(2), 1000);

    println!("Testing head positioning...");
    let test_input = vec![0, 4, 7, 11, 0, 4];
    let head_history = head_learner.run(test_input);
    println!("Final head position: {}", head_learner.utm.position());
    println!("Success: {}", head_learner.utm.position() == 2);

    // Goal 2: Reach state 1
    println!("\n2. Teaching UTM to reach state 1");
    let mut state_learner = RLearningUTM::new(12).with_params(0.2, 0.8, 0.4);
    state_learner.train(Goal::ReachState(1), 1000);

    println!("Testing state transition...");
    let state_history = state_learner.run(vec![0, 4, 7]);
    println!("Final state: {}", state_learner.utm.state());
    println!("Success: {}", state_learner.utm.state() == 1);

    Ok(())
}
