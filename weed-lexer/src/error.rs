#[derive(Debug)]
pub enum LexerError {
	UnexpectedToken,
}

impl Default for LexerError {
	fn default() -> Self {
		LexerError::UnexpectedToken
	}
}

impl std::error::Error for LexerError {}

impl std::fmt::Display for LexerError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self)
	}
}
