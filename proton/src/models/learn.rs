/*
    Appellation: learning_utm <module>
    Contrib: @FL03
*/
#![cfg(feature = "rand")]

use crate::types::{Direction, TriadClass};
use crate::models::WolframUTM;
use std::collections::HashMap;

/// TrainingData represents a sequence of execution steps used for training
pub type TrainingExample = (usize, usize, usize, Direction);

/// RuleProb represents a probability distribution over possible rules
pub type RuleProb = HashMap<(usize, usize, Direction), f64>;

/// LearningUTM extends WolframUTM with rule learning capabilities
pub struct LearningUTM {
    /// Base UTM implementation
    utm: WolframUTM,
    
    /// Learned transition probabilities: (state, symbol) -> HashMap<(new_state, new_symbol, direction), probability>
    learned_rules: HashMap<(usize, usize), RuleProb>,
    
    /// Learning rate for updating probabilities
    learning_rate: f64,
    
    /// Exploration rate (epsilon) for exploration vs exploitation
    exploration_rate: f64,
}

impl LearningUTM {
    /// Create a new learning UTM with modulus 12 (standard musical system)
    pub fn new(modulus: usize) -> Self {
        LearningUTM {
            utm: WolframUTM::new(HashMap::new(), modulus),
            learned_rules: HashMap::new(),
            learning_rate: 0.1,
            exploration_rate: 0.2,
        }
    }
    
    /// Set the learning parameters
    pub fn with_params(mut self, learning_rate: f64, exploration_rate: f64) -> Self {
        self.learning_rate = learning_rate;
        self.exploration_rate = exploration_rate;
        self
    }
    
    /// Train the UTM on a set of examples
    pub fn train(&mut self, examples: Vec<TrainingExample>, epochs: usize) {
        for _ in 0..epochs {
            for &(state, symbol, next_state, direction) in &examples {
                self.learn_rule(state, symbol, next_state, symbol, direction);
            }
            
            // Gradually reduce exploration rate
            self.exploration_rate *= 0.95;
        }
        
        // Generate deterministic ruleset from learned probabilities
        let rules = self.extract_deterministic_rules();
        self.utm.transition_rules = rules.clone();
        
        println!("Training completed. Learned {} rules.", rules.len());
    }
    
    /// Learn a single rule from an observation
    pub fn learn_rule(&mut self, state: usize, symbol: usize, next_state: usize, next_symbol: usize, direction: Direction) {
        let rule_key = (state, symbol);
        let action = (next_state, next_symbol, direction);
        
        // Get or create the rule probability distribution
        let rule_probs = self.learned_rules.entry(rule_key).or_insert_with(HashMap::new);
        
        // Update the probability for this action
        let prob = rule_probs.entry(action).or_insert(0.0);
        *prob += self.learning_rate * (1.0 - *prob);
        
        // Decrease probabilities for other actions
        for (other_action, other_prob) in rule_probs.iter_mut() {
            if *other_action != action {
                *other_prob *= 1.0 - self.learning_rate;
            }
        }
    }
    
    /// Extract deterministic rules from learned probabilities
    pub fn extract_deterministic_rules(&self) -> HashMap<(usize, usize), (usize, usize, Direction)> {
        let mut deterministic_rules = HashMap::new();
        
        for (&(state, symbol), rule_probs) in &self.learned_rules {
            if !rule_probs.is_empty() {
                // Find the action with highest probability
                let best_action = rule_probs
                    .iter()
                    .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                    .map(|(&action, _)| action)
                    .unwrap();
                
                deterministic_rules.insert((state, symbol), best_action);
            }
        }
        
        deterministic_rules
    }
    
    /// Run the UTM with the given input using learned rules
    pub fn run(&mut self, input: Vec<usize>) -> Vec<(usize, Vec<usize>, [usize; 3], TriadClass)> {
        self.utm.run(input)
    }
    
    /// Step through execution using learned rules with exploration
    pub fn exploratory_step(&mut self) -> bool {
        let state = self.utm.state();
        let symbol = self.utm.get_current_symbol();
        
        // Check if we need to apply transformation
        if !self.utm.is_valid_symbol(symbol) {
            self.utm.apply_leading_transformation();
        }
        
        // Get the rule probabilities for this state-symbol pair
        if let Some(rule_probs) = self.learned_rules.get(&(state, symbol)) {
            // Decide whether to explore or exploit
            let explore = rand::random::<f64>() < self.exploration_rate;
            
            let action = if explore {
                // Explore: choose a random action
                let actions: Vec<_> = rule_probs.keys().cloned().collect();
                if actions.is_empty() {
                    return false; // No actions available
                }
                actions[rand::random::<u128>() as usize % actions.len()]
            } else {
                // Exploit: choose the best action
                rule_probs
                    .iter()
                    .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                    .map(|(&action, _)| action)
                    .unwrap_or_else(|| {
                        // Default action if none found
                        (0, 0, Direction::Right)
                    })
            };
            
            // Apply the selected action
            let (new_state, new_symbol, direction) = action;
            
            // Update the tape
            if self.utm.position() >= self.utm.tape().len() {
                self.utm.tape.resize(self.utm.position() + 1, 0);
            }
            self.utm.tape[self.utm.position] = new_symbol;
            
            // Update state
            self.utm.state = new_state;
            
            // Move the head
            match direction {
                Direction::Left => {
                    if self.utm.position > 0 {
                        self.utm.position -= 1;
                    } else {
                        self.utm.tape.insert(0, 0);
                    }
                }
                Direction::Right => {
                    self.utm.position += 1;
                    if self.utm.position() >= self.utm.tape().len() {
                        self.utm.tape.push(0);
                    }
                }
                Direction::Stay => {}
            }
            
            true
        } else {
            // No rule found for this state-symbol pair
            false
        }
    }
    
    /// Get the current rule probabilities
    pub fn rule_probabilities(&self) -> &HashMap<(usize, usize), RuleProb> {
        &self.learned_rules
    }
    
    /// Get a reference to the underlying UTM
    pub fn utm(&self) -> &WolframUTM {
        &self.utm
    }
    
    /// Get a mutable reference to the underlying UTM
    pub fn utm_mut(&mut self) -> &mut WolframUTM {
        &mut self.utm
    }
}