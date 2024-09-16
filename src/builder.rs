use crate::{Graph, Isomorphism, IsomorphismIter};
use std::fmt::Debug;

/// Creates a new [`Vf2ppBuilder`] to find
/// isomorphisms from `query` to `data`.
///
/// Node and edge equality are not checked by default.
/// Use [`node_eq`], [`edge_eq`], and [`default_eq`]
/// on the builder to set equality functions.
///
/// [`node_eq`]: Vf2ppBuilder::node_eq
/// [`edge_eq`]: Vf2ppBuilder::edge_eq
/// [`default_eq`]: Vf2ppBuilder::default_eq
pub fn isomorphisms<'a, Query, Data>(
    query: &'a Query,
    data: &'a Data,
) -> DefaultVf2ppBuilder<'a, Query, Data>
where
    Query: Graph,
    Data: Graph,
{
    DefaultVf2ppBuilder::new(Problem::Isomorphism, query, data)
}

/// Creates a new [`Vf2ppBuilder`] to find
/// subgraph isomorphisms from `query` to `data`.
///
/// Node and edge equality are not checked by default.
/// Use [`node_eq`], [`edge_eq`], and [`default_eq`]
/// on the builder to set equality functions.
///
/// [`node_eq`]: Vf2ppBuilder::node_eq
/// [`edge_eq`]: Vf2ppBuilder::edge_eq
/// [`default_eq`]: Vf2ppBuilder::default_eq
pub fn subgraph_isomorphisms<'a, Query, Data>(
    query: &'a Query,
    data: &'a Data,
) -> DefaultVf2ppBuilder<'a, Query, Data>
where
    Query: Graph,
    Data: Graph,
{
    DefaultVf2ppBuilder::new(Problem::SubgraphIsomorphism, query, data)
}

/// Creates a new [`Vf2ppBuilder`] to find
/// induced subgraph isomorphisms from `query` to `data`.
///
/// Node and edge equality are not checked by default.
/// Use [`node_eq`], [`edge_eq`], and [`default_eq`]
/// on the builder to set equality functions.
///
/// [`node_eq`]: Vf2ppBuilder::node_eq
/// [`edge_eq`]: Vf2ppBuilder::edge_eq
/// [`default_eq`]: Vf2ppBuilder::default_eq
pub fn induced_subgraph_isomorphisms<'a, Query, Data>(
    query: &'a Query,
    data: &'a Data,
) -> DefaultVf2ppBuilder<'a, Query, Data>
where
    Query: Graph,
    Data: Graph,
{
    DefaultVf2ppBuilder::new(Problem::InducedSubgraphIsomorphism, query, data)
}

/// A VF2++ builder.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Vf2ppBuilder<'a, Query, Data, NodeEq, EdgeEq> {
    /// Problem type.
    problem: Problem,
    /// Query graph.
    query: &'a Query,
    /// Data graph.
    data: &'a Data,
    /// Node equality function.
    node_eq: Option<NodeEq>,
    /// Edge equality function.
    edge_eq: Option<EdgeEq>,
}

/// Default VF2++ builder type.
///
/// This is [`Vf2ppBuilder`] with function pointers as
/// the node and edge equality function types.
pub type DefaultVf2ppBuilder<'a, Query, Data> = Vf2ppBuilder<
    'a,
    Query,
    Data,
    fn(&<Query as Graph>::NodeLabel, &<Data as Graph>::NodeLabel) -> bool,
    fn(&<Query as Graph>::EdgeLabel, &<Data as Graph>::EdgeLabel) -> bool,
>;

impl<'a, Query, Data> DefaultVf2ppBuilder<'a, Query, Data>
where
    Query: Graph,
    Data: Graph,
{
    /// Creates a new [`Vf2ppBuilder`] that does not check
    /// node and edge equality.
    fn new(problem: Problem, query: &'a Query, data: &'a Data) -> Self {
        Self {
            problem,
            query,
            data,
            node_eq: None,
            edge_eq: None,
        }
    }
}

impl<'a, Query, Data, NodeEq, EdgeEq> Vf2ppBuilder<'a, Query, Data, NodeEq, EdgeEq>
where
    Query: Graph,
    Data: Graph,
    NodeEq: Fn(&Query::NodeLabel, &Data::NodeLabel) -> bool,
    EdgeEq: Fn(&Query::EdgeLabel, &Data::EdgeLabel) -> bool,
{
    /// Configures VF2++ to use the [`PartialEq`] implementations
    /// for node and edge equalities.
    pub fn default_eq(self) -> DefaultVf2ppBuilder<'a, Query, Data>
    where
        Query::NodeLabel: PartialEq<Data::NodeLabel>,
        Query::EdgeLabel: PartialEq<Data::EdgeLabel>,
    {
        Vf2ppBuilder {
            problem: self.problem,
            query: self.query,
            data: self.data,
            node_eq: Some(<Query::NodeLabel as PartialEq<Data::NodeLabel>>::eq),
            edge_eq: Some(<Query::EdgeLabel as PartialEq<Data::EdgeLabel>>::eq),
        }
    }

    /// Configures VF2++ to use `node_eq` as the node equality function.
    pub fn node_eq<NewNodeEq>(
        self,
        node_eq: NewNodeEq,
    ) -> Vf2ppBuilder<'a, Query, Data, NewNodeEq, EdgeEq>
    where
        NewNodeEq: Fn(&Query::NodeLabel, &Data::NodeLabel) -> bool,
    {
        Vf2ppBuilder {
            problem: self.problem,
            query: self.query,
            data: self.data,
            node_eq: Some(node_eq),
            edge_eq: self.edge_eq,
        }
    }

    /// Configures VF2++ to use `edge_eq` as the edge equality function.
    pub fn edge_eq<NewEdgeEq>(
        self,
        edge_eq: NewEdgeEq,
    ) -> Vf2ppBuilder<'a, Query, Data, NodeEq, NewEdgeEq>
    where
        NewEdgeEq: Fn(&Query::EdgeLabel, &Data::EdgeLabel) -> bool,
    {
        Vf2ppBuilder {
            problem: self.problem,
            query: self.query,
            data: self.data,
            node_eq: self.node_eq,
            edge_eq: Some(edge_eq),
        }
    }

    /// Returns the first isomorphism
    /// from the query graph to the data graph.
    pub fn first(self) -> Option<Isomorphism> {
        self.iter().into_next()
    }

    /// Returns a vector of isomorphisms
    /// from the query graph to the data graph.
    pub fn vec(self) -> Vec<Isomorphism> {
        self.iter().collect()
    }

    /// Returns an iterator of isomorphisms
    /// from the query graph to the data graph.
    pub fn iter(self) -> IsomorphismIter<'a, Query, Data, NodeEq, EdgeEq> {
        if self.problem == Problem::Isomorphism {
            assert_eq!(
                self.query.node_count(),
                self.data.node_count(),
                "graphs must be the same size"
            );
        }
        let induced = match self.problem {
            Problem::Isomorphism => true,
            Problem::SubgraphIsomorphism => false,
            Problem::InducedSubgraphIsomorphism => true,
        };
        IsomorphismIter::new(self.query, self.data, self.node_eq, self.edge_eq, induced)
    }
}

/// Problem type.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Problem {
    /// Graph isomorphism.
    Isomorphism,
    /// Subgraph isomorphism.
    SubgraphIsomorphism,
    /// Induced subgraph isomorphism.
    InducedSubgraphIsomorphism,
}
