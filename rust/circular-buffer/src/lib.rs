use std::collections::LinkedList;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct CircularPointer{
	idx: usize,
	capacity: usize,
}
impl CircularPointer{
	pub fn new(capacity: usize) -> Self {
		Self { idx: 0, capacity: capacity }
	}
	pub fn next(&mut self) {
		self.idx = (self.idx + 1) % self.capacity;
	}
	pub fn get_idx(&self) -> usize { self.idx }
}

pub struct CircularBuffer<T> {
	list: LinkedList<T>,
	capacity: usize,
}

#[derive(Debug, PartialEq)]
pub enum Error {
	EmptyBuffer,
	FullBuffer,
}

impl<T> CircularBuffer<T>{
	pub fn new(capacity: usize) -> Self {
		Self {
			list: LinkedList::new(),
			capacity: capacity,
		}
	}

	pub fn write(&mut self, _element: T) -> Result<(), Error> {
		if self.is_full() {
			Err(Error::FullBuffer)
		} else {
			self.list.push_back(_element);
			Ok(())
		}
	}

	pub fn read(&mut self) -> Result<T, Error> {
		match self.list.pop_front() {
			Some(t) => Ok(t),
			None    => Err(Error::EmptyBuffer),
		}
	}

	pub fn clear(&mut self) {
		self.list.clear();
	}

	pub fn overwrite(&mut self, _element: T) {
		if self.is_full(){
			self.list.pop_front();
		}
		let _ = self.write(_element);
	}

	fn is_full(&self) -> bool { self.list.len() == self.capacity }

}
