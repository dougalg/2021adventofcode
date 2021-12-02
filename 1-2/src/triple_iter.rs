// Based on https://stackoverflow.com/questions/66234046/how-to-generate-iterator-with-sliding-window-pairs
pub struct TripleIter<I, T>
where
	I: Iterator<Item = T>,
	T: Clone,
{
	iterator: I,
	one_ago_item: Option<T>,
	two_ago_item: Option<T>,
}

impl<I, T> TripleIter<I, T>
where
   I: Iterator<Item = T>,
   T: Clone,
{
	pub fn new(iterator: I) -> Self {
		TripleIter {
			iterator,
			one_ago_item: None,
			two_ago_item: None,
		}
	}
}

impl<I, T> Iterator for TripleIter<I, T>
where
	I: Iterator<Item = T>,
	T: Clone,
{
	type Item = [T; 3];
	fn next(&mut self) -> Option<Self::Item> {
		if self.two_ago_item.is_none() {
			self.two_ago_item = self.iterator.next();
		}
		if self.two_ago_item.is_none() {
			return None;
		}
		if self.one_ago_item.is_none() {
			self.one_ago_item = self.iterator.next();
		}
		if self.one_ago_item.is_none() {
			return None;
		}
		let curr_item = self.iterator.next();
		if curr_item.is_none() {
			return None;
		}
		let temp_item_one_ago = curr_item.clone();
		let temp_item_two_ago = self.one_ago_item.clone();
		let result = [self.two_ago_item.take().unwrap(), self.one_ago_item.take().unwrap(), curr_item.unwrap()];
		self.one_ago_item = temp_item_one_ago;
		self.two_ago_item = temp_item_two_ago;
		Some(result)
	}
}
