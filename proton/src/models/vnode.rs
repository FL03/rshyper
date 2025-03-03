/*
    Appellation: vnode <module>
    Contrib: @FL03
*/
use crate::mem::TopologicalMemory;
use crate::nrt::{LPR, Tonnetz, TriadClass};
use crate::plant::Plant;
use rshyper::EdgeId;
use rstm::{Direction, Head, State, Tail};
use std::collections::HashMap;

/// A virtualized workspace responsible for managing its associated plant.
#[derive(Clone, Debug)]
pub struct VNode {
    /// The underlying UTM
    pub plant: Plant,

    /// Current position in the Tonnetz
    pub position: EdgeId,

    /// Topological memory
    pub memory: TopologicalMemory,

    /// Learned rules based on topological features
    pub learned_rules: HashMap<Vec<usize>, Tail<usize, usize>>,

    /// History of visited positions
    pub path_history: Vec<EdgeId>,

    /// Performance metrics for different rules
    rule_performance: HashMap<Vec<usize>, f64>,
}

impl VNode {
    pub fn new(plant: Plant, position: EdgeId) -> Self {
        VNode {
            plant,
            position,
            memory: TopologicalMemory::new(),
            learned_rules: HashMap::new(),
            path_history: vec![position],
            rule_performance: HashMap::new(),
        }
    }

    /// Move the plant to a new position via a transformation
    pub fn apply_transformation(&mut self, transform: LPR, tonnetz: &Tonnetz) -> crate::Result<()> {
        // Get the current triad data
        let cplant = tonnetz
            .plants
            .get(&self.position)
            .ok_or_else(|| rshyper::Error::HyperedgeDoesNotExist(self.position.to_string()))?;

        // Apply the transformation to the plant
        let nplant = self.plant.apply_transform(transform);

        // find the destination triad in the Tonnetz
        let transformations = tonnetz.transformations.get(&self.position).ok_or_else(|| {
            crate::Error::Unknown("No transformations available for this triad".into())
        })?;
        // get the next position
        let next_position = transformations.get(&transform).ok_or_else(|| {
            crate::Error::Unknown(format!(
                "Transformation {:?} not valid from this triad",
                transform
            ))
        })?;

        // Update position
        self.position = *next_position;
        self.path_history.push(*next_position);

        // Record the transformation in memory
        let next_triad = tonnetz
            .plants
            .get(&*next_position)
            .ok_or_else(|| rshyper::Error::HyperedgeDoesNotExist(next_position.to_string()))?;

        self.memory.record_transformation(
            *cplant.alphabet(),
            cplant.class(),
            transform,
            *next_triad.alphabet(),
            next_triad.class(),
        );

        self.memory.advance_time();

        Ok(())
    }

    /// Learn rules from topological features
    pub fn learn_from_topology(&mut self, _tonnetz: &Tonnetz) {
        // Get active features as potential rule templates
        let active_features = self.memory.find_active_features();

        for feature in active_features {
            if feature.dimension == 1 && feature.content.len() >= 9 {
                // Extract the transformation pattern from the feature
                let triad = [feature.content[0], feature.content[1], feature.content[2]];
                let class = TriadClass::from(feature.content[3]);
                let transform = LPR::from(feature.content[4]);

                // Create a key representing the state and symbol
                let current_state = self.plant.state();
                let current_symbol = self.plant.utm.get_current_symbol();
                let pattern_key = vec![*current_state, current_symbol, feature.content[4]];

                // Define a rule based on this pattern
                let rule = Tail::new(
                    Direction::default(),     // Default direction
                    State(1) - current_state, // Switch state
                    feature.content[5],       // Use first note of result triad
                );

                // Store the rule
                self.learned_rules.insert(pattern_key, rule);
            }
        }
    }

    /// Apply learned rules to the UTM
    pub fn apply_learned_rules(&mut self) {
        let head = self.plant.utm.head();
        let Head { state, symbol } = head;

        // Try to find matching rules
        for transform_type in 0..3 {
            // 0=Leading, 1=Parallel, 2=Relative
            let pattern_key = vec![*state, symbol, transform_type];

            if let Some(&tail) = self.learned_rules.get(&pattern_key) {
                // Apply the rule
                self.plant.utm.ruleset.insert(head, tail);

                println!("Applied learned rule: {head} → {tail}");

                break;
            }
        }
    }

    /// Evaluate performance of a learned rule
    pub fn evaluate_rule_performance(&mut self, pattern_key: Vec<usize>, performance: f64) {
        let current_perf = self
            .rule_performance
            .entry(pattern_key.clone())
            .or_insert(0.0);
        // Use exponential moving average
        *current_perf = 0.9 * *current_perf + 0.1 * performance;
    }

    /// Get best performing rules
    pub fn get_best_rules(&self) -> Vec<(Vec<usize>, Tail<usize, usize>, f64)> {
        let mut rules: Vec<_> = self
            .learned_rules
            .iter()
            .map(|(key, &rule)| {
                let performance = self.rule_performance.get(key).cloned().unwrap_or(0.0);
                (key.clone(), rule, performance)
            })
            .collect();

        rules.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
        rules
    }
}
