use crate::error::LexerError;
use crate::token::Token;
use crate::traits::{Collector, Cursor};

pub struct StringsCollector;

#[derive(Debug, PartialEq)]
pub enum State {
	Initial,
	Collecting,
	Finished,
}

impl<'a, T, I> Collector<'a, T, I, Token> for StringsCollector
where
	I: Iterator<Item = char>,
	T: Cursor<I>,
{
	type State = State;
	type Error = LexerError;

	fn collect(cursor: &mut T) -> Result<Token, Self::Error> {
		let mut state = State::Initial;
		let stoped = false;
		let mut buffer = String::new();

		while state != State::Finished {
			match cursor.current() {
				Some(c) => match c {
					'"' => match state {
						State::Initial => state = State::Collecting,
						State::Collecting => {
							state = State::Finished;
						},
						_ => {}
					},
					_ => match state {
						State::Initial => return Err(Self::Error::default()),
						State::Collecting => buffer.push(c),
						_ => {}
					},
				},
				None => return Err(Self::Error::default()),
			}
			cursor.advance(stoped);
		}
		Ok(Token::get_string_literal(buffer))
	}
}
