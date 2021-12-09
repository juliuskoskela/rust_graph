/// Icludes

use std::{
    fmt::{Debug, Display},
    hash::Hash,
	sync::Arc
};

use crate::path::*;
use crate::edge::*;
use crate::global::*;
use crate::node::*;
use crate::traverse::*;

/// Digraph

pub struct Digraph<K, N = Empty, E = Empty>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    pub nodes: RefNodePool<K, N, E>,
	edge_count: usize,
}

/// Digraph: Implementations

impl<K, N, E> Default for Digraph<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, N, E> Digraph<K, N, E>
where
    K: Hash + Eq + Clone + Debug + Display + Sync + Send,
    N: Clone + Debug + Display + Sync + Send,
    E: Clone + Debug + Display + Sync + Send,
{
    pub fn new() -> Self {
        Self {
            nodes: RefNodePool::new(),
			edge_count: 0,
        }
    }

	pub fn insert(&mut self, key: K, data: N) -> bool {
        if self.nodes.contains_key(&key) {
            let node = self.nodes[&key].clone();
            node.store(data);
            false
        } else {
            let node = RefNode::new(Node::new(key.clone(), data));
            self.nodes.insert(key, node);
            true
        }
    }

	pub fn insert_node(&mut self, node: Node<K, N, E>) -> bool {
        if self.nodes.contains_key(&node.key()) {
            let node = self.nodes[&node.key()].clone();
            node.store(node.load());
            false
        } else {
            self.nodes.insert(node.key().clone(), Arc::new(node.clone()));
            true
        }
    }

    pub fn connect(&mut self, source: &K, target: &K, data: E) {
        if self.nodes.contains_key(source) && self.nodes.contains_key(target) {
            if connect(&self.nodes[source], &self.nodes[target], data) {
				self.edge_count += 1;
			}
        }
    }

	pub fn disconnect(&mut self, source: &K, target: &K) {
        if self.nodes.contains_key(source) && self.nodes.contains_key(target) {
            if disconnect(&self.nodes[source], &self.nodes[target]) {
				self.edge_count -= 1;
			}
        }
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

	pub fn edge_count(&self) -> usize {
		self.edge_count
    }

	pub fn bytesize(&self) -> usize {
		let node_size = std::mem::size_of::<Node<K, N, E>>();
		let edge_size = std::mem::size_of::<Edge<K, N, E>>();
		(self.node_count() * node_size) + (self.edge_count() * edge_size)
	}

	pub fn print(&self) {
		for n in self.nodes.iter() {
			println!("{}", n.1);
		}
	}

	pub fn node(&self, key: &K) -> Option<&RefNode<K, N, E>> {
		self.nodes.get(key)
	}

	pub fn edge(&self, source: &K, target: &K) -> Option<RefEdge<K, N, E>> {
		let s = self.nodes.get(source);
		let t = self.nodes.get(target);
		match s {
            Some(ss) => match t {
                Some(tt) => ss.outbound().find(ss, tt),
                None => None,
            },
            None => None,
        }
	}

    pub fn get_leaves(&self) -> Vec<RefNode<K, N, E>> {
        let mut res: Vec<RefNode<K, N, E>> = vec![];
        for n in self.nodes.values() {
            if n.outbound().is_empty() {
                res.push(n.clone());
            }
        }
        res
    }

	pub fn depth_first<F>(
		&self,
		source: &K,
		f: F
	) -> Option<Path<K, N, E>>
	where
		F: Fn (&RefEdge<K, N, E>) -> Traverse,
	{
		let s = self.node(source);
		match s {
    	    Some(ss) => depth_traversal_directed(ss, f),
    	    None => None,
    	}
	}

	pub fn breadth_first<F>(
		&self,
		source: &K,
		f: F
	) -> Option<EdgeList<K, N, E>>
	where
		F: Fn (&RefEdge<K, N, E>) -> Traverse + std::marker::Sync + std::marker::Send + std::marker::Copy,
	{
		let s = self.node(source);
		match s {
    	    Some(ss) => directed_breadth_first_traversal(ss, f),
    	    None => None,
    	}
	}

	pub fn par_breadth_first<F>(
		&self,
		source: &K,
		f: F
	) -> Option<EdgeList<K, N, E>>
	where
		F: Fn (&RefEdge<K, N, E>) -> Traverse + std::marker::Sync + std::marker::Send + std::marker::Copy,
	{
		let s = self.node(source);
		match s {
			Some(ss) => {
				let ret = parallel_directed_breadth_first_traversal(ss, f);
				// self.clear_locks();
				ret
			},
    	    None => None,
    	}
	}

	pub fn print_nodes(&self) {
		for (_, node) in self.nodes.iter() {
			println!("{}", node);
		}
	}

	pub fn print_edges(&self) {
		for (_, node) in self.nodes.iter() {
			for edge in node.outbound().iter() {
				println!("{}", edge);
			}
		}
	}

	pub fn clear_locks(&self) {
		for (_, node) in self.nodes.iter() {
			node.open();
			for edge in node.outbound().iter() {
				edge.open();
			}
		}
	}
}
