use std::{array, mem};

pub struct Lookahead<const LOOKAHEAD: usize, T: Default, Input: Iterator<Item = T>> {
	input: Input,
    ring_buffer: [T; LOOKAHEAD],
	position: usize,
	available_lookahead: usize
}

pub struct LookaheadConsume<'a, const LOOKAHEAD: usize, T: Default, Input: Iterator<Item = T>> {
	lookahead: &'a mut Lookahead<LOOKAHEAD, T, Input>
}

impl<'a, const LOOKAHEAD: usize, T: Default, Input: Iterator<Item = T>> Iterator for LookaheadConsume<'a, LOOKAHEAD, T, Input> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		let look = &mut self.lookahead;
		if look.available_lookahead == 0 { return None; }
		match look.input.next() {
			// Write to the current position *before* incrementing it, because the current position will become the
			// furthest-ahead position once the increment happens.
			Some(byte) => {
				let leaving = mem::replace(&mut look.ring_buffer[look.position % LOOKAHEAD], byte);
				look.position += 1;
				Some(leaving)
			},
			None => {
				let leaving = mem::take(&mut look.ring_buffer[look.position % LOOKAHEAD]);
				look.position += 1;
				look.available_lookahead = look.available_lookahead.saturating_sub(1);
				Some(leaving)
			}
		}
	}
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

	pub fn at_end(&self) -> bool {
		self.available_lookahead == 0
	}

	pub fn position(&self) -> usize {
		self.position
	}

	pub fn consume(&mut self) -> LookaheadConsume<'_, LOOKAHEAD, T, Input> {
		LookaheadConsume { lookahead: self }
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