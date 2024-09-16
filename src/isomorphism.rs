use crate::NodeIndex;

/// An isomorphism mapping query nodes to data nodes.
///
/// The value at index `i` is the data node index
/// that query node index `i` maps to.
pub type Isomorphism = Vec<NodeIndex>;
