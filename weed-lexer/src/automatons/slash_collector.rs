use crate::error::LexerError;
use crate::token::Token;
use crate::traits::{Collector, Cursor};

pub struct SlashCollector;

#[derive(Debug, PartialEq)]
pub enum State {
	Initial,
	Slash,
	LineComment,
	BlockComment,
	BlockSemiFull,
	Finished,
}

impl<'a, T, I> Collector<'a, T, I, Option<Token>> for SlashCollector
where
	I: Iterator<Item = char>,
	T: Cursor<I>,
{
	type State = State;
	type Error = LexerError;

	fn collect(cursor: &mut T) -> Result<Option<Token>, Self::Error> {
		let mut state = State::Initial;
		let mut buffer = String::new();

		while state != State::Finished {
			match state {
				State::Initial => match cursor.current() {
					Some(c) => match c {
						'/' => {
							state = State::Slash;
							cursor.push_buffer(&mut buffer, c);
						}
						_ => {}
					},
					_ => {}
				},
				State::Slash => match cursor.current() {
					Some(c) => match c {
						'/' => {
							state = State::LineComment;
							cursor.push_buffer(&mut buffer, c);
						}
						'*' => {
							state = State::BlockComment;
							cursor.push_buffer(&mut buffer, c);
						}
						_ => state = State::Finished,
					},
					_ => {}
				},
				State::LineComment => match cursor.current() {
					Some(c) => match c {
						'\n' => state = State::Finished,
						_ => cursor.consume(),
					},
					_ => {}
				},
				State::BlockComment => match cursor.current() {
					Some(c) => match c {
						'*' => {
							state = State::BlockSemiFull;
							cursor.consume();
						}
						_ => cursor.consume(),
					},
					_ => {}
				},
				State::BlockSemiFull => match cursor.current() {
					Some(c) => match c {
						'/' => {
							state = State::Finished;
							cursor.consume();
						}
						_ => {
							state = State::BlockComment;
							cursor.consume()
						}
					},
					_ => {}
				},
				_ => {}
			}
		}
		if buffer.eq("/") {
			Ok(Some(Token::get(buffer)))
		} else {
			Ok(None)
		}
	}
}
