extern crate ilog2;
pub type T = u64;

pub struct Bitalloc<'a> {
	entries: usize,
	segment_size: usize,
	buffer: &'a mut [T],
}

impl<'a> Bitalloc<'a,> {
	pub fn get_bit_count(buffer: & [T]) -> usize {
		buffer.len() * ilog2::bit_length::<T>()
	}
	pub fn wrap(entries: usize, buffer: &'a mut [T]) -> Option<Bitalloc<'a>> {
		let mut ok : bool = true;
		let bits = Bitalloc::get_bit_count(buffer);
		if entries > bits {
			ok = false;
		}
		if ok {
			Some(
				Bitalloc::<'a> {
					entries: entries,
					segment_size: buffer.len(),
					buffer: buffer,
				}
			)
		} else {
			None
		}
	}
	pub fn wrap_with_segments(entries: usize, segment_size: usize, buffer: &'a mut [T]) -> Option<Bitalloc<'a>> {
		let mut ok : bool = true;
		let bits = Bitalloc::get_bit_count(buffer);
		if entries > bits {
			ok = false;
		}
		if ok {
			Some(
				Bitalloc::<'a> {
					entries: entries,
					segment_size: segment_size,
					buffer: buffer,
				}
			)
		} else {
			None
		}
	}
	pub fn clear_all(&mut self) {
		for item in self.buffer.iter_mut() {
			*item = 0;
		}
	}
	pub fn fill_all(&mut self) {
		for item in self.buffer.iter_mut() {
			*item = ilog2::bit_mask::<T>();
		}
	}
}