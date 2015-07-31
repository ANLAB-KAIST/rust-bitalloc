pub type T = u64;

pub struct Bitalloc<'a> {
	entries: usize,
	buffer: &'a mut [T],
}

impl<'a> Bitalloc<'a,> {
	pub fn wrap(entries: usize, buffer: &'a mut [T]) -> Option<Bitalloc<'a>> {
		Some(
			Bitalloc::<'a> {
				entries: entries,
				buffer: buffer,
			}
		)
	}
}