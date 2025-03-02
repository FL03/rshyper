/*
    Appellation: memory <module>
    Contrib: @FL03
*/

use crate::types::{Transformation, TriadClass};
use rshyper::EdgeId;
use std::collections::{HashMap, HashSet};

/// Represents a topological feature that persists across transformations
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PersistentFeature {
    /// Unique identifier for the feature
    pub id: usize,

    /// The birth time of this feature (when it was created)
    pub birth: usize,

    /// The death time of this feature (when it disappeared), None if still active
    pub death: Option<usize>,

    /// The dimension of this feature (0 = point, 1 = edge, etc.)
    pub dimension: usize,

    /// Associated content with this feature
    pub content: Vec<usize>,
}

/// Memory system based on persistent homology
pub struct TopologicalMemory {
    /// Collection of persistent features
    pub features: Vec<PersistentFeature>,

    /// Current time counter
    pub time: usize,

    /// Features indexed by dimension for quick access
    dimension_index: HashMap<usize, HashSet<usize>>,

    /// Features indexed by content
    content_index: HashMap<Vec<usize>, HashSet<usize>>,

    /// Next feature ID
    next_id: usize,
}

impl TopologicalMemory {
    pub fn new() -> Self {
        TopologicalMemory {
            features: Vec::new(),
            time: 0,
            dimension_index: HashMap::new(),
            content_index: HashMap::new(),
            next_id: 0,
        }
    }

    /// Create a new feature with the given dimension and content
    pub fn create_feature(&mut self, dimension: usize, content: Vec<usize>) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        // Create the feature
        let feature = PersistentFeature {
            id,
            birth: self.time,
            death: None,
            dimension,
            content: content.clone(),
        };

        // Add to the collection
        self.features.push(feature);

        // Update indices
        self.dimension_index
            .entry(dimension)
            .or_insert_with(HashSet::new)
            .insert(id);

        self.content_index
            .entry(content)
            .or_insert_with(HashSet::new)
            .insert(id);

        id
    }

    /// End the life of a feature
    pub fn kill_feature(&mut self, id: usize) {
        if let Some(feature) = self
            .features
            .iter_mut()
            .find(|f| f.id == id && f.death.is_none())
        {
            feature.death = Some(self.time);
        }
    }

    /// Advance the time counter
    pub fn advance_time(&mut self) {
        self.time += 1;
    }

    /// Find features containing specific content
    pub fn find_by_content(&self, content: &[usize]) -> Vec<&PersistentFeature> {
        if let Some(ids) = self.content_index.get(content) {
            ids.iter()
                .filter_map(|&id| self.features.iter().find(|f| f.id == id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Find features of a specific dimension
    pub fn find_by_dimension(&self, dimension: usize) -> Vec<&PersistentFeature> {
        if let Some(ids) = self.dimension_index.get(&dimension) {
            ids.iter()
                .filter_map(|&id| self.features.iter().find(|f| f.id == id))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Find active features (those that haven't died)
    pub fn find_active_features(&self) -> Vec<&PersistentFeature> {
        self.features.iter().filter(|f| f.death.is_none()).collect()
    }

    /// Record a transformation applied to a triad
    pub fn record_transformation(
        &mut self,
        triad: [usize; 3],
        class: TriadClass,
        transform: Transformation,
        result_triad: [usize; 3],
        result_class: TriadClass,
    ) -> usize {
        // Create a feature representing this transformation
        let content = vec![
            triad[0],
            triad[1],
            triad[2],
            class as usize,
            transform as usize,
            result_triad[0],
            result_triad[1],
            result_triad[2],
            result_class as usize,
        ];

        self.create_feature(1, content) // Dimension 1 for transformations
    }

    /// Analyze persistence of features
    pub fn persistence_diagram(&self) -> Vec<(usize, usize, Option<usize>)> {
        self.features
            .iter()
            .map(|f| (f.id, f.birth, f.death))
            .collect()
    }
}
