/*!
box layout solver

this module provides a "box layout solver", which takes a tree of size definitions,
and builds a list of [`Instruction`], each representing a single draw command.

the algorithm used is based on the one presented in the
[Clay library](https://www.nicbarker.com/clay), though more generalized.
*/

use std::{cell::RefCell};

#[derive(Debug)]
enum Instruction {
	Rectangle {
		position: (u32, u32),
		size: (u32, u32),
	},
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Sizing {
	Fit,
	Grow,
	Fixed(u32),
}
impl Sizing {
	#[inline]
	fn to(self) -> u32 {
		match self {
			Sizing::Fit => 0,
			Sizing::Grow => 0,
			Sizing::Fixed(v) => v,
		}
	}
}

#[derive(Debug)]
enum Direction {
	LeftRight,
	TopBottom,
}



#[derive(Debug)]
struct Build {
	size: (u32, u32),
}
impl Build {
	fn new() -> Self {
		Self {
			size: (0, 0),
		}
	}
}



#[derive(Debug)]
struct Element {
	position: (u32, u32),
	size: (Sizing, Sizing),
	padding: u32,
	gap: u32,
	color: (),
	direction: Direction,
	children: Vec<Element>,
	dynamic: RefCell<Build>,
}
impl Default for Element {
	fn default() -> Self {
		Self {
			position: (0, 0),
			size: (Sizing::Fit, Sizing::Fit),
			padding: 0,
			gap: 0,
			color: (),
			direction: Direction::LeftRight,
			children: Vec::new(),
			dynamic: RefCell::new(Build::new()),
		}
	}
}
impl Element {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn build(self) -> Vec<Instruction> {
		build(self)
	}
}


fn build(root: Element) -> Vec<Instruction> {

	pass_fit(&mut Build::new(), &root);

	println!(":: {:#?}", root);

	pass_grow(&root);

	println!(":: {:#?}", root);

	todo!()
}

fn pass_fit(parent: &mut Build, node: &Element) {
	let data = &mut node.dynamic.borrow_mut();
	data.size.0 = node.size.0.to();
	data.size.1 = node.size.1.to();

	for child in &node.children {
		pass_fit(data, child);
	}

	let gap = (node.children.len() as i32 - 1).max(0) as u32 * node.gap;

	data.size.0 += node.padding * 2 + gap;
	data.size.1 += node.padding * 2;

	parent.size.0 += data.size.0;
	parent.size.1 = parent.size.1.max(data.size.1);
}

fn pass_grow(node: &Element) {
	let data = node.dynamic.borrow_mut();

	let mut remain_w = (data.size.0 - node.padding * 2) -
		(node.children
			.iter()
			.map(|e| e.dynamic.borrow().size.0)
			.reduce(|a, e| a + e)
			.unwrap_or(0)) -
		((node.children.len() as i32 - 1).max(0) as u32 * node.gap);

	let remain_h = data.size.1 - node.padding * 2;

	let growable = node.children.iter().filter(|e| e.size.0 == Sizing::Grow).collect::<Vec<_>>();

	while remain_w > 0 {
		let mut smallest = growable[0].dynamic.borrow().size.0;
		let mut next = u32::MAX;
		let mut to_add = remain_w;

		for child in &growable {
			let width = child.dynamic.borrow().size.0;
			if width < smallest {
				next = smallest;
				smallest = width;
			}
			if width > smallest {
				next = next.min(width);
				to_add = next - smallest;
			}
		}

		to_add = to_add.min(remain_w / growable.len() as u32);

		for child in &growable {
			let width = &mut child.dynamic.borrow_mut().size.0;
			if *width == smallest {
				*width += to_add;
				remain_w -= to_add;
			}
		}

	}


	for child in &node.children {
		let mut child_data = child.dynamic.borrow_mut();
		if child.size.0 == Sizing::Grow {
			child_data.size.0 += remain_w;
		}
		if child.size.1 == Sizing::Grow {
			child_data.size.1 += remain_h - child_data.size.1;
		}
	}

	for child in &node.children {
		pass_grow(child);
	}
}



// #[cfg(test)]
mod test {
	use super::*;
	// #[test]
	fn main() {
		let element = Element {
			size: (Sizing::Fixed(400), Sizing::Fit),
			padding: 20,
			gap: 20,
			children: vec![
				Element {
					size: (Sizing::Fixed(100), Sizing::Fixed(100)),
					..Default::default()
				},
				Element {
					size: (Sizing::Grow, Sizing::Grow),
					..Default::default()
				},
				Element {
					size: (Sizing::Fixed(100), Sizing::Fixed(80)),
					..Default::default()
				},
			],
			..Default::default()
		};

		println!("{:?}", element);

		let inst = element.build();

		println!("{:?}", inst);

		assert!(false);
	}
}


