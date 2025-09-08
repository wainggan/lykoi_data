/*!
a "[directed acyclic graph](https://en.wikipedia.org/wiki/Directed_acyclic_graph)".

this module provides the [`Dag`] struct, which represents a graph `Dag<T>` that can be
constructed into an ordered `Vec<T>` such that any point that had an edge coming into
it will be placed *after* the point the edge started from.

see the [documentation]([Dag]) for more info.
*/

#[derive(Debug)]
pub enum DagError {
	Incomplete,
}
impl std::error::Error for DagError {}
impl std::fmt::Display for DagError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", match self {
			DagError::Incomplete => "Graph was incomplete",
		})
	}
}

/// represents a point. obtained from [`Dag::point()`], to be used with [`Dag::depend()`].
pub struct Index(usize);

/**
a "[directed acyclic graph](https://en.wikipedia.org/wiki/Directed_acyclic_graph)".

essentially, you provide the points and oriented edges (edges that "start" at
a point and "end" at another), and this structure will build a sorted vector, such that
any point that has an edge coming into it will come after where that edge started.

```
# use lykoi_data::dag::Dag;
let mut dag = Dag::new();

let a = dag.point("a");
let b = dag.point("b");
let c = dag.point("c");

dag.depend(&a, &b); // `a` depends on `b`
dag.depend(&b, &c); // `b` depends on `c`

let order = dag.build().unwrap();

// "c" must come first, as "b" depended on "c"
assert_eq!(order, vec!["c", "b", "a"]);
```

this is useful for representing a complex chain of dependencies,
such as for complex shader passes!
*/
#[derive(Debug, Clone)]
pub struct Dag<T> {
	points: Vec<T>,
	edges: Vec<(usize, usize)>,
}
impl<T: Clone> Dag<T> {
	/// constructs a new `Dag<T>`.
	pub fn new() -> Self {
		Self {
			points: Vec::new(),
			edges: Vec::new(),
		}
	}

	/**
	creates a new point, and returns an [`Index`] to be used with [`Self::depend()`].
	
	```
	# use lykoi_data::dag::{Dag, Index};
	# let mut dag = Dag::new();
	let one: Index = dag.point("1");
	let two: Index = dag.point("2");
	```
	*/
	#[must_use]
	pub fn point(&mut self, value: T) -> Index {
		self.points.push(value);
		Index(self.points.len() - 1)
	}

	/**
	sets up an edge from `value` to `needs`. ie: "value X needs Y"

	```
	# use lykoi_data::dag::Dag;
	# let mut dag = Dag::new();
	# let lykoi = 0;
	# let lykoi_drive = 1;
	# let lykoi_gl = 2;
	# let lykoi_data = 3;
	let i_0 = dag.point(lykoi);
	let i_1 = dag.point(lykoi_drive);
	let i_2 = dag.point(lykoi_gl);
	let i_3 = dag.point(lykoi_data);

	dag.depend(&i_0, &i_3); // `lykoi` needs `lykoi_data`
	dag.depend(&i_0, &i_1); // `lykoi` also needs `lykoi_drive`
	dag.depend(&i_1, &i_2); // and `lykoi_drive` needs `lykoi_gl`!
	```
	*/
	pub fn depend(&mut self, value: &Index, needs: &Index) {
		self.edges.push((value.0, needs.0));
	}

	/**
	consumes `self`, and attempts to build a `Vec<T>` sorted such that any
	point with dependencies appears after those dependencies.

	`Err` is produced if the graph is incomplete, or two points in the
	graph have a cyclic dependency.

	at the moment, stability of the list produced isn't guaranteed: moving the
	order in which edges were added (via [`Self::depend()`]) may alter the
	order of the list.
	
	```
	# use lykoi_data::dag::Dag;
	# let mut dag = Dag::new();
	# let lykoi = 0;
	# let lykoi_drive = 1;
	# let lykoi_gl = 2;
	# let lykoi_data = 3;
	let i_0 = dag.point(lykoi);
	let i_1 = dag.point(lykoi_drive);
	let i_2 = dag.point(lykoi_gl);
	let i_3 = dag.point(lykoi_data);

	dag.depend(&i_0, &i_3); // `lykoi` depends on `lykoi_data`
	dag.depend(&i_0, &i_1); // `lykoi` depends on `lykoi_drive`
	dag.depend(&i_1, &i_2); // `lykoi_drive` depends on `lykoi_gl`

	let order = dag.build().unwrap(); // consumes `self`

	// note that `lykoi_gl` appears before `lykoi_drive`
	assert_eq!(order, vec![lykoi_data, lykoi_gl, lykoi_drive, lykoi]);
	```
	*/
	pub fn build(self) -> Result<Vec<T>, DagError> {
		let mut counts = vec![Some(0u32); self.points.len()];

		let mut queue = Vec::new();
		let mut result = Vec::new();

		loop {
		
			for i in 0..self.edges.len() {
				if counts[self.edges[i].1] == None {
					continue;
				}
				let Some(ref mut e) = counts[self.edges[i].0] else {
					continue;
				};
				*e += 1;
			}

			for i in 0..counts.len() {
				let c = counts[i];
				if c == Some(0) {
					queue.push(i);
				}
			}
			
			let mut check_end = false;
			while let Some(o) = queue.pop() {
				result.push(o);
				counts[o].take();
				check_end = true;
			}
			if !check_end {
				break;
			}

			for i in 0..self.edges.len() {
				let Some(ref mut e) = counts[self.edges[i].0] else {
					continue;
				};
				*e = 0;
			}

		}

		if result.len() != self.points.len() {
			return Err(DagError::Incomplete);
		}

		Ok(result.into_iter().map(|x| self.points[x].clone()).collect())
	}
}


#[cfg(test)]
mod test {

	#[test]
	fn test_build_0() {
		let mut dag = super::Dag::new();
	
		let a = dag.point(10);
		let b = dag.point(20);
		let c = dag.point(30);
		let d = dag.point(40);
	
		dag.depend(&b, &a);
		dag.depend(&c, &a);
		dag.depend(&d, &c);
		dag.depend(&d, &b);
	
		let order = dag.build();
	
		assert_eq!(&order.unwrap(), &[10, 30, 20, 40]);
	}

	#[test]
	fn test_build_1() {
		let mut dag = super::Dag::new();
	
		let a = dag.point(10);
		let b = dag.point(20);
		let c = dag.point(30);
	
		dag.depend(&a, &c);
		dag.depend(&b, &a);
		dag.depend(&a, &b);
	
		let order = dag.build();
	
		assert!(&order.is_err());
	}
}




