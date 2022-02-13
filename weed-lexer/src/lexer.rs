use crate::cursor::Cursor;
use crate::error::LexerError;
use crate::token::Token;

pub fn tokenize<'a, I>(input: I) -> impl Iterator<Item = Result<Token, LexerError>>
where
	I: Iterator<Item = char>,
{
	let mut cursor = Cursor::new(input);
	std::iter::from_fn(move || cursor.next_token())
}

#[cfg(test)]
mod tests {
	use crate::{lexer::*, token::*};

	#[test]
	fn test_hello_world() {
		let input = "
			burn fire\\/:dry[ //comment
				console_output\\\"hello world\"/;
			]
		";

		let token_secuence = tokenize(input.chars());

		let expected = vec![
			Ok(Token::new(TokenKind::Keyword(Keyword::Burn))),
			Ok(Token::new(TokenKind::Identifier("fire".to_string()))),
			Ok(Token::new(TokenKind::Symbol(Symbol::Group(
				GroupSymbols::BkSlash,
			)))),
			Ok(Token::new(TokenKind::Symbol(Symbol::Slash))),
			Ok(Token::new(TokenKind::Symbol(Symbol::Punctuation(
				PunctuationSymbols::Colon,
			)))),
			Ok(Token::new(TokenKind::Keyword(Keyword::Dry))),
			Ok(Token::new(TokenKind::Symbol(Symbol::Group(
				GroupSymbols::LBracket,
			)))),
			Ok(Token::new(TokenKind::Identifier(
				"console_output".to_string(),
			))),
			Ok(Token::new(TokenKind::Symbol(Symbol::Group(
				GroupSymbols::BkSlash,
			)))),
			Ok(Token::new(TokenKind::Literal(Literal::Str {
				value: "hello world".to_string(),
			}))),
			Ok(Token::new(TokenKind::Symbol(Symbol::Slash))),
			Ok(Token::new(TokenKind::Symbol(Symbol::Punctuation(
				PunctuationSymbols::Semicolon,
			)))),
			Ok(Token::new(TokenKind::Symbol(Symbol::Group(
				GroupSymbols::RBracket,
			)))),
		];

		assert_eq!(
			token_secuence.collect::<Vec<Result<Token, LexerError>>>(),
			expected
		);
	}

	#[test]
	fn test_basic_statement() {
		let input = "
			//comment
			puff sativa dollars_in_the_bank = 0 - 0 + 0 * 0000000;
		";

		let token_secuence = tokenize(input.chars());

		let expected = vec![
			Ok(Token::new(TokenKind::Keyword(Keyword::Puff))),
			Ok(Token::new(TokenKind::Keyword(Keyword::Sativa))),
			Ok(Token::new(TokenKind::Identifier(
				"dollars_in_the_bank".to_string(),
			))),
			Ok(Token::new(TokenKind::Symbol(Symbol::Operator(
				OperatorSymbols::Assign,
			)))),
			Ok(Token::new(TokenKind::Literal(Literal::Int {
				value: "0".to_string(),
			}))),
			Ok(Token::new(TokenKind::Symbol(Symbol::Arithmetic(
				ArithmeticSymbols::Minus,
			)))),
			Ok(Token::new(TokenKind::Literal(Literal::Int {
				value: "0".to_string(),
			}))),
			Ok(Token::new(TokenKind::Symbol(Symbol::Arithmetic(
				ArithmeticSymbols::Plus,
			)))),
			Ok(Token::new(TokenKind::Literal(Literal::Int {
				value: "0".to_string(),
			}))),
			Ok(Token::new(TokenKind::Symbol(Symbol::Arithmetic(
				ArithmeticSymbols::Multiply,
			)))),
			Ok(Token::new(TokenKind::Literal(Literal::Int {
				value: "0".to_string(),
			}))),
			Ok(Token::new(TokenKind::Symbol(Symbol::Punctuation(
				PunctuationSymbols::Semicolon,
			)))),
		];

		assert_eq!(
			token_secuence.collect::<Vec<Result<Token, LexerError>>>(),
			expected
		);
	}

	#[test]
	fn this_fails() {
		let input = "
			/***/
		";

		let token_secuence = tokenize(input.chars());

		let expected: Vec<Result<Token, LexerError>> = vec![];
		assert_eq!(
			token_secuence.collect::<Vec<Result<Token, LexerError>>>(),
			expected
		);
	}
}
