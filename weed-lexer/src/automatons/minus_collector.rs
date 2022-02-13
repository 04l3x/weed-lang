use crate::error::LexerError;
use crate::token::Token;
use crate::traits::{Collector, Cursor};

pub struct MinusCollector;

#[derive(Debug, PartialEq)]
pub enum State {
	Initial,
	Minus,
	MinusEq,
	Negative,
	NegativeDecimal,
	Finished,
}

impl<'a, T, I> Collector<'a, T, I, Token> for MinusCollector
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
						'-' => {
							state = State::Minus;
							cursor.push_buffer(&mut buffer, c);
						}
						_ => return Err(Self::Error::default()),
					},
					_ => return Err(Self::Error::default()),
				},
				State::Minus => match cursor.current() {
					Some(c) => match c {
						'=' => {
							state = State::MinusEq;
							cursor.push_buffer(&mut buffer, c);
						}
						'.' => {
							state = State::NegativeDecimal;
							if buffer.eq("-") {
								buffer.push('0');
							}
							cursor.push_buffer(&mut buffer, c);
						}
						'0' => {
							if buffer.eq("-0") {
								return Err(Self::Error::default());
							} else {
								cursor.push_buffer(&mut buffer, c);
							}
						}
						'1'..='9' => {
							state = State::Negative;
							cursor.push_buffer(&mut buffer, c);
						}
						_ => state = State::Finished,
					},
					_ => state = State::Finished,
				},
				State::MinusEq => state = State::Finished,
				State::Negative => match cursor.current() {
					Some(c) => match c {
						'0'..='9' => cursor.push_buffer(&mut buffer, c),
						'.' => {
							state = State::NegativeDecimal;
							cursor.push_buffer(&mut buffer, c);
						}
						_ => state = State::Finished,
					},
					_ => state = State::Finished,
				},
				State::NegativeDecimal => match cursor.current() {
					Some(c) => match c {
						'0'..='9' => cursor.push_buffer(&mut buffer, c),
						_ => state = State::Finished,
					},
					_ => state = State::Finished,
				},
				State::Finished => {}
			}
		}

		if buffer.len() > 2 {
			if buffer.contains(".") {
				Ok(Token::get_float_literal(buffer))
			} else {
				Ok(Token::get_int_literal(buffer))
			}
		} else {
			Ok(Token::get(buffer))
		}
	}
}
