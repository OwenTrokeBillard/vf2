use petgraph::data::{Element, FromElements};
use petgraph::graph::{DiGraph, UnGraph};
use petgraph::{Directed, EdgeType, Graph, Undirected};

/// Tests graph isomorphism enumeration on directed graphs.
#[test]
fn isomorphisms_directed() {
    let query = DiGraph::<(), ()>::from_edges([(0, 2), (1, 2), (2, 3)]);
    let data = DiGraph::<(), ()>::from_edges([(0, 2), (1, 2), (2, 3)]);

    let isomorphisms = vf2::isomorphisms(&query, &data).vec();

    assert_eq!(isomorphisms, vec![vec![0, 1, 2, 3], vec![1, 0, 2, 3]]);
}

/// Tests graph isomorphism enumeration on undirected graphs.
#[test]
fn isomorphisms_undirected() {
    let query = UnGraph::<(), ()>::from_edges([(0, 2), (1, 2), (2, 3)]);
    let data = UnGraph::<(), ()>::from_edges([(0, 2), (1, 2), (2, 3)]);

    let isomorphisms = vf2::isomorphisms(&query, &data).vec();

    assert_eq!(
        isomorphisms,
        vec![
            vec![0, 1, 2, 3],
            vec![0, 3, 2, 1],
            vec![1, 0, 2, 3],
            vec![1, 3, 2, 0],
            vec![3, 0, 2, 1],
            vec![3, 1, 2, 0],
        ]
    );
}

/// Tests subgraph isomorphism enumeration on directed graphs.
#[test]
fn subgraph_isomorphisms_directed() {
    let (query, data) = small_graphs::<Directed>();

    let isomorphisms = vf2::subgraph_isomorphisms(&query, &data).vec();

    assert_eq!(
        isomorphisms,
        vec![
            vec![0, 1, 3, 4, 5],
            vec![0, 2, 3, 4, 5],
            vec![1, 0, 3, 4, 5],
            vec![1, 2, 3, 4, 5],
            vec![2, 0, 3, 4, 5],
            vec![2, 1, 3, 4, 5],
        ]
    );
}

/// Tests subgraph isomorphism enumeration on undirected graphs.
#[test]
fn subgraph_isomorphisms_undirected() {
    let (query, data) = small_graphs::<Undirected>();

    let isomorphisms = vf2::subgraph_isomorphisms(&query, &data).vec();

    assert_eq!(
        isomorphisms,
        vec![
            vec![0, 1, 3, 4, 5],
            vec![0, 1, 3, 6, 7],
            vec![0, 2, 3, 4, 5],
            vec![0, 2, 3, 6, 7],
            vec![0, 4, 3, 1, 2],
            vec![0, 4, 3, 2, 1],
            vec![0, 4, 3, 6, 7],
            vec![0, 6, 3, 1, 2],
            vec![0, 6, 3, 2, 1],
            vec![0, 6, 3, 4, 5],
            vec![1, 0, 3, 4, 5],
            vec![1, 0, 3, 6, 7],
            vec![1, 2, 3, 4, 5],
            vec![1, 2, 3, 6, 7],
            vec![1, 4, 3, 6, 7],
            vec![1, 6, 3, 4, 5],
            vec![2, 0, 3, 4, 5],
            vec![2, 0, 3, 6, 7],
            vec![2, 1, 3, 4, 5],
            vec![2, 1, 3, 6, 7],
            vec![2, 4, 3, 6, 7],
            vec![2, 6, 3, 4, 5],
            vec![4, 0, 3, 1, 2],
            vec![4, 0, 3, 2, 1],
            vec![4, 0, 3, 6, 7],
            vec![4, 1, 3, 6, 7],
            vec![4, 2, 3, 6, 7],
            vec![4, 6, 3, 1, 2],
            vec![4, 6, 3, 2, 1],
            vec![6, 0, 3, 1, 2],
            vec![6, 0, 3, 2, 1],
            vec![6, 0, 3, 4, 5],
            vec![6, 1, 3, 4, 5],
            vec![6, 2, 3, 4, 5],
            vec![6, 4, 3, 1, 2],
            vec![6, 4, 3, 2, 1],
        ]
    );
}

/// Tests induced subgraph isomorphism enumeration on directed graphs.
#[test]
fn induced_subgraph_isomorphisms_directed() {
    let (query, data) = small_graphs::<Directed>();

    let isomorphisms = vf2::induced_subgraph_isomorphisms(&query, &data).vec();

    assert_eq!(
        isomorphisms,
        vec![
            vec![0, 1, 3, 4, 5],
            vec![0, 2, 3, 4, 5],
            vec![1, 0, 3, 4, 5],
            vec![2, 0, 3, 4, 5],
        ]
    );
}

