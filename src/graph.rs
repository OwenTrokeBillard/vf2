/// A graph.
pub trait Graph {
    /// Node label type.
    type NodeLabel;

    /// Edge label type.
    type EdgeLabel;

    /// Returns `true` if the graph is directed
    /// or `false` if the graph is undirected.
    fn is_directed(&self) -> bool;

    /// Returns the number of nodes in the graph.
    fn node_count(&self) -> usize;

    /// Returns a reference to the label of `node`;
    fn node_label(&self, node: NodeIndex) -> Option<&Self::NodeLabel>;

    /// Returns an iterator of neighbors of `node`.
    ///
    /// If the graph is directed, returns neighbors in `direction` only.
    /// If undirected, ignores `direction` and returns all neighbors.
    fn neighbors(&self, node: NodeIndex, direction: Direction) -> impl Iterator<Item = NodeIndex>;

    /// Returns `true` if there is an edge from `source` to `target`.
    ///
    /// If the graph is directed, the edge must go from `source` to `target`.
    /// If undirected, an edge must exist between `source` and `target`.
    fn contains_edge(&self, source: NodeIndex, target: NodeIndex) -> bool;

    /// Returns a reference to the label of the edge from `source` to `target`.
    ///
    /// If the graph is directed, the edge must go from `source` to `target`.
    /// If undirected, the edge must be between `source` and `target`.
    fn edge_label(&self, source: NodeIndex, target: NodeIndex) -> Option<&Self::EdgeLabel>;
}

/// A node index.
pub type NodeIndex = usize;

/// Edge direction.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Direction {
    Outgoing,
    Incoming,
}
