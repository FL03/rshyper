/*
    appellation: tracker <module>
    authors: @FL03
*/
use super::{EdgeId, Frame, IndexError, RawIndex, VertexId};
use crate::AddStep;
use alloc::vec::Vec;

/// the [`IndexTracker`] keeps a history of the created indices and the current cursor position.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(C)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default, rename_all = "snake_case")
)]
pub struct IndexTracker<Ix = usize>
where
    Ix: RawIndex,
{
    pub(crate) cursor: Frame<Ix>,
    pub(crate) edges: Vec<EdgeId<Ix>>,
    pub(crate) points: Vec<VertexId<Ix>>,
}

impl<Ix> IndexTracker<Ix>
where
    Ix: RawIndex,
{
    /// creates a new instance with an empty history and a default cursor.
    pub fn new() -> Self
    where
        Ix: Default,
    {
        Self::from_cursor(Frame::default())
    }
    /// creates a new instance with an empty history and the given cursor.
    pub fn from_cursor(cursor: Frame<Ix>) -> Self {
        Self {
            cursor,
            edges: Vec::new(),
            points: Vec::new(),
        }
    }
    /// create a new history with the cursor initialized to `1`
    pub fn one() -> Self
    where
        Ix: num_traits::One,
    {
        Self::from_cursor(Frame::one())
    }
    /// create a new history with the cursor initialized to `0`
    pub fn zero() -> Self
    where
        Ix: num_traits::Zero,
    {
        Self::from_cursor(Frame::zero())
    }
    /// returns a reference to the current cursor.
    pub const fn cursor(&self) -> &Frame<Ix> {
        &self.cursor
    }
    /// returns a mutable reference to the current cursor.
    pub const fn cursor_mut(&mut self) -> &mut Frame<Ix> {
        &mut self.cursor
    }
    /// returns an immutable reference to the edge history
    pub const fn edges(&self) -> &Vec<EdgeId<Ix>> {
        &self.edges
    }
    /// returns a mutable reference to the edge history
    pub const fn edges_mut(&mut self) -> &mut Vec<EdgeId<Ix>> {
        &mut self.edges
    }
    /// returns an immutable reference to the node history
    pub const fn points(&self) -> &Vec<VertexId<Ix>> {
        &self.points
    }
    /// returns a mutable reference to the history of created nodes
    pub const fn points_mut(&mut self) -> &mut Vec<VertexId<Ix>> {
        &mut self.points
    }
    /// set the current position and return a mutable reference to the tracker
    #[inline]
    pub fn set_cursor(&mut self, cursor: Frame<Ix>) -> &mut Self {
        *self.cursor_mut() = cursor;
        self
    }
    /// overwrite the edge history and return a mutable reference to the tracker
    #[inline]
    pub fn set_edges(&mut self, edges: Vec<EdgeId<Ix>>) -> &mut Self {
        *self.edges_mut() = edges;
        self
    }
    /// overwrite the node history and return a mutable reference to the tracker
    #[inline]
    pub fn set_points(&mut self, nodes: Vec<VertexId<Ix>>) -> &mut Self {
        *self.points_mut() = nodes;
        self
    }
    /// consumes the current instance to create another with the given position
    #[inline]
    pub fn with_cursor(self, cursor: Frame<Ix>) -> Self {
        Self { cursor, ..self }
    }
    /// consumes the current instance to create another with the given edge history
    #[inline]
    pub fn with_edges(self, edges: Vec<EdgeId<Ix>>) -> Self {
        Self { edges, ..self }
    }
    /// consumes the current instance to create another with the given node history
    #[inline]
    pub fn with_points(self, nodes: Vec<VertexId<Ix>>) -> Self {
        Self {
            points: nodes,
            ..self
        }
    }
    /// add a new edge index to the history.
    #[inline]
    pub fn add_edge(&mut self, index: EdgeId<Ix>) -> &mut Self {
        self.edges_mut().push(index);
        self
    }
    /// add a new node index to the history.
    #[inline]
    pub fn add_point(&mut self, index: VertexId<Ix>) -> &mut Self {
        self.points_mut().push(index);
        self
    }
    /// returns true if the element is in the edge history
    #[inline]
    pub fn contains_edge(&self, index: &EdgeId<Ix>) -> bool
    where
        Ix: PartialEq,
    {
        self.edges().contains(index)
    }
    /// returns true if the element is in the node history
    #[inline]
    pub fn contains_point(&self, index: &VertexId<Ix>) -> bool
    where
        Ix: PartialEq,
    {
        self.points().contains(index)
    }
    /// returns a reference to the edge if at the given index
    #[inline]
    pub fn get_edge<I>(&self, index: I) -> Option<&I::Output>
    where
        I: core::slice::SliceIndex<[EdgeId<Ix>]>,
    {
        self.edges().get(index)
    }
    /// returns a mutable reference to the edge id
    #[inline]
    pub fn get_edge_mut<I>(&mut self, index: I) -> Option<&mut I::Output>
    where
        I: core::slice::SliceIndex<[EdgeId<Ix>]>,
    {
        self.edges_mut().get_mut(index)
    }
    /// returns a reference to the node if at the given index
    #[inline]
    pub fn get_point<I>(&self, index: I) -> Option<&I::Output>
    where
        I: core::slice::SliceIndex<[VertexId<Ix>]>,
    {
        self.points().get(index)
    }
    /// returns a mutable reference to the node id
    #[inline]
    pub fn get_point_mut<I>(&mut self, index: I) -> Option<&mut I::Output>
    where
        I: core::slice::SliceIndex<[VertexId<Ix>]>,
    {
        self.points_mut().get_mut(index)
    }
    /// remove the index from the history for the specified kind.
    #[inline]
    pub fn remove_edge(&mut self, index: &Ix) -> &mut Self
    where
        Ix: PartialEq,
    {
        self.retain_edge(|i| i != index);
        self
    }
    /// removes an edge index from the history for the specified kind.
    #[inline]
    pub fn remove_edge_at(&mut self, index: usize) -> &mut Self {
        self.edges_mut().remove(index);
        self
    }
    /// remove the index from the history for the specified kind.
    #[inline]
    pub fn remove_point(&mut self, index: &Ix) -> &mut Self
    where
        Ix: PartialEq,
    {
        self.retain_point(|i| i != index);
        self
    }
    /// removes an edge index from the history for the specified kind.
    #[inline]
    pub fn remove_point_at(&mut self, index: usize) -> &mut Self {
        self.points_mut().remove(index);
        self
    }
    /// remove the index from the history for the specified kind.
    #[inline]
    pub fn retain_edge<F>(&mut self, f: F)
    where
        F: FnMut(&EdgeId<Ix>) -> bool,
    {
        self.edges_mut().retain(f);
    }
    /// retains only nodes satisfying the predicate `f`.
    #[inline]
    pub fn retain_point<F>(&mut self, f: F)
    where
        F: FnMut(&VertexId<Ix>) -> bool,
    {
        self.points_mut().retain(f);
    }
    /// steps the edge index forward before storing and returning the previous index
    #[inline]
    pub fn next_edge(&mut self) -> Result<EdgeId<Ix>, IndexError>
    where
        Ix: AddStep<Output = Ix> + Copy + PartialEq,
    {
        // step the current edge cursor forward before replacing and returning
        // the previous index
        let prev = self.cursor_mut().next_edge()?;
        // check if the previous edge index is already in the history
        if self.edges().contains(&prev) {
            return Err(IndexError::DuplicateIndex);
        }
        // add the previous edge index to the history
        self.add_edge(prev);
        // return the previous edge index
        Ok(prev)
    }
    /// steps the node index forward before storing and returning the previous index
    #[inline]
    pub fn next_point(&mut self) -> Result<VertexId<Ix>, IndexError>
    where
        Ix: AddStep<Output = Ix> + Copy + PartialEq,
    {
        // step the current node cursor forward before replacing and returning
        // the previous index
        let prev = self.cursor_mut().next_point()?;
        // check if the previous node index is already in the history
        if self.points().contains(&prev) {
            return Err(IndexError::DuplicateIndex);
        }
        // add the previous node index to the history
        self.add_point(prev);
        // return the previous node index
        Ok(prev)
    }
}

