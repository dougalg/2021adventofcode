// Based on https://stackoverflow.com/questions/66234046/how-to-generate-iterator-with-sliding-window-pairs
pub struct Window<I, T>
where
	I: Iterator<Item = T>,
	T: Clone,
{
	iterator: I,
	previous_items: Vec<Option<T>>,
	is_full: bool,
}

impl<I, T> Window<I, T>
where
	I: Iterator<Item = T>,
	T: Clone,
{
	pub fn new(iterator: I, window_size: usize) -> Self {
		Window {
			iterator,
			previous_items: vec![None; window_size],
			is_full: false,
		}
	}
}

impl<I, T> Iterator for Window<I, T>
where
	I: Iterator<Item = T>,
	T: Clone,
{
	type Item = Vec<T>;
	fn next(&mut self) -> Option<Self::Item> {
		let next = self.iterator.next();
		if next.is_none() {
			return None;
		}
		self.previous_items.remove(0);
		self.previous_items.push(next);
		self.is_full = !self.previous_items[0].is_none();
		if !self.is_full {
			self.next()
		} else {
			let result = self.previous_items.iter()
				.filter_map(|item| item.clone())
				.collect();
			Some(result)
		}
	}
}
