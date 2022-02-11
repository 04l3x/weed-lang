pub trait Cursor<I>
where
	I: Iterator<Item = char>,
{
	fn consume(&mut self);

	fn current(&self) -> Option<char>;

	fn prev(&self) -> Option<char>;

	fn advance(&mut self, is_stopped: bool) {
		if !is_stopped {
			self.consume();
		}
	}

	fn push_buffer(&mut self, buffer: &mut String, c: char) {
		buffer.push(c);
		self.consume();
	}
}

pub trait Diner<T, I>
where
	I: Iterator<Item = char>,
	T: Cursor<I>,
{
	type State;

	fn eat(cursor: &mut T);
}

pub trait Collector<'a, T, I, U>
where
	I: Iterator<Item = char>,
	T: Cursor<I>,
{
	type State;
	type Error;

	fn collect(cursor: &mut T) -> Result<U, Self::Error>;
}
