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
	array: Vec<T>,
	head: CircularPointer,
	tail: CircularPointer,
	is_full: bool,
}

#[derive(Debug, PartialEq)]
pub enum Error {
	EmptyBuffer,
	FullBuffer,
}

impl<T> CircularBuffer<T>{
	pub fn new(capacity: usize) -> Self {
		Self {
			array: Vec::with_capacity(capacity),
			head: CircularPointer::new(capacity),
			tail: CircularPointer::new(capacity),
			is_full: false,
		}
	}

	pub fn write(&mut self, _element: T) -> Result<(), Error> {
		if self.is_full {
			Err(Error::FullBuffer)
		} else {
			self.array[self.tail.get_idx()] = _element;
			self.tail.next();
			if self.tail == self.head {
				self.is_full = true;
			}

			Ok(())
		}
	}

	pub fn read(&mut self) -> Result<T, Error> {
		if self.head == self.tail{
			Err(Error::EmptyBuffer)
		} else {
			Ok(self.array[self.head.get_idx()])
		}
	}

	pub fn clear(&mut self) {
		self.head = self.tail;
		self.is_full = false;
	}

	pub fn overwrite(&mut self, _element: T) {
		if !self.is_full{
			self.write(_element);
		} else {
			self.array[self.tail.get_idx()] = _element;
			self.tail.next();
			// Over write the last element.
			self.head.next();
			// It's already full, no need to maintain is_full
		}
	}

}
