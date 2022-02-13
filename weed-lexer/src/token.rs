use crate::error;

type Result<T> = std::result::Result<T, error::LexerError>;

#[derive(Debug, Eq, PartialEq)]
pub struct Token {
	token: TokenKind,
}

impl Token {
	pub fn new(token: TokenKind) -> Self {
		Self { token }
	}

	pub fn get(value: String) -> Token {
		match value.as_str() {
			"==" => Token {
				token: TokenKind::Symbol(Symbol::Comparison(ComparisonSymbols::Equals)),
			},
			">=" => Token {
				token: TokenKind::Symbol(Symbol::Comparison(ComparisonSymbols::HigherEq)),
			},
			"<=" => Token {
				token: TokenKind::Symbol(Symbol::Comparison(ComparisonSymbols::LessEq)),
			},
			"+=" => Token {
				token: TokenKind::Symbol(Symbol::Operator(OperatorSymbols::PlusEq)),
			},
			"-=" => Token {
				token: TokenKind::Symbol(Symbol::Operator(OperatorSymbols::MinusEq)),
			},
			"*=" => Token {
				token: TokenKind::Symbol(Symbol::Operator(OperatorSymbols::MultiplyEq)),
			},
			"/=" => Token {
				token: TokenKind::Symbol(Symbol::Operator(OperatorSymbols::DivideEq)),
			},
			"%=" => Token {
				token: TokenKind::Symbol(Symbol::Operator(OperatorSymbols::ModEq)),
			},
			"&&" => Token {
				token: TokenKind::Symbol(Symbol::Logic(LogicSymbols::And)),
			},
			"||" => Token {
				token: TokenKind::Symbol(Symbol::Logic(LogicSymbols::Or)),
			},
			"=" => Token {
				token: TokenKind::Symbol(Symbol::Operator(OperatorSymbols::Assign)),
			},
			">" => Token {
				token: TokenKind::Symbol(Symbol::Comparison(ComparisonSymbols::Higher)),
			},
			"<" => Token {
				token: TokenKind::Symbol(Symbol::Comparison(ComparisonSymbols::Less)),
			},
			"+" => Token {
				token: TokenKind::Symbol(Symbol::Arithmetic(ArithmeticSymbols::Plus)),
			},
			"-" => Token {
				token: TokenKind::Symbol(Symbol::Arithmetic(ArithmeticSymbols::Minus)),
			},
			"*" => Token {
				token: TokenKind::Symbol(Symbol::Arithmetic(ArithmeticSymbols::Multiply)),
			},
			"%" => Token {
				token: TokenKind::Symbol(Symbol::Arithmetic(ArithmeticSymbols::Mod)),
			},
			"|" => Token {
				token: TokenKind::Symbol(Symbol::Punctuation(PunctuationSymbols::Pleca)),
			},

			"(" => Token {
				token: TokenKind::Symbol(Symbol::Group(GroupSymbols::LParent)),
			},
			")" => Token {
				token: TokenKind::Symbol(Symbol::Group(GroupSymbols::RParent)),
			},
			"[" => Token {
				token: TokenKind::Symbol(Symbol::Group(GroupSymbols::LBracket)),
			},
			"]" => Token {
				token: TokenKind::Symbol(Symbol::Group(GroupSymbols::RBracket)),
			},
			"{" => Token {
				token: TokenKind::Symbol(Symbol::Group(GroupSymbols::LBrace)),
			},
			"}" => Token {
				token: TokenKind::Symbol(Symbol::Group(GroupSymbols::RBrace)),
			},
			"\\" => Token {
				token: TokenKind::Symbol(Symbol::Group(GroupSymbols::BkSlash)),
			},
			"/" => Token {
				token: TokenKind::Symbol(Symbol::Slash),
			},
			":" => Token {
				token: TokenKind::Symbol(Symbol::Punctuation(PunctuationSymbols::Colon)),
			},
			";" => Token {
				token: TokenKind::Symbol(Symbol::Punctuation(PunctuationSymbols::Semicolon)),
			},
			"." => Token {
				token: TokenKind::Symbol(Symbol::Punctuation(PunctuationSymbols::Dot)),
			},
			"," => Token {
				token: TokenKind::Symbol(Symbol::Punctuation(PunctuationSymbols::Comma)),
			},
			"!" => Token {
				token: TokenKind::Symbol(Symbol::Logic(LogicSymbols::Not)),
			},

			_ => match Keyword::try_from(value.as_str()) {
				Ok(k) => Token {
					token: TokenKind::Keyword(k),
				},
				_ => Token {
					token: TokenKind::Identifier(value),
				},
			},
		}
	}

