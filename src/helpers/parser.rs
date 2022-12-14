use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum ParseError<TToken: Debug> {
    UnexpectedToken(TToken, TToken),
    InvalidList,
    InvalidInteger,
    NotImplemented,
    LeftOverTokens,
}

#[derive(Debug, PartialEq)]
pub enum LexError {
    InvalidCharacter(char),
}

pub type LexResult<T> = Result<T, LexError>;
pub type ParseResult<T, TToken> = Result<T, ParseError<TToken>>;

#[derive(Debug)]
pub struct Token<LexTokenType: PartialEq> {
    pub token_type: LexTokenType,
    pub content: String,
}

impl<LexTokenType: PartialEq> Token<LexTokenType> {
    pub fn new(token_type: LexTokenType, content: String) -> Self {
        Self {
            token_type,
            content,
        }
    }
}

pub trait Parser<LexTokenType: PartialEq> {
    fn tokens(&self) -> &Vec<Token<LexTokenType>>;
    fn pos(&self) -> usize;
    fn set_pos(&mut self, pos: usize);

    fn is_eof(&self) -> bool {
        self.pos() >= self.tokens().len()
    }

    fn peek(&self) -> &Token<LexTokenType> {
        &self.tokens()[self.pos()]
    }

    fn is_match(&self, token_type: LexTokenType) -> bool {
        !self.is_eof() && self.peek().token_type == token_type
    }

    fn advance(&mut self) {
        self.set_pos(self.pos() + 1);
    }
}
