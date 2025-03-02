/*
    Appellation: learn <module>
    Contrib: @FL03
*/
#![cfg(feature = "rand")]

use crate::models::WolframUTM;
use crate::{RuleSet, TriadClass};
use rand::Rng;
use rstm::{Direction, Head, State, Tail};
use std::collections::HashMap;

/// Rule represents a transition rule: what to do given a state and symbol
pub type Rule = (usize, usize, Direction);

/// A goal that the UTM should learn to achieve
pub enum Goal {
    /// Transform the tape to match this pattern
    TargetPattern(Vec<usize>),

    /// Move the head to a specific position
    HeadPosition(usize),

    /// Reach a specific state
    ReachState(usize),

    /// Transform a specific input to produce a specific output
    Transform(Vec<usize>, Vec<usize>),
}

/// RLearningUTM implements a UTM that learns through reinforcement
pub struct RLearningUTM {
    /// Underlying UTM implementation
    pub utm: WolframUTM,

    /// Q-values for state-symbol-action pairs
    q_values: HashMap<Head<usize, usize>, HashMap<Tail<usize, usize>, f64>>,

    /// Learning rate
    alpha: f64,

    /// Discount factor
    gamma: f64,

    /// Exploration rate
    epsilon: f64,

    /// Possible actions (rules) the UTM can take
    actions: Vec<Tail<usize, usize>>,

    /// Maximum steps per episode
    max_steps: usize,
}

impl RLearningUTM {
    /// Create a new reinforcement learning UTM
    pub fn new(modulus: usize) -> Self {
        // Define possible actions for a Wolfram [2,3] UTM
        let mut actions = Vec::new();

        // For each state (0, 1) and each symbol in our alphabet
        for state in 0..2 {
            for &symbol in &[0, 4, 7, 11] {
                // For each possible next state
                for next_state in 0..2 {
                    // For each possible write symbol
                    for &write_symbol in &[0, 4, 7, 11] {
                        // For each direction
                        for &direction in &[Direction::Left, Direction::Right] {
                            actions.push(Tail::new(direction, State(next_state), write_symbol));
                        }
                    }
                }
            }
        }

        RLearningUTM {
            utm: WolframUTM::new(HashMap::new(), modulus),
            q_values: HashMap::new(),
            alpha: 0.1,
            gamma: 0.9,
            epsilon: 0.3,
            actions,
            max_steps: 100,
        }
    }

    /// Set learning parameters
    pub fn with_params(mut self, alpha: f64, gamma: f64, epsilon: f64) -> Self {
        self.alpha = alpha;
        self.gamma = gamma;
        self.epsilon = epsilon;
        self
    }

    /// Initialize the tape with the given input
    pub fn set_input(&mut self, input: Vec<usize>) {
        self.utm.tape = input;
        self.utm.position = 0;
        self.utm.set_state(State(0));
    }

    /// Choose an action using epsilon-greedy policy
    fn choose_action(&self, State(state): State<usize>, symbol: usize) -> Tail<usize, usize> {
        let mut rng = rand::rng();

        // With probability epsilon, choose a random action
        if rng.random::<f64>() < self.epsilon {
            return self.actions[rng.random_range(0..self.actions.len())];
        }
        let head = Head::new(State(state), symbol);
        // Otherwise, choose the best action based on Q-values
        if let Some(q_map) = self.q_values.get(&head) {
            if !q_map.is_empty() {
                // Find action with max Q-value
                return q_map
                    .iter()
                    .max_by(|(_, q1), (_, q2)| q1.partial_cmp(&q2).unwrap())
                    .map(|(&action, _)| action)
                    .unwrap();
            }
        }

        // Default to a random action if no Q-values exist
        self.actions[rng.random_range(0..self.actions.len())]
    }

