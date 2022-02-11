use crate::error::LexerError;
use crate::token::Token;
use crate::traits::{Collector, Cursor};

pub struct DoubleSymbolsCollector;

#[derive(Debug, PartialEq)]
pub enum State {
	Initial,
	Single,
	Double,
	Finished,
}

impl<'a, T, I> Collector<'a, T, I, Token> for DoubleSymbolsCollector
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
			match state {
				State::Initial => match cursor.current() {
					Some(c) => match c {
						'=' | '+' | '<' | '>' | '*' | '%' | '!' => {
							state = State::Single;
							cursor.push_buffer(&mut buffer, c);
						}
						_ => return Err(Self::Error::default()),
					},
					_ => state = State::Finished,
				},
				State::Single => match cursor.current() {
					Some(c) => match c {
						'=' => {
							state = State::Double;
							cursor.push_buffer(&mut buffer, c);
						}
						_ => state = State::Finished,
					},
					_ => state = State::Finished,
				},
				State::Double => state = State::Finished,
				State::Finished => {}
			}
		}
		Ok(Token::get(buffer))
	}
}
