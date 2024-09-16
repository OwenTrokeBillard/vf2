use crate::{Direction, Graph, NodeIndex};
use std::fmt::Debug;

/// A reserved value indicating the node is uncovered.
/// Assumes the graph size is below [`NodeIndex::MAX`].
const NOT_IN_MAP: NodeIndex = NodeIndex::MAX;

/// A reserved value indicating the node is not in the set.
const NOT_IN_SET: NodeIndex = 0;

#[derive(Clone, Debug)]
pub(crate) struct State<'a, Query, Data, NodeEq, EdgeEq> {
    /// Whether the subgraph is induced.
    induced: bool,
    /// Depth in the SSR tree.
    depth: usize,
    /// Query graph state.
    query: GraphState<'a, Query>,
    /// Data graph state.
    data: GraphState<'a, Data>,
    /// A stack of candidate pair sources.
    ///
    /// The value at index `i` is the source of nodes at depth `i + 1`.
    source_stack: Vec<Source>,
    /// The previous candidate pair at the current depth.
    previous: Option<Pair>,
    /// Node equality function.
    node_eq: Option<NodeEq>,
    /// Edge equality function.
    edge_eq: Option<EdgeEq>,
}

impl<'a, Query, Data, NodeEq, EdgeEq> State<'a, Query, Data, NodeEq, EdgeEq>
where
    Query: Graph,
    Data: Graph,
    NodeEq: Fn(&Query::NodeLabel, &Data::NodeLabel) -> bool,
    EdgeEq: Fn(&Query::EdgeLabel, &Data::EdgeLabel) -> bool,
{
    /// Creates a new [`State`].
    pub(crate) fn new(
        query: &'a Query,
        data: &'a Data,
        node_eq: Option<NodeEq>,
        edge_eq: Option<EdgeEq>,
        induced: bool,
    ) -> Self {
        assert!(query.node_count() > 0, "query graph cannot be empty");
        assert!(
            query.node_count() <= data.node_count(),
            "query graph cannot have more nodes than data graph"
        );
        assert!(
            data.node_count() < NOT_IN_MAP,
            "data graph is so large it uses reserved values"
        );
        Self {
            induced,
            depth: 0,
            query: GraphState::new(query),
            data: GraphState::new(data),
            source_stack: vec![Source::Outgoing; query.node_count()],
            previous: None,
            node_eq,
            edge_eq,
        }
    }

    /// Advances the search one step. Returns `true`
    /// if the map is ready or the search is complete.
    pub(crate) fn step(&mut self) -> bool {
        if let Some(pair) = self.next_pair() {
            self.previous = Some(pair);
            if self.feasible(pair) {
                self.push(pair);
            }
            self.all_covered()
        } else if self.depth > 0 {
            self.pop();
            false
        } else {
            true
        }
    }

    /// Pushes `pair` to the partial map. Increments depth.
    fn push(&mut self, pair: Pair) {
        self.depth += 1;
        self.previous = None;
        self.query.push(pair.query_node, pair.data_node, self.depth);
        self.data.push(pair.data_node, pair.query_node, self.depth);
    }

    /// Pops the last pair from the partial map. Decrements depth.
    fn pop(&mut self) {
        self.previous = Some(Pair {
            query_node: self.query.pop(self.depth),
            data_node: self.data.pop(self.depth),
        });
        self.depth -= 1;
    }

    /// Returns the next candidate pair.
    fn next_pair(&mut self) -> Option<Pair> {
        if self.all_covered() {
            None
        } else if let Some(previous) = self.previous {
            let source = self.source_stack[self.depth];
            self.following_pair(source, previous)
        } else {
            self.first_pair().map(|(pair, source)| {
                self.source_stack[self.depth] = source;
                pair
            })
        }
    }

    /// Returns the first candidate pair and its source.
    fn first_pair(&self) -> Option<(Pair, Source)> {
        let source = if self.query.outgoing_size > 0 && self.data.outgoing_size > 0 {
            Source::Outgoing
        } else if self.query.incoming_size > 0 && self.data.incoming_size > 0 {
            Source::Incoming
        } else {
            Source::Uncovered
        };
        self.first_pair_in(source).map(|pair| (pair, source))
    }

    /// Returns the first candidate pair from `source`.
    fn first_pair_in(&self, source: Source) -> Option<Pair> {
        if let Some(query_node) = self.query.first_node(source) {
            if let Some(data_node) = self.data.first_node(source) {
                return Some(Pair::new(query_node, data_node));
            }
        }
        None
    }

    /// Returns the candidate pair from `source` following `previous`.
    fn following_pair(&self, source: Source, previous: Pair) -> Option<Pair> {
        self.data
            .next_node(source, previous.data_node + 1)
            .map(|data_node| Pair::new(previous.query_node, data_node))
    }

    /// Returns `true` if a successor state would remain
    /// consistent with `pair` in the partial map.
    ///
    /// This is *F(s, n, m)* in the original VF2 paper.
    fn feasible(&self, pair: Pair) -> bool {
        self.feasible_syntactic(pair) && self.feasible_semantic(pair)
    }

    /// Returns `true` if a successor state would remain
    /// syntactically consistent with `pair` in the partial map.
    /// That is, if the graph structures would match.
    ///
    /// This is *F_syn* in the original VF2 paper.
    fn feasible_syntactic(&self, pair: Pair) -> bool {
        let consistent = if self.is_directed() {
            self.rule_neighbors(pair, Direction::Incoming)
                && self.rule_neighbors(pair, Direction::Outgoing)
        } else {
            // This will check all neighbors since the graphs are undirected.
            self.rule_neighbors(pair, Direction::Incoming)
        };
        consistent && self.rule_in(pair) && self.rule_out(pair) && self.rule_new(pair)
    }

    /// Returns `true` if the predecessors or successors rule
    /// is satisfied, depending on `direction`.
    ///
    /// [`Direction::Incoming`] is the predecessors rule.
    ///
    /// This is *R_pred* or *R_succ* in the original VF2 paper.
    fn rule_neighbors(&self, pair: Pair, direction: Direction) -> bool {
        let source_target = |node, neighbor| match direction {
            Direction::Outgoing => (node, neighbor),
            Direction::Incoming => (neighbor, node),
        };
        for neighbor in self
            .query
            .graph
            // If the graph is undirected, this returns all neighbors.
            .neighbors(pair.query_node, direction)
            .filter(|&n| self.query.is_covered(n))
        {
            let mapped = self.query.map[neighbor];
            let (source, target) = source_target(pair.data_node, mapped);
            if !self.data.graph.contains_edge(source, target) {
                return false;
            }
        }
        if !self.induced {
            return true;
        }
        for neighbor in self
            .data
            .graph
            // If the graph is undirected, this returns all neighbors.
            .neighbors(pair.data_node, direction)
            .filter(|&n| self.data.is_covered(n))
        {
            let mapped = self.data.map[neighbor];
            let (source, target) = source_target(pair.query_node, mapped);
            if !self.query.graph.contains_edge(source, target) {
                return false;
            }
        }
        true
    }

    /// Returns `true` if the in rule is satisfied.
    ///
    /// This is *R_in* in the original VF2 paper.
    fn rule_in(&self, _pair: Pair) -> bool {
        // Not implemented. The algorithm works without
        // this, but may be much slower.
        true
    }

    /// Returns `true` if the out rule is satisfied.
    ///
    /// This is *R_out* in the original VF2 paper.
    fn rule_out(&self, _pair: Pair) -> bool {
        // Not implemented. The algorithm works without
        // this, but may be much slower.
        true
    }

    /// Returns `true` if the new rule is satisfied.
    ///
    /// This is *R_new* in the original VF2 paper.
    fn rule_new(&self, _pair: Pair) -> bool {
        // Not implemented. The algorithm works without
        // this, but may be much slower.
        true
    }

    /// Returns `true` if a successor state would remain
    /// semantically consistent with `pair` in the partial map.
    /// That is, if the node and edge labels would match.
    ///
    /// This is *F_sem* in the original VF2 paper.
    fn feasible_semantic(&self, pair: Pair) -> bool {
        self.nodes_are_eq(pair)
            && if self.is_directed() {
                self.edges_are_eq(pair, Direction::Incoming)
                    && self.edges_are_eq(pair, Direction::Outgoing)
            } else {
                // This will check all neighbors since the graphs are undirected.
                self.edges_are_eq(pair, Direction::Incoming)
            }
    }

    /// Returns `true` if the nodes in the pair
    /// are semantically equivalent.
    fn nodes_are_eq(&self, pair: Pair) -> bool {
        let node_eq = match &self.node_eq {
            None => return true,
            Some(node_eq) => node_eq,
        };
        node_eq(
            self.query.node_label(pair.query_node),
            self.data.node_label(pair.data_node),
        )
    }

    /// Returns `true` if the pair edges in `direction`
    /// are semantically equivalent.
    fn edges_are_eq(&self, pair: Pair, direction: Direction) -> bool {
        let edge_eq = match &self.edge_eq {
            None => return true,
            Some(edge_eq) => edge_eq,
        };
        let source_target = |node, neighbor| match direction {
            Direction::Outgoing => (node, neighbor),
            Direction::Incoming => (neighbor, node),
        };
        // If the graph is undirected, this returns all neighbors.
        for neighbor in self
            .query
            .graph
            .neighbors(pair.query_node, direction)
            .filter(|&neighbor| self.query.is_covered(neighbor))
        {
            let (query_source, query_target) = source_target(pair.query_node, neighbor);
            let mapped = self.query.map[neighbor];
            let (data_source, data_target) = source_target(pair.data_node, mapped);
            if !edge_eq(
                self.query.edge_label(query_source, query_target),
                self.data.edge_label(data_source, data_target),
            ) {
                return false;
            }
        }
        true
    }

    /// Returns a reference to the query partial map.
    pub(crate) fn query_map(&self) -> &Vec<NodeIndex> {
        &self.query.map
    }

    /// Returns the query partial map.
    pub(crate) fn into_query_map(self) -> Vec<NodeIndex> {
        self.query.map
    }

    /// Returns `true` if all query nodes are covered.
    pub(crate) fn all_covered(&self) -> bool {
        self.depth == self.query.map.len()
    }

    /// Returns `true` if the graphs are directed.
    fn is_directed(&self) -> bool {
        self.query.graph.is_directed()
    }
}