    /// Apply a rule to the UTM
    fn apply_rule(&mut self, rule: Tail<usize, usize>) {
        let Tail {
            state: next_state,
            symbol: write_symbol,
            direction,
        } = rule;

        // Get the current symbol
        let symbol = self.utm.get_current_symbol();

        // Apply transformation if needed
        if !self.utm.is_valid_symbol(symbol) {
            self.utm.apply_leading_transformation();
        }

        // Write the new symbol
        if self.utm.position >= self.utm.tape.len() {
            self.utm.tape.resize(self.utm.position + 1, 0);
        }
        self.utm.tape[self.utm.position] = write_symbol;

        // Update state
        self.utm.set_state(next_state);

        // Move head
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
                if self.utm.position >= self.utm.tape.len() {
                    self.utm.tape.push(0);
                }
            }
            Direction::Stay => {}
        }
    }

    /// Calculate reward based on a goal
    fn calculate_reward(&self, goal: &Goal) -> f64 {
        match goal {
            Goal::TargetPattern(target) => {
                // Reward based on how many positions match the target
                let mut matches = 0;
                let min_len = self.utm.tape.len().min(target.len());

                for i in 0..min_len {
                    if self.utm.tape[i] == target[i] {
                        matches += 1;
                    }
                }

                // Normalize reward between -1 and 1
                2.0 * (matches as f64 / target.len() as f64) - 1.0
            }

            Goal::HeadPosition(target_pos) => {
                // Reward based on distance to target position
                let distance = (self.utm.position as isize - *target_pos as isize).abs();
                1.0 - (distance as f64 / self.utm.tape.len() as f64).min(1.0)
            }

            Goal::ReachState(target_state) => {
                // Binary reward: 1 if in target state, -1 otherwise
                if self.utm.state == *target_state {
                    1.0
                } else {
                    -0.1
                }
            }

            Goal::Transform(input, output) => {
                // Check if input has been transformed to output
                if self.utm.tape == *output {
                    return 10.0;
                }

                // Partial reward based on matching positions
                let mut matches = 0;
                let min_len = self.utm.tape.len().min(output.len());

                for i in 0..min_len {
                    if self.utm.tape[i] == output[i] {
                        matches += 1;
                    }
                }

                // Normalize between -1 and 1
                (2.0 * matches as f64 / output.len() as f64) - 1.0
            }
        }
    }

    /// Train the UTM to achieve a goal
    pub fn train(&mut self, goal: Goal, episodes: usize) {
        let mut rng = rand::rng();

        // Record the best performance
        let mut best_reward = std::f64::NEG_INFINITY;
        let mut best_rules = HashMap::new();

        println!("Training UTM for {} episodes...", episodes);

        for episode in 0..episodes {
            // Reset UTM state
            match &goal {
                Goal::Transform(input, _) => self.set_input(input.clone()),
                _ => self.set_input(vec![0, 4, 7]),
            }

            let mut total_reward = 0.0;

            // Run an episode
            for _ in 0..self.max_steps {
                let state = self.utm.state;
                let symbol = self.utm.get_current_symbol();

                // Choose action
                let action = self.choose_action(state, symbol);

                // Apply the action
                self.apply_rule(action);

                // Calculate reward
                let reward = self.calculate_reward(&goal);
                total_reward += reward;

                // Calculate max future Q-value
                let next_state = self.utm.state;
                let next_symbol = self.utm.get_current_symbol();
                let next_head = Head::new(next_state, next_symbol);
                let max_next_q = self
                    .q_values
                    .get(&next_head)
                    .map(|q_map| {
                        q_map
                            .values()
                            .fold(0.0, |max_q: f64, &q: &f64| max_q.max(q))
                    })
                    .unwrap_or(0.0);

                // Update Q-value
                let q_key = Head::new(state, symbol);
                let q_entry = self.q_values.entry(q_key).or_insert_with(HashMap::new);
                let q_value = q_entry.entry(action).or_insert(0.0);

                // Q-learning update rule
                *q_value += self.alpha * (reward + self.gamma * max_next_q - *q_value);

                // Terminal condition check
                if matches!(&goal, Goal::Transform(_, target) if self.utm.tape == *target)
                    || matches!(&goal, Goal::HeadPosition(pos) if self.utm.position == *pos)
                    || matches!(&goal, Goal::ReachState(s) if self.utm.state == *s)
                {
                    break;
                }
            }

            // Track the best performance
            if total_reward > best_reward {
                best_reward = total_reward;
                best_rules = self.extract_rules();
            }

            // Decay exploration rate
            if episode % 100 == 99 {
                self.epsilon *= 0.9;
                println!(
                    "Episode {}: Reward = {:.2}, Epsilon = {:.3}",
                    episode + 1,
                    total_reward,
                    self.epsilon
                );
            }
        }

        // Use the best rules found
        println!("Training complete. Best reward: {:.2}", best_reward);
        self.utm.ruleset = best_rules;
    }

    /// Extract deterministic rules from Q-values
    pub fn extract_rules(&self) -> RuleSet<usize, usize> {
        let mut rules = HashMap::new();

        for (&head, q_map) in &self.q_values {
            if !q_map.is_empty() {
                // Find action with max Q-value
                let best_action = q_map
                    .iter()
                    .max_by(|(_, q1), (_, q2)| q1.partial_cmp(q2).unwrap())
                    .map(|(&action, _)| action)
                    .unwrap();

                rules.insert(head, best_action);
            }
        }

        rules
    }

    /// Run the UTM with the learned rules
    pub fn run(
        &mut self,
        input: Vec<usize>,
    ) -> Vec<(State<usize>, Vec<usize>, [usize; 3], TriadClass)> {
        self.utm.run(input)
    }

    /// Get the internal state of the UTM
    pub fn get_state(&self) -> (State<usize>, Vec<usize>, usize, [usize; 3], TriadClass) {
        (
            self.utm.state,
            self.utm.tape.clone(),
            self.utm.position,
            self.utm.alphabet,
            self.utm.class,
        )
    }

    /// Print the current rules
    pub fn print_rules(&self) {
        println!("Learned Rules:");
        for (&head, &tail) in &self.utm.ruleset {
            println!("{head} → {tail}");
        }
    }
}
