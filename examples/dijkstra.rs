use gdsl::*;
use std::cell::Cell;

fn dijkstra_1() {
	use gdsl::*;
	// use std::cell::Cell;

	// We create a directed graph using the `graph!` macro. In the macro
	// invocation we specify the type of the nodes and the type of the edges
	// by specifying the type-signature `(NodeKey, NodeValue) => [EdgeValue]`.
	//
	// The `NodeKey` type is used to identify the nodes in the graph. The
	// `NodeValue` type is used to store the value of the node. The `EdgeValue`
	// type is used to store the value of the edge.
	//
	// The macro also specifies if the graph is directed or undirected. In this
	// case it is directed. If we want to create an undirected graph we have to
	// use the `:` operator instead of the `=>` operator. The macro returns
	// either a `DiGraph` or `UnGraph` type respectively.
	//
	// In this example the node stores the distance to the source node of the
	// search. The edge stores the weight of the edge. The distance is wrapped
	// in a `Cell` to allow for mutable access. We initialize the distance to
	// `std::u64::MAX` to indicate that the node is not part of the shortest
	// path.
	let g = graph![
		(char, Cell<u64>) => [u64]
		('A', Cell::new(u64::MAX)) => [ ('B', 4), ('H', 8) ]
		('B', Cell::new(u64::MAX)) => [ ('A', 4), ('H', 11), ('C', 8) ]
		('C', Cell::new(u64::MAX)) => [ ('B', 8), ('C', 2), ('F', 4), ('D', 7) ]
		('D', Cell::new(u64::MAX)) => [ ('C', 7), ('F', 14), ('E', 9) ]
		('E', Cell::new(u64::MAX)) => [ ('D', 9), ('F', 10) ]
		('F', Cell::new(u64::MAX)) => [ ('G', 2), ('C', 4), ('D', 14), ('E', 10) ]
		('G', Cell::new(u64::MAX)) => [ ('H', 1), ('I', 6), ('F', 2) ]
		('H', Cell::new(u64::MAX)) => [ ('A', 8), ('B', 11), ('I', 7), ('G', 1) ]
		('I', Cell::new(u64::MAX)) => [ ('H', 7), ('C', 2), ('G', 6) ]
	];

	// In order to find the shortest path we need to specify the source node and
	// set its distance to 0.
	g['A'].set(0);

	// In order to perform a dijkstra search we take the source node and call the
	// `pfs_min()` function which returns a search object. A search object is
	// like to an iterator. From the search object we call the `search_map()`
	// function which let's us read each edge in the search and to manipulate
	// the corresponding nodes.
	//
	// The `pfs_min()` function is a "priority first search". As opposed to a
	// breadth-first search which is a "depth first search", the priority first
	// search traverses the nodes in the graph in a priority order. The priority
	// of a node is determined by the node's value and thus has to implement
	// the `Ord` trait. Since `u64` implements the `Ord` trait we can use the
	// distance stored in the node as the priority.
	//
	// The `search_map()` function takes a `target` node and a closure which
	// is called for each edge in the search. The target is optional, in case
	// we want to search the whole graph. In this case the target is `None`,
	// so we will calculate the distance to all nodes.
	g['A'].pfs_min().search_map(None, &|u, v, e| {

		// Since we are using a `Cell` to store the distance we use `get()` to
		// read the distance values.
		let (u_dist, v_dist) = (u.get(), v.get());

		// Now we check if the distance stored in the node `v` is smaller than
		// the distance stored in the node `u` + the length (weight) of the
		// edge `e`. If this is the case we update the distance stored in the
		// node `v`.
		if v_dist > u_dist + e {
			v.set(u_dist + e);
		}
	});

	// We expect that the distance to the node `E` is 21.
	assert!(g['E'].take() == 21);
}

fn dijkstra_undir() {

	let g = graph![
		(&str, Cell<u64>) : [u64]
		("A", Cell::new(u64::MAX)) : [ ("B", 4), ("H", 8) ]
		("B", Cell::new(u64::MAX)) : [ ("A", 4), ("H", 11), ("C", 8) ]
		("C", Cell::new(u64::MAX)) : [ ("B", 8), ("C", 2), ("F", 4), ("D", 7) ]
		("D", Cell::new(u64::MAX)) : [ ("C", 7), ("F", 14), ("E", 9) ]
		("E", Cell::new(u64::MAX)) : [ ("D", 9), ("F", 10) ]
		("F", Cell::new(u64::MAX)) : [ ("G", 2), ("C", 4), ("D", 14), ("E", 10) ]
		("G", Cell::new(u64::MAX)) : [ ("H", 1), ("I", 6), ("F", 2) ]
		("H", Cell::new(u64::MAX)) : [ ("A", 8), ("B", 11), ("I", 7), ("G", 1) ]
		("I", Cell::new(u64::MAX)) : [ ("H", 7), ("C", 2), ("G", 6) ]
	];

	g["A"].set(0);

	g["A"].pfs_min().search_filter_map(Some(&g["E"]), &|u, v, e| {

		let (u_dist, v_dist) = (u.get(), v.get());

		match v_dist > u_dist + e {
			true => {
				v.set(u_dist + e);
				true
			},
			false => false,
		}

	});

	assert!(g["E"].take() == 21);
}

fn dijkstra_2() {
	use gdsl::*;
	use min_max_heap::MinMaxHeap;

	let g = graph![
		(&str, Cell<u64>) => [u64]
		("A", Cell::new(u64::MAX)) => [ ("B", 4), ("H", 8) ]
		("B", Cell::new(u64::MAX)) => [ ("A", 4), ("H", 11), ("C", 8) ]
		("C", Cell::new(u64::MAX)) => [ ("B", 8), ("C", 2), ("F", 4), ("D", 7) ]
		("D", Cell::new(u64::MAX)) => [ ("C", 7), ("F", 14), ("E", 9) ]
		("E", Cell::new(u64::MAX)) => [ ("D", 9), ("F", 10) ]
		("F", Cell::new(u64::MAX)) => [ ("G", 2), ("C", 4), ("D", 14), ("E", 10) ]
		("G", Cell::new(u64::MAX)) => [ ("H", 1), ("I", 6), ("F", 2) ]
		("H", Cell::new(u64::MAX)) => [ ("A", 8), ("B", 11), ("I", 7), ("G", 1) ]
		("I", Cell::new(u64::MAX)) => [ ("H", 7), ("C", 2), ("G", 6) ]
	];

	let mut heap = MinMaxHeap::new();
	let mut visited = std::collections::HashSet::new();

	g["A"].set(0);
	heap.push(g["A"].clone());

	'search: while let Some(u) = heap.pop_min() {
		for (v, delta) in &u {
			let (u_dist, v_dist) = (u.get(), v.get());

			if !visited.contains(v.key()) {
				if v_dist > u_dist + delta {
					visited.insert(v.key().clone());
					v.set(u_dist + delta);
					if v == g["E"] { break 'search }
					heap.push(v.clone());
				}
			}
		}
	}

	assert!(g["E"].take() == 21);
}

fn main() {
	dijkstra_1();
	dijkstra_2();
	dijkstra_undir();
}