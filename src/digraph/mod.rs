//! Directed Graph

//==== Submodules =============================================================

pub mod node;
pub mod graph;

//==== Includes ===============================================================

pub use crate::digraph::node::*;
pub use crate::digraph::graph::*;
pub use crate::Empty;
pub use crate::{graph, connect, dinode};

pub type DiEdge<K, N, E> = (DiNode<K, N, E>, DiNode<K, N, E>, E);