#[allow(deprecated)]
#[doc(hidden)]
impl<Ix> IndexTracker<Ix>
where
    Ix: RawIndex,
{
    #[deprecated(since = "0.1.2", note = "use `add_edge` instead")]
    pub fn add_edge_index(&mut self, index: EdgeId<Ix>) -> &mut Self {
        self.add_edge(index)
    }
    #[deprecated(since = "0.1.2", note = "use `add_node` instead")]
    pub fn add_vertex_index(&mut self, index: VertexId<Ix>) -> &mut Self {
        self.add_point(index)
    }
    #[deprecated(since = "0.1.2", note = "use `remove_node` instead")]
    pub fn remove_vertex(&mut self, index: &Ix) -> &mut Self
    where
        Ix: PartialEq,
    {
        self.remove_point(index)
    }
    #[deprecated(since = "0.1.2", note = "use `next_edge` instead")]
    pub fn next_edge_index(&mut self) -> Result<EdgeId<Ix>, IndexError>
    where
        Ix: AddStep<Output = Ix> + Copy + PartialEq,
    {
        self.next_edge()
    }
    #[deprecated(since = "0.1.2", note = "use `next_point` instead")]
    pub fn next_vertex_index(&mut self) -> Result<VertexId<Ix>, IndexError>
    where
        Ix: AddStep<Output = Ix> + Copy + PartialEq,
    {
        self.next_point()
    }
}