/// Tests induced subgraph isomorphism enumeration on undirected graphs.
#[test]
fn induced_subgraph_isomorphisms_undirected() {
    let (query, data) = small_graphs::<Undirected>();

    let isomorphisms = vf2::induced_subgraph_isomorphisms(&query, &data).vec();

    assert_eq!(
        isomorphisms,
        vec![
            vec![0, 1, 3, 4, 5],
            vec![0, 1, 3, 6, 7],
            vec![0, 2, 3, 4, 5],
            vec![0, 2, 3, 6, 7],
            vec![0, 4, 3, 6, 7],
            vec![0, 6, 3, 4, 5],
            vec![1, 0, 3, 4, 5],
            vec![1, 0, 3, 6, 7],
            vec![1, 4, 3, 6, 7],
            vec![1, 6, 3, 4, 5],
            vec![2, 0, 3, 4, 5],
            vec![2, 0, 3, 6, 7],
            vec![2, 4, 3, 6, 7],
            vec![2, 6, 3, 4, 5],
            vec![4, 0, 3, 6, 7],
            vec![4, 1, 3, 6, 7],
            vec![4, 2, 3, 6, 7],
            vec![6, 0, 3, 4, 5],
            vec![6, 1, 3, 4, 5],
            vec![6, 2, 3, 4, 5],
        ]
    );
}

/// Tests that node and edge labels are not compared by default.
#[test]
fn no_eq_by_default() {
    let (query, data) = small_labeled_graphs::<Directed>();

    let isomorphisms = vf2::induced_subgraph_isomorphisms(&query, &data).vec();

    assert_eq!(
        isomorphisms,
        vec![
            vec![0, 1, 3, 4, 5],
            vec![0, 2, 3, 4, 5],
            vec![1, 0, 3, 4, 5],
            vec![2, 0, 3, 4, 5],
        ]
    );
}

/// Tests default equality functions on directed graphs.
#[test]
fn default_eq_directed() {
    let (query, data) = small_labeled_graphs::<Directed>();

    let isomorphisms = vf2::induced_subgraph_isomorphisms(&query, &data)
        .default_eq()
        .vec();

    assert_eq!(isomorphisms, vec![vec![0, 2, 3, 4, 5]]);
}

/// Tests default equality functions on undirected graphs.
#[test]
fn default_eq_undirected() {
    let (query, data) = small_labeled_graphs::<Undirected>();

    let isomorphisms = vf2::induced_subgraph_isomorphisms(&query, &data)
        .default_eq()
        .vec();

    assert_eq!(
        isomorphisms,
        vec![
            vec![0, 2, 3, 4, 5],
            vec![0, 2, 3, 6, 7],
            vec![4, 2, 3, 6, 7],
            vec![6, 2, 3, 4, 5],
        ]
    );
}

/// Tests custom equality functions.
#[test]
fn custom_eq() {
    let (query, data) = small_labeled_graphs::<Directed>();

    let isomorphisms = vf2::induced_subgraph_isomorphisms(&query, &data)
        .node_eq(|left, right| left == right)
        .edge_eq(|left, right| left == right)
        .vec();

    assert_eq!(isomorphisms, vec![vec![0, 2, 3, 4, 5]]);
}

/// Tests enumeration on disconnected graphs.
#[test]
fn disconnected() {
    let query = DiGraph::<(), ()>::from_edges([(0, 1), (2, 3)]);
    let data = DiGraph::<(), ()>::from_edges([(0, 1), (1, 2), (3, 4)]);

    let isomorphisms = vf2::subgraph_isomorphisms(&query, &data).vec();

    assert_eq!(
        isomorphisms,
        vec![
            vec![0, 1, 3, 4],
            vec![1, 2, 3, 4],
            vec![3, 4, 0, 1],
            vec![3, 4, 1, 2],
        ]
    );
}

/// Tests that an empty query results in a panic.
#[test]
#[should_panic]
fn empty_query() {
    let query = DiGraph::<(), ()>::new();
    let data = DiGraph::<(), ()>::from_edges([(0, 1), (1, 2)]);

    // Should panic since query is empty.
    vf2::induced_subgraph_isomorphisms(&query, &data).vec();
}

/// Tests that query and data graphs must be the same size
/// when finding graph isomorphisms.
#[test]
#[should_panic]
fn isomorphisms_same_size() {
    let query = DiGraph::<(), ()>::from_edges([(0, 1)]);
    let data = DiGraph::<(), ()>::from_edges([(0, 1), (1, 2)]);

    // Should panic since query and data are not the same size.
    vf2::isomorphisms(&query, &data).vec();
}

