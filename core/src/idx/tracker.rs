/*
    appellation: tracker <module>
    authors: @FL03
*/
use super::{EdgeId, IndexCursor, IndexError, IndexKind, RawIndex, VertexId};
use crate::AddStep;
use core::hash::Hash;
use std::collections::HashMap;

/// the [`IndexTracker`] keeps a history of the created indices and the current cursor position.
#[derive(Clone, Debug, Default)]
#[repr(C)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "snake_case")
)]
pub struct IndexTracker<Ix>
where
    Ix: RawIndex,
{
    pub(crate) cursor: IndexCursor<Ix>,
    pub(crate) history: HashMap<IndexKind, Vec<Ix>>,
}

impl<Ix> IndexTracker<Ix>
where
    Ix: RawIndex + Eq + Hash,
{
    /// creates a new instance with an empty history and a default cursor.
    pub fn new() -> Self
    where
        Ix: Default,
    {
        let mut history = HashMap::new();
        history.insert(IndexKind::Edge, Vec::new());
        history.insert(IndexKind::Vertex, Vec::new());

        Self {
            cursor: IndexCursor::default(),
            history,
        }
    }
    /// returns a reference to the current cursor.
    pub const fn cursor(&self) -> &IndexCursor<Ix> {
        &self.cursor
    }
    /// returns a mutable reference to the current cursor.
    pub const fn cursor_mut(&mut self) -> &mut IndexCursor<Ix> {
        &mut self.cursor
    }
    /// returns a reference to the history of indices
    pub const fn history(&self) -> &HashMap<IndexKind, Vec<Ix>> {
        &self.history
    }
    /// returns a mutable reference to the history of indices
    pub const fn history_mut(&mut self) -> &mut HashMap<IndexKind, Vec<Ix>> {
        &mut self.history
    }
    /// set the current position and return a mutable reference to the tracker
    pub fn set_cursor(&mut self, cursor: IndexCursor<Ix>) -> &mut Self {
        *self.cursor_mut() = cursor;
        self
    }
    /// overwrite the history and return a mutable reference to the tracker
    pub fn set_history(&mut self, history: HashMap<IndexKind, Vec<Ix>>) -> &mut Self {
        *self.history_mut() = history;
        self
    }
    /// consumes the current instance to create another with the given position
    pub fn with_cursor(self, cursor: IndexCursor<Ix>) -> Self {
        Self { cursor, ..self }
    }
    /// consumes the current instance to create another with the given history
    pub fn with_history(self, history: HashMap<IndexKind, Vec<Ix>>) -> Self {
        Self { history, ..self }
    }
    /// add a new index to the history under the specified kind.
    pub fn add_index(&mut self, kind: IndexKind, index: Ix) -> &mut Self {
        self.history.entry(kind).or_default().push(index);
        self
    }
    /// add a new edge index to the history.
    pub fn add_edge_index(&mut self, index: Ix) -> &mut Self {
        self.add_index(IndexKind::Edge, index)
    }
    /// add a new vertex index to the history.
    pub fn add_vertex_index(&mut self, index: Ix) -> &mut Self {
        self.add_index(IndexKind::Vertex, index)
    }
    /// returns a reference to the set of indicies for the specified kind.
    pub fn get_history<Q>(&self, kind: &Q) -> Option<&Vec<Ix>>
    where
        Q: Eq + Hash,
        IndexKind: core::borrow::Borrow<Q>,
    {
        self.history().get(kind)
    }
    /// returns a mutable reference to the set of indices for the specified kind.
    pub fn get_history_mut<Q>(&mut self, kind: &Q) -> Option<&mut Vec<Ix>>
    where
        Q: Eq + Hash,
        IndexKind: core::borrow::Borrow<Q>,
    {
        self.history_mut().get_mut(kind)
    }
    /// removes an edge index from the history for the specified kind.
    pub fn remove_edge(&mut self, index: &Ix) -> &mut Self {
        if let Some(indices) = self.get_history_mut(&IndexKind::Edge) {
            indices.retain(|i| i != index);
        } else {
            #[cfg(feature = "tracing")]
            tracing::warn!("No edge indices found in history to retain.");
        }
        self
    }
    /// remove the index from the history for the specified kind.
    pub fn remove_vertex(&mut self, index: &Ix) -> &mut Self {
        if let Some(indices) = self.get_history_mut(&IndexKind::Vertex) {
            indices.retain(|i| i != index);
        } else {
            #[cfg(feature = "tracing")]
            tracing::warn!("No vertex indices found in history to retain.");
        }
        self
    }
    /// steps the edge index forward before storing and returning the previous index
    pub fn next_edge(&mut self) -> Result<EdgeId<Ix>, IndexError>
    where
        Ix: Copy + AddStep<Output = Ix>,
    {
        let prev = self.cursor_mut().next_edge()?;
        self.add_edge_index(*prev.get());
        Ok(prev)
    }
    /// steps the vertex index forward before storing and returning the previous index
    pub fn next_vertex(&mut self) -> Result<VertexId<Ix>, IndexError>
    where
        Ix: Copy + AddStep<Output = Ix>,
    {
        let prev = self.cursor_mut().next_vertex()?;
        self.add_vertex_index(*prev.get());
        Ok(prev)
    }
}
