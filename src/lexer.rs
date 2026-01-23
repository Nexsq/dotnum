use crate::token::{Token, TokenKind};

pub struct Lexer {
    src: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
    last_token: Option<TokenKind>,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            src: input.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
            last_token: None,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let t = self.next_token();
            let eof = t.kind == TokenKind::Eof;
            tokens.push(t);
            if eof {
                break;
            }
        }
        tokens
    }

    fn next_token(&mut self) -> Token {
        if let Some(k) = self.skip_ws_and_comments() {
            return self.make(k);
        }

        let c = match self.peek() {
            Some(c) => c,
            None => return self.make(TokenKind::Eof),
        };

        let kind = match c {
            '.' => { self.bump(); TokenKind::Dot }
            '(' => { self.bump(); TokenKind::LParen }
            ')' => { self.bump(); TokenKind::RParen }
            '{' => { self.bump(); TokenKind::LBrace }
            '}' => { self.bump(); TokenKind::RBrace }
            ',' => { self.bump(); TokenKind::Comma }
            ';' => { self.bump(); TokenKind::Semicolon }
            '+' => { self.bump(); TokenKind::Plus }
            '-' => { self.bump(); TokenKind::Minus }
            '*' => { self.bump(); TokenKind::Star }
            '/' => { self.bump(); TokenKind::Slash }
            '=' => {
                if self.peek2('=') { self.bump2(); TokenKind::EqEq }
                else { self.bump(); TokenKind::Eq }
            }
            '!' if self.peek2('=') => { self.bump2(); TokenKind::Ne }
            '!' => { self.bump(); TokenKind::Bang }
            '>' => {
                if self.peek2('=') { self.bump2(); TokenKind::Ge }
                else { self.bump(); TokenKind::Gt }
            }
            '<' => {
                if self.peek2('=') { self.bump2(); TokenKind::Le }
                else { self.bump(); TokenKind::Lt }
            }
            '&' if self.peek2('&') => { self.bump2(); TokenKind::AndAnd }
            '|' if self.peek2('|') => { self.bump2(); TokenKind::OrOr }
            '"' => self.read_string(),
            c if c.is_ascii_digit() => self.read_number(),
            c if c.is_ascii_alphabetic() || c == '_' => self.read_ident(),
            _ => { self.bump(); return self.next_token(); }
        };

        self.last_token = Some(kind.clone());
        self.make(kind)
    }

    fn skip_ws_and_comments(&mut self) -> Option<TokenKind> {
        let mut newline = false;

        loop {
            match self.peek() {
                Some('\n') => {
                    self.pos += 1;
                    self.line += 1;
                    self.col = 1;
                    newline = true;
                }
                Some(c) if c.is_whitespace() => {
                    self.bump();
                }
                Some('#') => {
                    while let Some(c) = self.peek() {
                        self.pos += 1;
                        if c == '\n' {
                            self.line += 1;
                            self.col = 1;
                            newline = true;
                            break;
                        }
                    }
                }
                _ => break,
            }
        }

        if newline {
            if let Some(t) = &self.last_token {
                match t {
                    TokenKind::Ident(_)
                    | TokenKind::Number(_)
                    | TokenKind::Str(_)
                    | TokenKind::True
                    | TokenKind::False
                    | TokenKind::RParen => {
                        return Some(TokenKind::Semicolon);
                    }
                    _ => {}
                }
            }
        }

        None
    }

    fn make(&self, kind: TokenKind) -> Token {
        Token { kind, line: self.line, col: self.col }
    }

    fn bump(&mut self) {
        self.pos += 1;
        self.col += 1;
    }

    fn bump2(&mut self) {
        self.pos += 2;
        self.col += 2;
    }

    fn peek(&self) -> Option<char> {
        self.src.get(self.pos).copied()
    }

    fn peek2(&self, c: char) -> bool {
        self.src.get(self.pos + 1) == Some(&c)
    }

    fn read_string(&mut self) -> TokenKind {
        self.bump();
        let mut s = String::new();
        while let Some(c) = self.peek() {
            self.bump();
            if c == '"' { break; }
            s.push(c);
        }
        TokenKind::Str(s)
    }

    fn read_number(&mut self) -> TokenKind {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if !c.is_ascii_digit() { break; }
            s.push(c);
            self.bump();
        }
        TokenKind::Number(s.parse().unwrap())
    }

    fn read_ident(&mut self) -> TokenKind {
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if !(c.is_ascii_alphanumeric() || c == '_') { break; }
            s.push(c);
            self.bump();
        }
        match s.as_str() {
            "var" => TokenKind::Var,
            "def" => TokenKind::Def,
            "return" => TokenKind::Return,
            "if" => TokenKind::If,
            "elif" => TokenKind::Elif,
            "else" => TokenKind::Else,
            "loop" => TokenKind::Loop,
            "while" => TokenKind::While,
            "async" => TokenKind::Async,
            "await" => TokenKind::Await,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            _ => TokenKind::Ident(s),
        }
    }
}