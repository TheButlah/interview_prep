//! The simplest arena allocator
//!
//! Features:
//! - No lifetimes
//! - Insertion O(1)
//! - Retrieval O(1)

use std::marker::PhantomData;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Idx<T> {
	inner: usize,
	phantom: PhantomData<T>,
}
impl<T> Idx<T> {
	fn new(idx: usize) -> Self {
		Self {
			inner: idx,
			phantom: Default::default(),
		}
	}
}

pub struct Arena<T> {
	v: Vec<T>,
}
impl<T> Arena<T> {
	pub fn new() -> Self {
		Self { v: Vec::new() }
	}

	pub fn len(&self) -> usize {
		self.v.len()
	}

	pub fn insert(&mut self, item: T) -> Idx<T> {
		self.v.push(item);
		Idx::new(self.v.len() - 1)
	}

	pub fn get(&self, idx: Idx<T>) -> Option<&T> {
		self.v.get(idx.inner)
	}

	pub fn get_mut(&mut self, idx: Idx<T>) -> Option<&mut T> {
		self.v.get_mut(idx.inner)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_arena() {
		let s1 = String::from("s1");
		let s2 = String::from("s2");

		let mut a = Arena::new();
		assert_eq!(a.len(), 0);

		let x1 = a.insert(&s1);
		assert_eq!(a.len(), 1);
		assert_eq!(*a.get(x1).unwrap(), &s1);

		let x2 = a.insert(&s2);
		assert_eq!(a.len(), 2);
		assert_eq!(*a.get(x1).unwrap(), &s1);
		assert_eq!(*a.get(x2).unwrap(), &s2);
	}
}
