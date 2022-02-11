use crate::error::LexerError;
use crate::token::Token;
use crate::traits::{Collector, Cursor};

pub struct AmpersandCollector;

#[derive(Debug, PartialEq)]
pub enum State {
	Initial,
	Ampersand,
	Finished,
}

impl<'a, T, I> Collector<'a, T, I, Token> for AmpersandCollector
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
						'&' => {
							state = State::Ampersand;
							cursor.push_buffer(&mut buffer, c);
						}
						_ => {}
					},
					_ => {}
				},
				State::Ampersand => match cursor.current() {
					Some(c) => match c {
						'&' => {
							state = State::Finished;
							cursor.push_buffer(&mut buffer, c);
						}
						_ => return Err(Self::Error::default()),
					},
					_ => state = State::Finished,
				},
				_ => {}
			}
		}
		Ok(Token::get(buffer))
	}
}
