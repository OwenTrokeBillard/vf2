# `vf2` — VF2 subgraph isomorphism algorithm in Rust

This crate implements the VF2 subgraph isomorphism algorithm [1].
It can find
[graph isomorphisms](https://en.wikipedia.org/wiki/Graph_isomorphism),
[subgraph isomorphisms](https://en.wikipedia.org/wiki/Subgraph_isomorphism_problem),
and [induced subgraph isomorphisms](https://en.wikipedia.org/wiki/Induced_subgraph_isomorphism_problem).
Graphs can be directed or undirected.

# Features

- [x] Enumerate graph isomorphisms
- [x] Enumerate subgraph isomorphisms
- [x] Enumerate induced subgraph isomorphisms
- [x] Support directed graphs
- [x] Support undirected graphs
- [x] Support disconnected graphs
- [x] Support node labels
- [x] Support edge labels
- [x] Graph trait

# What is subgraph isomorphism?

A graph is a structure consisting of a set of objects where some pairs of objects are connected. A graph isomorphism is
a one-to-one correspondence between two graphs such that objects connected in one are also connected in the other.

### Graph isomorphism

For two graphs to be isomorphic, there must be a one-to-one correspondence between nodes such that neighbors in one are
also neighbors in the other. The query and data graphs in the following image are isomorphic.

![graph-isomorphism.svg](/images/graph-isomorphism.svg)

### Subgraph isomorphism

It is often desirable to find instances of one graph within another. To do this, we search for subgraph isomorphisms. A
subgraph isomorphism is when one graph is isomorphic to a subgraph of another. There are two subgraph isomorphisms in
the following image.

![subgraph-isomorphism.svg](/images/subgraph-isomorphism.svg)

### Induced subgraph isomorphism

An induced subgraph isomorphism is the same as a subgraph isomorphism except that the subgraph must be induced, meaning
the query and data graphs must have the same edges.

![induced-subgraph-isomorphism.svg](/images/induced-subgraph-isomorphism.svg)

# Remaining work

- [ ] Implement VF2 cutting rules
- [ ] Implement VF2++ (only VF2 implemented so far)

# References

[1] L. P. Cordella, P. Foggia, C. Sansone, and M. Vento,
“A (sub)graph isomorphism algorithm for matching large graphs,”
IEEE Transactions on Pattern Analysis and Machine Intelligence, vol. 26, no. 10, pp. 1367–1372,
Oct. 2004, doi: https://doi.org/10.1109/tpami.2004.75.

[2] A. Jüttner and P. Madarasi,
“VF2++—An improved subgraph isomorphism algorithm,”
Discrete Applied Mathematics, vol. 242, pp. 69–81,
Jun. 2018, doi: https://doi.org/10.1016/j.dam.2018.02.018.