/// Tests that [`Debug`] is implemented for [`Vf2ppBuilder`].
///
/// [`Vf2ppBuilder`]: vf2::Vf2ppBuilder
#[test]
fn builder_debug() {
    let (query, data) = small_graphs::<Directed>();
    let builder = vf2::subgraph_isomorphisms(&query, &data);

    // This should not panic due to missing debug implementation.
    let debug = format!("{builder:#?}");

    assert!(!debug.is_empty());
}

/// Tests that [`Debug`] is implemented for [`IsomorphismIter`].
///
/// [`IsomorphismIter`]: vf2::IsomorphismIter
#[test]
fn iter_debug() {
    let (query, data) = small_graphs::<Directed>();
    let iter = vf2::subgraph_isomorphisms(&query, &data).iter();

    // This should not panic due to missing debug implementation.
    let debug = format!("{iter:#?}");

    assert!(!debug.is_empty());
}

/// Tests finding only the first isomorphism.
#[test]
fn first() {
    let (query, data) = small_graphs::<Directed>();

    let first = vf2::subgraph_isomorphisms(&query, &data).first();

    assert_eq!(first, Some(vec![0, 1, 3, 4, 5]));
}

/// Tests collecting isomorphisms into a vector.
#[test]
fn vec() {
    let (query, data) = small_graphs::<Directed>();

    let vec = vf2::subgraph_isomorphisms(&query, &data).vec();

    assert!(!vec.is_empty());
}

/// Tests getting an iterator of isomorphisms.
#[test]
fn iter() {
    let (query, data) = small_graphs::<Directed>();

    let mut iter = vf2::subgraph_isomorphisms(&query, &data).iter();

    assert!(iter.next().is_some());
}

/// Tests getting a reference to the next isomorphism.
#[test]
fn iter_next_ref() {
    let (query, data) = small_graphs::<Directed>();
    let mut iter = vf2::subgraph_isomorphisms(&query, &data).iter();

    let next_ref = iter.next_ref();

    assert_eq!(next_ref, Some(&vec![0, 1, 3, 4, 5]));
}

/// Tests converting the iterator into the next isomorphism.
#[test]
fn iter_into_next() {
    let (query, data) = small_graphs::<Directed>();
    let iter = vf2::subgraph_isomorphisms(&query, &data).iter();

    let next = iter.into_next();

    assert_eq!(next, Some(vec![0, 1, 3, 4, 5]));
}

/// Returns small query and data graphs used across tests.
fn small_graphs<D: EdgeType>() -> (Graph<(), (), D>, Graph<(), (), D>) {
    let query = Graph::<(), (), D>::from_edges([(0, 2), (1, 2), (2, 3), (3, 4)]);
    let data = Graph::<(), (), D>::from_edges([
        (0, 3),
        (1, 3),
        (2, 3),
        (1, 2),
        (3, 4),
        (4, 5),
        (3, 6),
        (7, 6),
    ]);
    (query, data)
}

/// Returns small query and data graphs,
/// with node and edge labels, used across tests.
#[rustfmt::skip]
fn small_labeled_graphs<D: EdgeType>() -> (Graph<Color, Color, D>, Graph<Color, Color, D>) {
    let query = Graph::<Color, Color, D>::from_elements([
        Element::Node { weight: Color::Black, },
        Element::Node { weight: Color::White, },
        Element::Node { weight: Color::White, },
        Element::Node { weight: Color::Black, },
        Element::Node { weight: Color::White, },
        Element::Edge { source: 0, target: 2, weight: Color::White },
        Element::Edge { source: 1, target: 2, weight: Color::Black },
        Element::Edge { source: 2, target: 3, weight: Color::White },
        Element::Edge { source: 3, target: 4, weight: Color::Black },
    ]);
    let data = Graph::<Color, Color, D>::from_elements([
        Element::Node { weight: Color::Black },
        Element::Node { weight: Color::White },
        Element::Node { weight: Color::White },
        Element::Node { weight: Color::White },
        Element::Node { weight: Color::Black },
        Element::Node { weight: Color::White },
        Element::Node { weight: Color::Black },
        Element::Node { weight: Color::White },
        Element::Edge { source: 0, target: 3, weight: Color::White },
        Element::Edge { source: 1, target: 3, weight: Color::White },
        Element::Edge { source: 2, target: 3, weight: Color::Black },
        Element::Edge { source: 1, target: 2, weight: Color::White },
        Element::Edge { source: 3, target: 4, weight: Color::White },
        Element::Edge { source: 4, target: 5, weight: Color::Black },
        Element::Edge { source: 3, target: 6, weight: Color::White },
        Element::Edge { source: 7, target: 6, weight: Color::Black },
    ]);
    (query, data)
}

/// A color enum used as node and edge labels.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Color {
    White,
    Black,
}