#[derive(Clone, Debug)]
struct GraphState<'a, G> {
    /// Graph.
    ///
    /// This is *G_1* or *G_2* in the original VF2 paper.
    graph: &'a G,
    /// A partial map of this graph's node indices to the other's.
    ///
    /// This is *M_1* or *M_2* in the original VF2 paper.
    map: Vec<NodeIndex>,
    /// Outgoing terminal set.
    ///
    /// This is *T^1_out* or *T^2_out* in the original VF2 paper.
    ///
    /// For undirected graphs, this set contains all the terminal
    /// nodes and [`Self::incoming`] is unused.
    ///
    /// A nonzero value at index *n* indicates node *n* is either
    /// in the set or covered by the partial map.
    /// The value is the depth in the SSR tree at which the node was added.
    outgoing: Vec<usize>,
    /// Number of nodes in the outgoing terminal set.
    outgoing_size: usize,
    /// Incoming terminal set.
    ///
    /// This is *T^1_in* or *T^2_in* in the original VF2 paper.
    incoming: Vec<usize>,
    /// Number of nodes in the incoming terminal set.
    incoming_size: usize,
    /// Tracks the order nodes were added to the partial map.
    ///
    /// The value at index `i` is the node that
    /// was added to the partial map at depth `i + 1`.
    node_stack: Vec<NodeIndex>,
}

