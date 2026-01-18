use crate::token::Token;

pub struct Lexer {
    src: Vec<char>,
    pos: usize,
    last_token: Option<Token>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            src: input.chars().collect(),
            pos: 0,
            last_token: None,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let t = self.next_token();
            if t == Token::Eof { tokens.push(t); break; }
            tokens.push(t);
        }
        tokens
    }

    fn next_token(&mut self) -> Token {
        if let Some(t) = self.skip_ws() {
            self.last_token = Some(t.clone());
            return t;
        }

        let c = match self.peek() {
            Some(c) => c,
            None => return Token::Eof,
        };

        let tok = match c {
            '(' => { self.pos += 1; Token::LParen }
            ')' => { self.pos += 1; Token::RParen }
            '{' => { self.pos += 1; Token::LBrace }
            '}' => { self.pos += 1; Token::RBrace }
            ',' => { self.pos += 1; Token::Comma }
            ';' => { self.pos += 1; Token::Semicolon }
            '=' => {
                if self.peek2('=') { self.pos += 2; Token::EqEq }
                else { self.pos += 1; Token::Eq }
            }
            '!' if self.peek2('=') => { self.pos += 2; Token::Ne }
            '>' => {
                if self.peek2('=') { self.pos += 2; Token::Ge }
                else { self.pos += 1; Token::Gt }
            }
            '<' => {
                if self.peek2('=') { self.pos += 2; Token::Le }
                else { self.pos += 1; Token::Lt }
            }
            '&' if self.peek2('&') => { self.pos += 2; Token::AndAnd }
            '|' if self.peek2('|') => { self.pos += 2; Token::OrOr }
            '"' => self.read_string(),
            c if c.is_ascii_digit() => self.read_number(),
            c if c.is_ascii_alphabetic() || c == '_' => self.read_ident(),
            _ => { self.pos += 1; return self.next_token(); }
        };

        self.last_token = Some(tok.clone());
        tok
    }

    fn skip_ws(&mut self) -> Option<Token> {
        let mut saw_newline = false;

        while let Some(c) = self.peek() {
            if c == '\n' {
                saw_newline = true;
                self.pos += 1;
            } else if c.is_whitespace() {
                self.pos += 1;
            } else {
                break;
            }
        }

        if saw_newline {
            if let Some(t) = &self.last_token {
                match t {
                    Token::Ident(_) |
                    Token::Number(_) |
                    Token::Str(_) |
                    Token::RParen => {
                        return Some(Token::Semicolon);
                    }
                    _ => {}
                }
            }
        }

        None
    }

    fn peek(&self) -> Option<char> {
        self.src.get(self.pos).copied()
    }

    fn peek2(&self, c: char) -> bool {
        self.src.get(self.pos + 1) == Some(&c)
    }

    fn read_string(&mut self) -> Token {
        self.pos += 1;
        let mut s = String::new();
        while let Some(c) = self.peek() {
            self.pos += 1;
            if c == '"' { break; }
            s.push(c);
        }
        Token::Str(s)
    }

    fn read_number(&mut self) -> Token {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if !c.is_ascii_digit() { break; }
            s.push(c);
            self.pos += 1;
        }
        Token::Number(s.parse().unwrap())
    }

    fn read_ident(&mut self) -> Token {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if !(c.is_ascii_alphanumeric() || c == '_') { break; }
            s.push(c);
            self.pos += 1;
        }

        match s.as_str() {
            "var" => Token::Var,
            "if" => Token::If,
            "else" => Token::Else,
            "loop" => Token::Loop,
            _ => Token::Ident(s),
        }
    }
}