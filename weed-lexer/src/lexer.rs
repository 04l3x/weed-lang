use crate::cursor::Cursor;
use crate::token::Token;

pub fn tokenize<'a, I>(input: I) -> impl Iterator<Item = Token>
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
			Token::new(TokenKind::Keyword(Keyword::Burn)),
			Token::new(TokenKind::Identifier("fire".to_string())),
			Token::new(TokenKind::Symbol(Symbol::Group(GroupSymbols::BkSlash))),
			Token::new(TokenKind::Symbol(Symbol::Slash)),
			Token::new(TokenKind::Symbol(Symbol::Punctuation(
				PunctuationSymbols::Colon,
			))),
			Token::new(TokenKind::Keyword(Keyword::Dry)),
			Token::new(TokenKind::Symbol(Symbol::Group(GroupSymbols::LBracket))),
			Token::new(TokenKind::Identifier("console_output".to_string())),
			Token::new(TokenKind::Symbol(Symbol::Group(GroupSymbols::BkSlash))),
			Token::new(TokenKind::Literal(Literal::Str {
				value: "hello world".to_string(),
			})),
			Token::new(TokenKind::Symbol(Symbol::Slash)),
			Token::new(TokenKind::Symbol(Symbol::Punctuation(
				PunctuationSymbols::Semicolon,
			))),
			Token::new(TokenKind::Symbol(Symbol::Group(GroupSymbols::RBracket))),
		];

		assert_eq!(token_secuence.collect::<Vec<Token>>(), expected);
	}

	#[test]
	fn test_basic_statement() {
		let input = "
			//comment
			puff sativa dollars_in_the_bank = 0 - 0 + 0 * 0000000;
		";

		let token_secuence = tokenize(input.chars());

		let expected = vec![
			Token::new(TokenKind::Keyword(Keyword::Puff)),
			Token::new(TokenKind::Keyword(Keyword::Sativa)),
			Token::new(TokenKind::Identifier("dollars_in_the_bank".to_string())),
			Token::new(TokenKind::Symbol(Symbol::Operator(OperatorSymbols::Assign))),
			Token::new(TokenKind::Literal(Literal::Int {
				value: "0".to_string(),
			})),
			Token::new(TokenKind::Symbol(Symbol::Arithmetic(
				ArithmeticSymbols::Minus,
			))),
			Token::new(TokenKind::Literal(Literal::Int {
				value: "0".to_string(),
			})),
			Token::new(TokenKind::Symbol(Symbol::Arithmetic(
				ArithmeticSymbols::Plus,
			))),
			Token::new(TokenKind::Literal(Literal::Int {
				value: "0".to_string(),
			})),
			Token::new(TokenKind::Symbol(Symbol::Arithmetic(
				ArithmeticSymbols::Multiply,
			))),
			Token::new(TokenKind::Literal(Literal::Int {
				value: "0".to_string(),
			})),
			Token::new(TokenKind::Symbol(Symbol::Punctuation(
				PunctuationSymbols::Semicolon,
			))),
		];

		assert_eq!(token_secuence.collect::<Vec<Token>>(), expected);
	}

	#[test]
	#[ignore] //FIXME
	fn this_fails() {
		let input = "
			/***/
		";

		let token_secuence = tokenize(input.chars());

		let expected = vec![];
		assert_eq!(token_secuence.collect::<Vec<Token>>(), expected);
	}
}
