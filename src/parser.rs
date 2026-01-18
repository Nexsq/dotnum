use crate::{ast::{Expr, Node, Op}, token::Token};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Node>, String> {
        let mut nodes = Vec::new();
        while !self.check(Token::Eof) {
            nodes.push(self.stmt()?);
        }
        Ok(nodes)
    }

    fn stmt(&mut self) -> Result<Node, String> {
        while self.match_tok(Token::Semicolon) {}

        match self.peek() {
            Token::Var => self.var_decl(),
            Token::If => self.if_stmt(),
            Token::Loop => self.loop_stmt(),
            Token::Ident(_) => self.call_or_assign(),
            t => Err(format!("Unexpected token {:?}", t)),
        }
    }

    fn var_decl(&mut self) -> Result<Node, String> {
        self.advance();
        let name = self.ident()?;
        self.expect(Token::Eq)?;
        let value = self.expr()?;
        self.expect(Token::Semicolon)?;
        Ok(Node::VarDecl { name, value })
    }

    fn call_or_assign(&mut self) -> Result<Node, String> {
        let name = self.ident()?;
        if self.match_tok(Token::Eq) {
            let value = self.expr()?;
            self.expect(Token::Semicolon)?;
            Ok(Node::Assign { name, value })
        } else {
            self.expect(Token::LParen)?;
            let args = self.args()?;
            self.expect(Token::RParen)?;
            self.expect(Token::Semicolon)?;
            Ok(Node::Call { name, args })
        }
    }

    fn if_stmt(&mut self) -> Result<Node, String> {
        self.advance();
        self.expect(Token::LParen)?;
        let cond = self.expr()?;
        self.expect(Token::RParen)?;
        let then_body = self.block()?;
        let else_body = if self.match_tok(Token::Else) {
            Some(self.block()?)
        } else {
            None
        };
        Ok(Node::If { cond, then_body, else_body })
    }

    fn loop_stmt(&mut self) -> Result<Node, String> {
        self.advance();
        self.expect(Token::LParen)?;
        let times = self.expr()?;
        self.expect(Token::RParen)?;
        let body = self.block()?;
        Ok(Node::Loop { times, body })
    }

    fn block(&mut self) -> Result<Vec<Node>, String> {
        self.expect(Token::LBrace)?;
        let mut nodes = Vec::new();
        while !self.check(Token::RBrace) && !self.check(Token::Eof) {
            nodes.push(self.stmt()?);
        }
        self.expect(Token::RBrace)?;
        Ok(nodes)
    }

    fn expr(&mut self) -> Result<Expr, String> {
        self.logic_or()
    }

    fn logic_or(&mut self) -> Result<Expr, String> {
        let mut e = self.logic_and()?;
        while self.match_tok(Token::OrOr) {
            let r = self.logic_and()?;
            e = Expr::Binary(Box::new(e), Op::Or, Box::new(r));
        }
        Ok(e)
    }

    fn logic_and(&mut self) -> Result<Expr, String> {
        let mut e = self.equality()?;
        while self.match_tok(Token::AndAnd) {
            let r = self.equality()?;
            e = Expr::Binary(Box::new(e), Op::And, Box::new(r));
        }
        Ok(e)
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut e = self.compare()?;
        while let Some(op) = match self.peek() {
            Token::EqEq => Some(Op::Eq),
            Token::Ne => Some(Op::Ne),
            _ => None,
        } {
            self.advance();
            let r = self.compare()?;
            e = Expr::Binary(Box::new(e), op, Box::new(r));
        }
        Ok(e)
    }

    fn compare(&mut self) -> Result<Expr, String> {
        let mut e = self.primary()?;
        while let Some(op) = match self.peek() {
            Token::Gt => Some(Op::Gt),
            Token::Lt => Some(Op::Lt),
            Token::Ge => Some(Op::Ge),
            Token::Le => Some(Op::Le),
            _ => None,
        } {
            self.advance();
            let r = self.primary()?;
            e = Expr::Binary(Box::new(e), op, Box::new(r));
        }
        Ok(e)
    }

    fn primary(&mut self) -> Result<Expr, String> {
        match self.advance() {
            Token::Number(n) => Ok(Expr::Number(n)),
            Token::Str(s) => Ok(Expr::Str(s)),
            Token::Ident(s) => Ok(Expr::Var(s)),
            Token::LParen => {
                let e = self.expr()?;
                self.expect(Token::RParen)?;
                Ok(e)
            }
            t => Err(format!("Unexpected token {:?}", t)),
        }
    }

    fn args(&mut self) -> Result<Vec<Expr>, String> {
        let mut a = Vec::new();
        if !self.check(Token::RParen) {
            a.push(self.expr()?);
            while self.match_tok(Token::Comma) {
                a.push(self.expr()?);
            }
        }
        Ok(a)
    }

    fn ident(&mut self) -> Result<String, String> {
        if let Token::Ident(s) = self.advance() {
            Ok(s)
        } else {
            Err("Expected identifier".into())
        }
    }

    fn expect(&mut self, t: Token) -> Result<(), String> {
        if self.match_tok(t.clone()) {
            Ok(())
        } else {
            Err(format!("Expected {:?}, got {:?}", t, self.peek()))
        }
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.pos).cloned().unwrap_or(Token::Eof)
    }

    fn advance(&mut self) -> Token {
        let t = self.peek();
        self.pos += 1;
        t
    }

    fn check(&self, t: Token) -> bool {
        self.peek() == t
    }

    fn match_tok(&mut self, t: Token) -> bool {
        if self.check(t.clone()) {
            self.advance();
            true
        } else {
            false
        }
    }
}