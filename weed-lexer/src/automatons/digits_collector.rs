use crate::error::LexerError;
use crate::token::Token;
use crate::traits::{Collector, Cursor};

pub struct DigitsCollector;

#[derive(Debug, PartialEq)]
pub enum State {
	Initial,
	CollectingIntegers,
	CollectingDecimals,
	StopCollecting,
}

impl<'a, T, I> Collector<'a, T, I, Token> for DigitsCollector
where
	I: Iterator<Item = char>,
	T: Cursor<I>,
{
	type State = State;
	type Error = LexerError;

	fn collect(cursor: &mut T) -> Result<Token, Self::Error> {
		let mut state = State::Initial;
		let mut buffer = String::new();

		while state != State::StopCollecting {
			match cursor.current() {
				Some(c) => match c {
					'0' => match state {
						State::Initial => {
							if buffer.is_empty() {
								cursor.push_buffer(&mut buffer, c);
							} else {
								cursor.consume();
							}
						}
						_ => cursor.push_buffer(&mut buffer, c),
					},

					'1'..='9' => match state {
						State::Initial => {
							state = State::CollectingIntegers;
							cursor.push_buffer(&mut buffer, c);
						}
						_ => cursor.push_buffer(&mut buffer, c),
					},

					'.' => match state {
						State::Initial => {
							state = State::CollectingDecimals;
							if buffer.is_empty() {
								buffer.push('0');
							}
							cursor.push_buffer(&mut buffer, c);
						}
						State::CollectingIntegers => {
							state = State::CollectingDecimals;
							cursor.push_buffer(&mut buffer, c);
						}
						State::CollectingDecimals => return Err(Self::Error::default()),
						_ => {}
					},

					_ => match state {
						_ => state = State::StopCollecting,
					},
				},
				_ => state = State::StopCollecting,
			}
		}

		if buffer.contains(".") {
			Ok(Token::get_float_literal(buffer))
		} else {
			Ok(Token::get_int_literal(buffer))
		}
	}
}
