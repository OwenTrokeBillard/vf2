use crate::{Direction, Graph, NodeIndex};
use petgraph::adj::IndexType;
use petgraph::EdgeType;
use std::fmt::Debug;

impl<N, E, Ty, Ix> Graph for petgraph::Graph<N, E, Ty, Ix>
where
    N: Debug,
    E: Debug,
    Ty: EdgeType,
    Ix: IndexType,
{
    type NodeLabel = N;
    type EdgeLabel = E;

    #[inline]
    fn is_directed(&self) -> bool {
        self.is_directed()
    }

    #[inline]
    fn node_count(&self) -> usize {
        self.node_count()
    }

    #[inline]
    fn node_label(&self, index: NodeIndex) -> Option<&Self::NodeLabel> {
        self.node_weight(petgraph::graph::NodeIndex::<Ix>::new(index))
    }

    #[inline]
    fn neighbors(&self, node: NodeIndex, direction: Direction) -> impl Iterator<Item = NodeIndex> {
        self.neighbors_directed(
            petgraph::graph::NodeIndex::<Ix>::new(node),
            match direction {
                Direction::Outgoing => petgraph::Direction::Outgoing,
                Direction::Incoming => petgraph::Direction::Incoming,
            },
        )
        .map(|neighbor| neighbor.index())
    }

    #[inline]
    fn contains_edge(&self, source: NodeIndex, target: NodeIndex) -> bool {
        self.contains_edge(
            petgraph::graph::NodeIndex::<Ix>::new(source),
            petgraph::graph::NodeIndex::<Ix>::new(target),
        )
    }

    #[inline]
    fn edge_label(&self, source: NodeIndex, target: NodeIndex) -> Option<&Self::EdgeLabel> {
        self.find_edge(
            petgraph::graph::NodeIndex::<Ix>::new(source),
            petgraph::graph::NodeIndex::<Ix>::new(target),
        )
        .and_then(|index| self.edge_weight(index))
    }
}
