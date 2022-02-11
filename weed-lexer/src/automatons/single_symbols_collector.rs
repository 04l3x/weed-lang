use crate::error::LexerError;
use crate::token::Token;
use crate::traits::{Collector, Cursor};

pub struct SingleSymbolsCollector;

#[derive(Debug, PartialEq)]
pub enum State {
	Initial,
	Finished,
}

impl<'a, T, I> Collector<'a, T, I, Token> for SingleSymbolsCollector
where
	I: Iterator<Item = char>,
	T: Cursor<I>,
{
	type State = State;
	type Error = LexerError;

	fn collect(cursor: &mut T) -> Result<Token, Self::Error> {
		let mut state = State::Initial;
		let mut buffer = String::new();

		while state != State::Finished {
			match cursor.current() {
				Some(c) => {
					cursor.push_buffer(&mut buffer, c);
					state = State::Finished;
				}
				_ => state = State::Finished,
			}
		}
		Ok(Token::get(buffer))
	}
}