impl<'a, G> GraphState<'a, G>
where
    G: Graph,
{
    /// Creates a new [`GraphState`].
    fn new(graph: &'a G) -> Self {
        Self {
            graph,
            map: vec![NOT_IN_MAP; graph.node_count()],
            outgoing: vec![NOT_IN_SET; graph.node_count()],
            outgoing_size: 0,
            incoming: vec![NOT_IN_SET; graph.node_count()],
            incoming_size: 0,
            node_stack: vec![0; graph.node_count()],
        }
    }

    /// Returns the first node in `source`.
    fn first_node(&self, source: Source) -> Option<NodeIndex> {
        self.next_node(source, 0)
    }

    /// Returns the next node in `source` beginning at `skip`.
    fn next_node(&self, source: Source, skip: usize) -> Option<NodeIndex> {
        match source {
            Source::Outgoing => self.terminal_nodes(&self.outgoing, skip).next(),
            Source::Incoming => self.terminal_nodes(&self.incoming, skip).next(),
            Source::Uncovered => self.uncovered_nodes(skip).next(),
        }
    }

    /// Returns an iterator of nodes in the terminal set beginning at `skip`.
    fn terminal_nodes(
        &self,
        set: &'a [usize],
        skip: usize,
    ) -> impl Iterator<Item = NodeIndex> + '_ {
        (skip..self.map.len()).filter(|&node| self.in_terminal_set(node, set))
    }

    /// Returns `true` if `node` is in the terminal set.
    fn in_terminal_set(&self, node: NodeIndex, set: &[usize]) -> bool {
        set[node] != NOT_IN_SET && !self.is_covered(node)
    }

    /// Returns an iterator of uncovered nodes beginning at `skip`.
    fn uncovered_nodes(&self, skip: usize) -> impl Iterator<Item = NodeIndex> + '_ {
        (skip..self.map.len()).filter(|&node| !self.is_covered(node))
    }

    /// Pushes a map from `node` to `to_node` to the partial map.
    fn push(&mut self, node: NodeIndex, to_node: NodeIndex, depth: usize) {
        self.node_stack[depth - 1] = node;
        self.map[node] = to_node;
        if self.outgoing[node] != NOT_IN_SET {
            self.outgoing_size -= 1;
        }
        self.push_neighbors(node, Direction::Outgoing, depth);
        if self.graph.is_directed() {
            if self.incoming[node] != NOT_IN_SET {
                self.incoming_size -= 1;
            }
            self.push_neighbors(node, Direction::Incoming, depth);
        }
    }

    /// Pushes neighbors of `node` in `direction` to the corresponding terminal set.
    fn push_neighbors(&mut self, node: NodeIndex, direction: Direction, depth: usize) {
        let (set, len) = match direction {
            Direction::Outgoing => (&mut self.outgoing, &mut self.outgoing_size),
            Direction::Incoming => (&mut self.incoming, &mut self.incoming_size),
        };
        // If the graph is undirected, this returns all neighbors.
        for neighbor in self.graph.neighbors(node, direction) {
            if set[neighbor] == NOT_IN_SET {
                set[neighbor] = depth;
                if self.map[neighbor] == NOT_IN_MAP {
                    *len += 1;
                }
            }
        }
    }

    /// Pops the node at `depth` from the partial map and returns it.
    fn pop(&mut self, depth: usize) -> NodeIndex {
        let node = self.node_stack[depth - 1];
        self.map[node] = NOT_IN_MAP;
        if self.outgoing[node] != NOT_IN_SET {
            self.outgoing_size += 1;
        }
        self.pop_neighbors(node, Direction::Outgoing, depth);
        if self.graph.is_directed() {
            if self.incoming[node] != NOT_IN_SET {
                self.incoming_size += 1;
            }
            self.pop_neighbors(node, Direction::Incoming, depth);
        }
        node
    }

    /// Pops neighbors of `node` in `direction` from the corresponding
    /// terminal set if they were added at `depth`.
    fn pop_neighbors(&mut self, node: NodeIndex, direction: Direction, depth: usize) {
        let (set, len) = match direction {
            Direction::Outgoing => (&mut self.outgoing, &mut self.outgoing_size),
            Direction::Incoming => (&mut self.incoming, &mut self.incoming_size),
        };
        // If the graph is undirected, this returns all neighbors.
        for neighbor in self.graph.neighbors(node, direction) {
            if set[neighbor] == depth {
                set[neighbor] = NOT_IN_SET;
                if self.map[neighbor] == NOT_IN_MAP {
                    *len -= 1;
                }
            }
        }
    }

    /// Returns `true` if `node` is covered by the partial map.
    fn is_covered(&self, node: NodeIndex) -> bool {
        self.map[node] != NOT_IN_MAP
    }

    /// Returns the label of `node`.
    fn node_label(&self, node: NodeIndex) -> &G::NodeLabel {
        self.graph.node_label(node).expect("node should exist")
    }

    /// Returns the label of `node`.
    ///
    /// Has the same behaviour as [`Graph::edge_label`].
    fn edge_label(&self, source: NodeIndex, target: NodeIndex) -> &G::EdgeLabel {
        self.graph
            .edge_label(source, target)
            .expect("edge should exist")
    }
}

/// Candidate pair source.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Source {
    /// Uncovered neighbors of covered nodes that are edge destinations.
    ///
    /// This is *T_out* in the original VF2 paper.
    Outgoing,
    /// Uncovered neighbors of covered nodes that are edge sources.
    ///
    /// This is *T_in* in the original VF2 paper.
    Incoming,
    /// Uncovered nodes.
    ///
    /// This is *P^d* in the original VF2 paper.
    Uncovered,
}

/// A pair of query and data node indices.
#[derive(Copy, Clone, Debug)]
struct Pair {
    query_node: NodeIndex,
    data_node: NodeIndex,
}

impl Pair {
    fn new(query_node: NodeIndex, data_node: NodeIndex) -> Self {
        Self {
            query_node,
            data_node,
        }
    }
}
