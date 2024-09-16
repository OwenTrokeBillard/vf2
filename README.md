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

# Remaining work

- [ ] Implement VF2 cutting rules

# References

[1] L. P. Cordella, P. Foggia, C. Sansone, and M. Vento,
“A (sub)graph isomorphism algorithm for matching large graphs,”
IEEE Transactions on Pattern Analysis and Machine Intelligence, vol. 26, no. 10, pp. 1367–1372,
Oct. 2004, doi: https://doi.org/10.1109/tpami.2004.75.

[2] A. Jüttner and P. Madarasi,
“VF2++—An improved subgraph isomorphism algorithm,”
Discrete Applied Mathematics, vol. 242, pp. 69–81,
Jun. 2018, doi: https://doi.org/10.1016/j.dam.2018.02.018.
