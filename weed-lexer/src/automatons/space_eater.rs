use crate::traits::{Cursor, Diner};

pub struct SpaceEater;

#[derive(Debug, PartialEq)]
pub enum State {
	Initial,
	Hungry,
	Full,
}

impl<T, I> Diner<T, I> for SpaceEater
where
	I: Iterator<Item = char>,
	T: Cursor<I>,
{
	type State = State;

	fn eat(cursor: &mut T) {
		let mut state = State::Initial;

		while state != State::Full {
			match state {
				State::Initial | State::Hungry => match cursor.current() {
					Some(c) => {
						if c.is_whitespace() {
							if state == State::Initial {
								state = State::Hungry;
							}
							cursor.consume();
						} else {
							state = State::Full;
						}
					}
					_ => state = State::Full,
				},
				_ => {}
			}
		}
	}
}
