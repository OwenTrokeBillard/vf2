use crate::state::State;
use crate::{Graph, Isomorphism};
use std::fmt::Debug;

/// An isomorphism iterator.
#[derive(Clone, Debug)]
pub struct IsomorphismIter<'a, Query, Data, NodeEq, EdgeEq> {
    state: State<'a, Query, Data, NodeEq, EdgeEq>,
}

impl<'a, Query, Data, NodeEq, EdgeEq> IsomorphismIter<'a, Query, Data, NodeEq, EdgeEq>
where
    Query: Graph,
    Data: Graph,
    NodeEq: Fn(&Query::NodeLabel, &Data::NodeLabel) -> bool,
    EdgeEq: Fn(&Query::EdgeLabel, &Data::EdgeLabel) -> bool,
{
    pub(crate) fn new(
        query: &'a Query,
        data: &'a Data,
        node_eq: Option<NodeEq>,
        edge_eq: Option<EdgeEq>,
        induced: bool,
    ) -> Self {
        Self {
            state: State::new(query, data, node_eq, edge_eq, induced),
        }
    }

    /// Advances the search and returns the next isomorphism.
    ///
    /// Unlike [`next`], this does not allocate.
    /// Returns [`None`] if the search is complete.
    ///
    /// [`next`]: Self::next
    pub fn into_next(mut self) -> Option<Isomorphism> {
        match self.next_ref() {
            None => None,
            Some(_) => Some(self.state.into_query_map()),
        }
    }

    /// Advances the search and returns a reference
    /// to the next isomorphism.
    ///
    /// Unlike [`next`], this returns a reference so as not to allocate.
    /// Returns [`None`] when the search is complete.
    ///
    /// [`next`]: Self::next
    pub fn next_ref(&mut self) -> Option<&Isomorphism> {
        while !self.state.step() {}
        self.state.all_covered().then_some(self.state.query_map())
    }
}

impl<'a, Query, Data, NodeEq, EdgeEq> Iterator for IsomorphismIter<'a, Query, Data, NodeEq, EdgeEq>
where
    Query: Graph,
    Data: Graph,
    NodeEq: Fn(&Query::NodeLabel, &Data::NodeLabel) -> bool,
    EdgeEq: Fn(&Query::EdgeLabel, &Data::EdgeLabel) -> bool,
{
    type Item = Isomorphism;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_ref().cloned()
    }
}