	pub fn get_string_literal(value: String) -> Token {
		Token {
			token: TokenKind::Literal(Literal::Str { value }),
		}
	}

	pub fn get_int_literal(value: String) -> Token {
		Token {
			token: TokenKind::Literal(Literal::Int { value }),
		}
	}

	pub fn get_float_literal(value: String) -> Token {
		Token {
			token: TokenKind::Literal(Literal::Float { value }),
		}
	}

	pub fn get_bool_literal(value: String) -> Token {
		Token {
			token: TokenKind::Literal(Literal::Bool { value }),
		}
	}
}

#[derive(Debug, Eq, PartialEq)]
pub enum TokenKind {
	Literal(Literal),
	Keyword(Keyword),
	Identifier(String),
	Symbol(Symbol),
	Comment(Comment),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Literal {
	Str { value: String },
	Int { value: String },
	Float { value: String },
	Bool { value: String },
}

#[derive(Debug, Eq, PartialEq)]
pub enum Keyword {
	Burn,
	Sativa,
	Indica,
	Hybrid,
	Dry,
	Puff,
	Genetic,
	Variety,
	Cupcake,
}

impl TryFrom<&str> for Keyword {
	type Error = error::LexerError;

	fn try_from(value: &str) -> Result<Self> {
		match value {
			"burn" => Ok(Keyword::Burn),
			"sativa" => Ok(Keyword::Sativa),
			"indica" => Ok(Keyword::Indica),
			"hybrid" => Ok(Keyword::Hybrid),
			"dry" => Ok(Keyword::Dry),
			"puff" => Ok(Keyword::Puff),
			"genetic" => Ok(Keyword::Genetic),
			"variety" => Ok(Keyword::Variety),
			"cupcake" => Ok(Keyword::Cupcake),
			_ => Err(Self::Error::default()),
		}
	}
}

#[derive(Debug, Eq, PartialEq)]
pub enum Symbol {
	Comparison(ComparisonSymbols),
	Operator(OperatorSymbols),
	Arithmetic(ArithmeticSymbols),
	Group(GroupSymbols),
	Logic(LogicSymbols),
	Punctuation(PunctuationSymbols),
	Slash,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ComparisonSymbols {
	/// ==
	Equals,
	/// >
	Higher,
	/// >=
	HigherEq,
	/// <
	Less,
	/// <=
	LessEq,
}

#[derive(Debug, Eq, PartialEq)]
pub enum OperatorSymbols {
	/// =
	Assign,
	/// +=
	PlusEq,
	/// -=
	MinusEq,
	/// *=
	MultiplyEq,
	/// /=
	DivideEq,
	/// %=
	ModEq,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ArithmeticSymbols {
	/// +
	Plus,
	/// -
	Minus,
	/// *
	Multiply,
	/// %
	Mod,
}

#[derive(Debug, Eq, PartialEq)]
pub enum GroupSymbols {
	///(
	LParent,
	///),
	RParent,
	///{
	LBrace,
	///},
	RBrace,
	///[
	LBracket,
	///],
	RBracket,
	/// \,
	BkSlash,
}

#[derive(Debug, Eq, PartialEq)]
pub enum PunctuationSymbols {
	/// :
	Colon,
	/// ;
	Semicolon,
	/// .
	Dot,
	/// ,
	Comma,
	/// |,
	Pleca,
}

#[derive(Debug, Eq, PartialEq)]
pub enum LogicSymbols {
	/// &&
	And,
	/// ||
	Or,
	/// !
	Not,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Comment {
	Line,
	Block,
}
