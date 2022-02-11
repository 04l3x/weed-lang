use crate::traits::{Cursor, Diner};

pub struct CommentEater;

#[derive(Debug, PartialEq)]
pub enum State {
	Initial,
	Started,
	EatingLine,
	EatingBlock,
	SemiFull,
	Full,
}

impl<T, I> Diner<T, I> for CommentEater
where
	I: Iterator<Item = char>,
	T: Cursor<I>,
{
	type State = State;

	fn eat(cursor: &mut T) {
		let mut state = State::Initial;
		let mut stoped = false;

		while state != State::Full {
			match state {
				State::Initial => match cursor.current() {
					Some(c) => {
						if c == '/' {
							state = State::Started;
						}
					}
					None => {
						state = State::Full;
						stoped = true;
					}
				},
				State::Started => match cursor.current() {
					Some(c) => match c {
						'/' => state = State::EatingLine,
						'*' => state = State::EatingBlock,
						_ => panic!(),
					},
					None => {
						state = State::Full;
						stoped = true;
					}
				},
				State::EatingLine => match cursor.current() {
					Some(c) => match c {
						'\n' => {
							state = State::Full;
							stoped = true;
						}
						_ => {}
					},
					None => {
						state = State::Full;
						stoped = true;
					}
				},
				State::EatingBlock => match cursor.current() {
					Some(c) => match c {
						'*' => state = State::SemiFull,
						_ => {}
					},
					None => {
						state = State::Full;
						stoped = true;
					}
				},
				State::SemiFull => match cursor.current() {
					Some(c) => match c {
						'/' => {
							state = State::Full;
							stoped = true;
						}
						_ => state = State::SemiFull,
					},
					None => {
						state = State::Full;
						stoped = true;
					}
				},

				_ => {}
			}
			cursor.advance(stoped);
		}
	}
}
