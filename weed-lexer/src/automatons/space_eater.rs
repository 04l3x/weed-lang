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
		println!("eating");
		let mut state = State::Initial;

		while state != State::Full {
			println!("eating: {:?}", cursor.current());
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
					None => {
						state = State::Full;
					}
				},
				_ => {}
			}
		}
	}
}
