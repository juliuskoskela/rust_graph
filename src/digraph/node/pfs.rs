//==== Includes ===============================================================

use std::{
    fmt::Display,
    hash::Hash,
	collections::HashSet
};

use min_max_heap::MinMaxHeap;

use crate::digraph::*;
use crate::digraph::node::method::*;

enum Priority {
	Min,
	Max
}

pub struct DiPFS<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone,
	E: Clone,
{
	root: DiNode<K, N, E>,
	target: Option<&'a K>,
	method: Method<'a, K, N, E>,
	transpose: Direction,
	priority: Priority,
}

impl<'a, K, N, E> DiPFS<'a, K, N, E>
where
	K: Clone + Hash + Display + PartialEq + Eq,
	N: Clone + Ord,
	E: Clone,
{
	pub fn new(root: &DiNode<K, N, E>) -> Self {
		DiPFS {
			root: root.clone(),
			target: None,
			method: Method::NullMethod,
			transpose: Direction::Forward,
			priority: Priority::Min,
		}
	}

	pub fn min(mut self) -> Self {
		self.priority = Priority::Min;
		self
	}

	pub fn max(mut self) -> Self {
		self.priority = Priority::Max;
		self
	}

	pub fn target(mut self, target: &'a K) -> Self {
		self.target = Some(target);
		self
	}

	pub fn transpose(mut self) -> Self {
		self.transpose = Direction::Backward;
		self
	}

	pub fn map(mut self, f: Map<'a, K, N, E>) -> Self {
		self.method = Method::Map(f);
		self
	}

	pub fn filter(mut self, f: Filter<'a, K, N, E>) -> Self {
		self.method = Method::Filter(f);
		self
	}


	pub fn filter_map(mut self, f: FilterMap<'a, K, N, E>) -> Self {
		self.method = Method::FilterMap(f);
		self
	}

	fn forward_min(
		&self,
		result: &mut Vec<DiEdge<K, N, E>>,
		visited: &mut HashSet<K>,
		queue: &mut MinMaxHeap<DiNode<K, N, E>>,
	) -> bool {
		while let Some(node) = queue.pop_min() {
			for (u, v, e) in node.iter_out() {
				if !visited.contains(v.key()) {
					if self.method.exec(&u, &v, &e) {
						visited.insert(v.key().clone());
						result.push((u, v.clone(), e));
						if self.target.is_some() && self.target.unwrap() == v.key() {
							return true;
						}
						queue.push(v.clone());
					}
				}
			}
		}
		false
	}

	fn backward_min(
		&self,
		result: &mut Vec<DiEdge<K, N, E>>,
		visited: &mut HashSet<K>,
		queue: &mut MinMaxHeap<DiNode<K, N, E>>,
	) -> bool {
		while let Some(node) = queue.pop_min() {
			for (v, u, e) in node.iter_in() {
				if !visited.contains(v.key()) {
					if self.method.exec(&u, &v, &e) {
						visited.insert(v.key().clone());
						result.push((u, v.clone(), e));
						if self.target.is_some() && self.target.unwrap() == v.key() {
							return true;
						}
						queue.push(v.clone());
					}
				}
			}
		}
		false
	}

	fn forward_max(
		&self,
		result: &mut Vec<DiEdge<K, N, E>>,
		visited: &mut HashSet<K>,
		queue: &mut MinMaxHeap<DiNode<K, N, E>>,
	) -> bool {
		while let Some(node) = queue.pop_max() {
			for (u, v, e) in node.iter_out() {
				if !visited.contains(v.key()) {
					if self.method.exec(&u, &v, &e) {
						visited.insert(v.key().clone());
						result.push((u, v.clone(), e));
						if self.target.is_some() && self.target.unwrap() == v.key() {
							return true;
						}
						queue.push(v.clone());
					}
				}
			}
		}
		false
	}

	fn backward_max(
		&self,
		result: &mut Vec<DiEdge<K, N, E>>,
		visited: &mut HashSet<K>,
		queue: &mut MinMaxHeap<DiNode<K, N, E>>,
	) -> bool {
		while let Some(node) = queue.pop_max() {
			for (v, u, e) in node.iter_in() {
				if !visited.contains(v.key()) {
					if self.method.exec(&u, &v, &e) {
						visited.insert(v.key().clone());
						result.push((u, v.clone(), e));
						if self.target.is_some() && self.target.unwrap() == v.key() {
							return true;
						}
						queue.push(v.clone());
					}
				}
			}
		}
		false
	}

	pub fn find(&mut self) -> Option<DiNode<K, N, E>> {
		let path = self.path_nodes();
		match path {
			Some(path) => Some(path.last().unwrap().clone()),
			None => None,
		}
	}

	pub fn cycle(&'a mut self) -> Option<Vec<DiNode<K, N, E>>> {
		let mut result = vec![];
		let mut edges = vec![];
		let mut queue = MinMaxHeap::new();
		let mut visited = HashSet::new();
		let target_found;

		self.target = Some(self.root.key());
		queue.push(self.root.clone());

		match self.transpose {
			Direction::Forward => {
				match self.priority {
					Priority::Min => {
						target_found = self.forward_min(&mut edges, &mut visited, &mut queue);
					}
					Priority::Max => {
						target_found = self.forward_max(&mut edges, &mut visited, &mut queue);
					}
				}
			}
			Direction::Backward => {
				match self.priority {
					Priority::Min => {
						target_found = self.backward_min(&mut edges, &mut visited, &mut queue);
					}
					Priority::Max => {
						target_found = self.backward_max(&mut edges, &mut visited, &mut queue);
					}
				}
			}
		}
		if target_found {
			let mut nodes = edges.iter().map(|(_, v, _)| v.clone()).collect();
			result.append(&mut nodes);
			return Some(result);
		}
		None
	}

	pub fn path_edges(&mut self) -> Option<Vec<DiEdge<K, N, E>>> {
		let mut edges = vec![];
		let mut queue = MinMaxHeap::new();
		let mut visited = HashSet::new();
		let target_found;

		queue.push(self.root.clone());
		visited.insert(self.root.key().clone());

		match self.transpose {
			Direction::Forward => {
				match self.priority {
					Priority::Min => {
						target_found = self.forward_min(&mut edges, &mut visited, &mut queue);
					}
					Priority::Max => {
						target_found = self.forward_max(&mut edges, &mut visited, &mut queue);
					}
				}
			}
			Direction::Backward => {
				match self.priority {
					Priority::Min => {
						target_found = self.backward_min(&mut edges, &mut visited, &mut queue);
					}
					Priority::Max => {
						target_found = self.backward_max(&mut edges, &mut visited, &mut queue);
					}
				}
			}
		}
		if target_found {
			return Some(edges);
		}
		None
	}

	pub fn path_nodes(&mut self) -> Option<Vec<DiNode<K, N, E>>> {
		let edges = self.path_edges();
		match edges {
			Some(edges) => {
				let mut nodes = vec![self.root.clone()];
				for (_, v, _) in edges {
					nodes.push(v.clone());
				}
				return Some(nodes);
			}
			None => {
				return None;
			}
		}
	}
}