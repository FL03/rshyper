use proton::prelude::{Direction, LearningUTM};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut lutm = LearningUTM::new(12).with_params(0.2, 0.3);
    
    println!("Learning UTM Demonstration");
    println!("=========================\n");
    
    // Define some training examples for Wolfram [2,3]
    // Format: (state, symbol, next_state, direction)
    let training_examples = vec![
        // State 0 rules for C-Major triad [0,4,7]
        (0, 0, 0, Direction::Right),  // If in state 0 and see 0, stay in state 0 and move right
        (0, 4, 1, Direction::Right),  // If in state 0 and see 4, go to state 1 and move right
        (0, 7, 0, Direction::Left),   // If in state 0 and see 7, stay in state 0 and move left
        
        // State 1 rules for E-Minor triad [4,7,11]
        (1, 4, 1, Direction::Left),   // If in state 1 and see 4, stay in state 1 and move left
        (1, 7, 0, Direction::Right),  // If in state 1 and see 7, go to state 0 and move right
        (1, 11, 1, Direction::Right), // If in state 1 and see 11, stay in state 1 and move right
        
        // Rules for symbols requiring transformations
        (0, 11, 1, Direction::Left),  // Need L transformation to handle 11 in state 0
        (1, 0, 0, Direction::Right),  // Need L transformation to handle 0 in state 1
    ];
    
    println!("Training on {} examples...", training_examples.len());
    
    // Train the UTM for 50 epochs on these examples
    lutm.train(training_examples, 50);
    
    // Display the learned rules
    println!("\nLearned Rule Probabilities:");
    for ((state, symbol), probs) in lutm.rule_probabilities() {
        println!("State {}, Symbol {}:", state, symbol);
        let best_rule = probs.iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap();
        let (next_state, next_symbol, dir) = best_rule.0;
        println!("  → State {}, Write {}, Move {:?} (p={:.2})", 
                next_state, next_symbol, dir, best_rule.1);
    }
    
    // Create test input
    let test_input = vec![0, 4, 7, 11, 0, 4, 7, 0];
    
    println!("\nRunning UTM with learned rules on test input: {:?}", test_input);
    let history = lutm.run(test_input);
    
    println!("\nExecution Steps:");
    for (i, (state, tape, alphabet, class)) in history.iter().enumerate() {
        println!("Step {}: State {}, Head at position {}", i, state, lutm.utm().position());
        println!("  Tape: {:?}", tape);
        println!("  Alphabet: {:?} ({:?})", alphabet, class);
        println!("  ------------------------------");
    }
    
    println!("\nFinal State: {}", lutm.utm().printed());
    
    Ok(())
}