//! This crate implements the VF2 subgraph isomorphism algorithm.
//! It can find
//! [graph isomorphisms](https://en.wikipedia.org/wiki/Graph_isomorphism),
//! [subgraph isomorphisms](https://en.wikipedia.org/wiki/Subgraph_isomorphism_problem),
//! and [induced subgraph isomorphisms](https://en.wikipedia.org/wiki/Induced_subgraph_isomorphism_problem).
//! Graphs can be directed or undirected.
//!
//! See the [repository](https://github.com/OwenTrokeBillard/vf2) for more information.
//!
//! # Usage
//!
//! Add `vf2` to your dependencies in **Cargo.toml**.
//!
//! ```toml
//! [dependencies]
//! vf2 = "1.0"
//! ```
//!
//! Create your query and data graphs with [petgraph](https://github.com/petgraph/petgraph)
//! or any library that implements the [`Graph`] trait. Then, call one of the following
//! functions based on the problem type.
//!
//! | Problem type                  | Call                                                                  |
//! |-------------------------------|-----------------------------------------------------------------------|
//! | Graph isomorphisms            | [`vf2::isomorphisms`](isomorphisms)                                   |
//! | Subgraph isomorphisms         | [`vf2::subgraph_isomorphisms`](subgraph_isomorphisms)                 |
//! | Induced subgraph isomorphisms | [`vf2::induced_subgraph_isomorphisms`](induced_subgraph_isomorphisms) |
//!
//! \
//! These return a [`Vf2Builder`] with the algorithm configured.
//! Next, call one of the following on the builder to enumerate the isomorphisms.
//!
//! | Desired output           | Call                         |
//! |--------------------------|------------------------------|
//! | First isomorphism        | [`first`](Vf2Builder::first) |
//! | Vector of isomorphisms   | [`vec`](Vf2Builder::vec)     |
//! | Iterator of isomorphisms | [`iter`](Vf2Builder::iter)   |
//!
//! \
//! Filling a vector can consume a significant amount of memory.
//! Use the iterator to inspect isomorphisms as they are found.
//! For the best performance, call [`next_ref`](IsomorphismIter::next_ref)
//! on the iterator
//! instead of [`next`](IsomorphismIter::next)
//! to avoid cloning each isomorphism.
//!
//! You can configure the node and edge equality functions on the builder
//! with [`node_eq`](Vf2Builder::node_eq) and [`edge_eq`](Vf2Builder::edge_eq),
//! respectively.
//!
//! # Example
//!
//! This example shows how to find subgraph isomorphisms.
//!
//! ```
//! use petgraph::data::{Element, FromElements};
//! use petgraph::graph::DiGraph;
//!
//! // Create query graph.
//! let query = DiGraph::<i32, ()>::from_elements([
//!     Element::Node { weight: 0, },
//!     Element::Node { weight: 1, },
//!     Element::Edge { source: 0, target: 1, weight: () },
//! ]);
//!
//! // Create data graph.
//! let data = DiGraph::<i32, ()>::from_elements([
//!     Element::Node { weight: 0, },
//!     Element::Node { weight: 1, },
//!     Element::Node { weight: 2, },
//!     Element::Edge { source: 0, target: 1, weight: () },
//!     Element::Edge { source: 1, target: 2, weight: () },
//! ]);
//!
//! // Find subgraph isomorphisms.
//! let isomorphisms = vf2::subgraph_isomorphisms(&query, &data).vec();
//! assert_eq!(isomorphisms, vec![vec![0, 1], vec![1, 2]]);
//! ```

mod builder;
mod graph;
mod isomorphism;
mod iter;
#[cfg(feature = "petgraph")]
mod petgraph;
mod state;

pub use builder::*;
pub use graph::*;
pub use isomorphism::*;
pub use iter::*;
