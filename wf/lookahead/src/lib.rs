use std::array;

pub struct Lookahead<const LOOKAHEAD: usize, T: Default, Input: Iterator<Item = T>> {
	input: Input,
    ring_buffer: [T; LOOKAHEAD],
	position: usize,
	available_lookahead: usize
}

impl<const LOOKAHEAD: usize, T: Default, Input: Iterator<Item = T>> Lookahead<LOOKAHEAD, T, Input> {
    pub fn new(mut input: Input) -> Self {
		let mut ring_buffer = array::from_fn(|_| Default::default());
		let mut available_lookahead = 0;
		for slot in ring_buffer.iter_mut() {
			let Some(byte) = input.next() else { break };
			*slot = byte;
			available_lookahead += 1;
		}
		Self {
			input,
			ring_buffer,
			position: 0,
			available_lookahead
		}
    }

	pub fn position(&self) -> usize {
		self.position
	}

	pub fn consume(&mut self, number_of_items: usize) -> Option<()> {
		debug_assert!(number_of_items > 0);
		if self.available_lookahead == 0 { return None; }
		for _ in 0..number_of_items {
			match self.input.next() {
				// Write to the current position *before* incrementing it, because the current position will become the
				// furthest-ahead position once the increment happens.
				Some(byte) => self.ring_buffer[self.position % LOOKAHEAD] = byte,
				None => self.available_lookahead = self.available_lookahead.saturating_sub(1)
			}
			self.position += 1;
		}
        Some(())
    }

    pub fn peek(&mut self, offset: usize) -> Option<&T> {
		debug_assert!(offset < LOOKAHEAD);
		// Imagine there's only one valid index - this corresponds to offset 0.
		// We want offset 0 to pass but offset 1 to fail.
        if offset >= self.available_lookahead { return None; }
		let index = (self.position + offset) % LOOKAHEAD;
        Some(&self.ring_buffer[index])
    }
}