use crate::automatons::{
	AmpersandCollector, DigitsCollector, DoubleSymbolsCollector, MinusCollector, PlecaCollector,
	SingleSymbolsCollector, SlashCollector, SpaceEater, StringsCollector, WordsCollector,
};
use crate::error::LexerError;
use crate::token::Token;
use crate::traits::{Collector, Cursor as C, Diner};

#[derive(Debug)]
pub struct Cursor<I>
where
	I: Iterator<Item = char>,
{
	input: I,
	position: Position,
	current: Option<char>,
	prev: Option<char>,
	started: bool,
}

impl<I> Cursor<I>
where
	I: Iterator<Item = char>,
{
	pub fn new(input: I) -> Self {
		Self {
			input,
			position: Position::default(),
			current: None,
			prev: None,
			started: false,
		}
	}

	pub fn next_token<'a>(&mut self) -> Option<Token> {
		if !self.started {
			self.consume();
			self.started = true;
		}
		match Tokenizer::collect(self) {
			Ok(t) => t,
			Err(e) => panic!("Lexer Error: {:?}", e),
		}
	}
}

impl<I> C<I> for Cursor<I>
where
	I: Iterator<Item = char>,
{
	fn consume(&mut self) {
		self.prev = self.current;
		self.current = self.input.next();
		self.position.advance(self.current);
	}

	fn current(&self) -> Option<char> {
		self.current
	}

	fn prev(&self) -> Option<char> {
		self.prev
	}
}

#[derive(Debug)]
struct Position {
	absolute: usize,
	line: usize,
	column: usize,
}

impl Default for Position {
	fn default() -> Self {
		Self {
			absolute: 0,
			line: 1,
			column: 0,
		}
	}
}

impl Position {
	fn advance(&mut self, next: Option<char>) {
		if let Some(next) = next {
			if next == '\n' {
				self.next_line();
			} else {
				self.next_column()
			}
		}
	}

	fn next_column(&mut self) {
		self.column += 1;
		self.advance_absolute();
	}

	fn next_line(&mut self) {
		self.line += 1;
		self.column = 1;
		self.advance_absolute();
	}

	fn advance_absolute(&mut self) {
		self.absolute += 1;
	}
}

struct Tokenizer;
#[derive(Debug, PartialEq)]
enum TokenizerState {
	Initial,
	Finished,
}

impl<'a, Cursor, I> Collector<'a, Cursor, I, Option<Token>> for Tokenizer
where
	I: Iterator<Item = char>,
	Cursor: C<I>,
{
	type State = TokenizerState;
	type Error = LexerError;

	fn collect(cursor: &mut Cursor) -> Result<Option<Token>, Self::Error> {
		let mut state = Self::State::Initial;

		while state != Self::State::Finished {
			SpaceEater::eat(cursor);

			match state {
				Self::State::Initial => match cursor.current() {
					Some(c) => {
						match c {
							'/' => match SlashCollector::collect(cursor) {
								Ok(c) => match c {
									Some(t) => return Ok(Some(t)),
									_ => {}
								},
								Err(e) => return Err(e),
							},
							'-' => return Ok(Some(MinusCollector::collect(cursor)?)),
							'|' => return Ok(Some(PlecaCollector::collect(cursor)?)),
							'&' => return Ok(Some(AmpersandCollector::collect(cursor)?)),
							'=' | '+' | '<' | '>' | '*' | '%' | '!' => {
								return Ok(Some(DoubleSymbolsCollector::collect(cursor)?))
							}
							'(' | ')' | '[' | ']' | '{' | '}' | '\\' | ':' | ';' | '.' | ',' => {
								return Ok(Some(SingleSymbolsCollector::collect(cursor)?))
							}
							'\"' => return Ok(Some(StringsCollector::collect(cursor)?)),
							'0'..='9' => return Ok(Some(DigitsCollector::collect(cursor)?)),
							'\0' => state = Self::State::Finished,
							_ => return Ok(Some(WordsCollector::collect(cursor)?)),
						}
					}
					_ => return Ok(None),
				},
				_ => {}
			}
		}
		Ok(None)
	}
}
