use crate::{ast::{Expr, Node, Op}, token::{Token, TokenKind}};

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
        while !self.check(TokenKind::Eof) {
            nodes.push(self.stmt()?);
        }
        Ok(nodes)
    }

    fn err<T>(&self, msg: &str) -> Result<T, String> {
        let t = self.peek();
        Err(format!("line {}, column {}: {}", t.line, t.col, msg))
    }

    fn stmt(&mut self) -> Result<Node, String> {
        while self.match_tok(TokenKind::Semicolon) {}
        match &self.peek().kind {
            TokenKind::Var => self.var_decl(),
            TokenKind::If => self.if_stmt(),
            TokenKind::Loop => self.loop_stmt(),
            TokenKind::While => self.while_stmt(),
            TokenKind::Async => self.async_stmt(),
            TokenKind::Await => self.await_stmt(),
            TokenKind::Def => self.func_def(),
            TokenKind::Return => self.return_stmt(),
            TokenKind::Break => { self.advance(); self.match_tok(TokenKind::Semicolon); Ok(Node::Break) }
            TokenKind::Continue => { self.advance(); self.match_tok(TokenKind::Semicolon); Ok(Node::Continue) }
            TokenKind::Ident(_) => self.call_or_assign(),
            _ => self.err("Unexpected token"),
        }
    }

    fn async_stmt(&mut self) -> Result<Node, String> {
        self.advance();
        while self.match_tok(TokenKind::Semicolon) {}
        let body = self.block()?;
        Ok(Node::Async { body })
    }

    fn await_stmt(&mut self) -> Result<Node, String> {
        self.advance();
        self.expect(TokenKind::LParen)?;

        let key = match &self.advance().kind {
            TokenKind::Ident(s) => s.clone(),
            TokenKind::Number(n) => n.to_string(),
            _ => return self.err("Expected key identifier or number"),
        };

        self.expect(TokenKind::RParen)?;
        while self.match_tok(TokenKind::Semicolon) {}
        let body = self.block()?;
        Ok(Node::Await { key, body })
    }

    fn func_def(&mut self) -> Result<Node, String> {
        self.advance();
        let name = self.ident()?;
        self.expect(TokenKind::LParen)?;

        let mut params = Vec::new();
        if !self.check(TokenKind::RParen) {
            loop {
                let p = self.ident()?;
                let default = if self.match_tok(TokenKind::Eq) {
                    Some(self.expr()?)
                } else {
                    None
                };
                params.push((p, default));
                if !self.match_tok(TokenKind::Comma) {
                    break;
                }
            }
        }

        self.expect(TokenKind::RParen)?;
        let body = self.block()?;
        Ok(Node::Function { name, params, body })
    }

    fn return_stmt(&mut self) -> Result<Node, String> {
        self.advance();
        if self.match_tok(TokenKind::Semicolon) {
            Ok(Node::Return(None))
        } else {
            let v = self.expr()?;
            self.terminator()?;
            Ok(Node::Return(Some(v)))
        }
    }

    fn while_stmt(&mut self) -> Result<Node, String> {
        self.advance();
        self.expect(TokenKind::LParen)?;
        let cond = self.expr()?;
        self.expect(TokenKind::RParen)?;
        while self.match_tok(TokenKind::Semicolon) {}
        let body = self.block()?;
        Ok(Node::While { cond, body })
    }

    fn var_decl(&mut self) -> Result<Node, String> {
        self.advance();
        let name = self.ident()?;
        self.expect(TokenKind::Eq)?;
        let value = self.expr()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Node::VarDecl { name, value })
    }

    fn call_or_assign(&mut self) -> Result<Node, String> {
        let name = self.ident()?;
        if self.match_tok(TokenKind::Eq) {
            let value = self.expr()?;
            self.terminator()?;
            Ok(Node::Assign { name, value })
        } else {
            self.expect(TokenKind::LParen)?;
            let args = self.args()?;
            self.expect(TokenKind::RParen)?;
            self.terminator()?;
            Ok(Node::Call { name, args })
        }
    }

    fn terminator(&mut self) -> Result<(), String> {
        if self.match_tok(TokenKind::Semicolon) {
            return Ok(());
        }
        match self.peek().kind {
            TokenKind::RBrace | TokenKind::Else | TokenKind::Elif | TokenKind::Eof => Ok(()),
            _ => self.err("Expected semicolon"),
        }
    }

    fn if_stmt(&mut self) -> Result<Node, String> {
        self.expect(TokenKind::If)?;
        self.expect(TokenKind::LParen)?;
        let cond = self.expr()?;
        self.expect(TokenKind::RParen)?;
        while self.match_tok(TokenKind::Semicolon) {}
        let then_body = self.block()?;

        while self.match_tok(TokenKind::Semicolon) {}

        let else_body = if self.match_tok(TokenKind::Elif) {
            let elif_node = self.elif_chain()?;
            Some(vec![elif_node])
        } else if self.match_tok(TokenKind::Else) {
            Some(self.block()?)
        } else {
            None
        };

        Ok(Node::If { cond, then_body, else_body })
    }

    fn elif_chain(&mut self) -> Result<Node, String> {
        self.expect(TokenKind::LParen)?;
        let cond = self.expr()?;
        self.expect(TokenKind::RParen)?;
        while self.match_tok(TokenKind::Semicolon) {}
        let then_body = self.block()?;

        while self.match_tok(TokenKind::Semicolon) {}

        let else_body = if self.match_tok(TokenKind::Elif) {
            Some(vec![self.elif_chain()?])
        } else if self.match_tok(TokenKind::Else) {
            Some(self.block()?)
        } else {
            None
        };

        Ok(Node::If { cond, then_body, else_body })
    }

    fn loop_stmt(&mut self) -> Result<Node, String> {
        self.advance();
        self.expect(TokenKind::LParen)?;
        let times = self.expr()?;
        self.expect(TokenKind::RParen)?;
        while self.match_tok(TokenKind::Semicolon) {}
        let body = self.block()?;
        Ok(Node::Loop { times, body })
    }

    fn block(&mut self) -> Result<Vec<Node>, String> {
        self.expect(TokenKind::LBrace)?;
        let mut nodes = Vec::new();
        while !self.check(TokenKind::RBrace) && !self.check(TokenKind::Eof) {
            nodes.push(self.stmt()?);
        }
        self.expect(TokenKind::RBrace)?;
        Ok(nodes)
    }

    fn expr(&mut self) -> Result<Expr, String> {
        self.logic_or()
    }

    fn logic_or(&mut self) -> Result<Expr, String> {
        let mut e = self.logic_and()?;
        while self.match_tok(TokenKind::OrOr) {
            let r = self.logic_and()?;
            e = Expr::Binary(Box::new(e), Op::Or, Box::new(r));
        }
        Ok(e)
    }

    fn logic_and(&mut self) -> Result<Expr, String> {
        let mut e = self.equality()?;
        while self.match_tok(TokenKind::AndAnd) {
            let r = self.equality()?;
            e = Expr::Binary(Box::new(e), Op::And, Box::new(r));
        }
        Ok(e)
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut e = self.compare()?;
        loop {
            let op = match &self.peek().kind {
                TokenKind::EqEq => Op::Eq,
                TokenKind::Ne => Op::Ne,
                _ => break,
            };
            self.advance();
            let r = self.compare()?;
            e = Expr::Binary(Box::new(e), op, Box::new(r));
        }
        Ok(e)
    }

    fn compare(&mut self) -> Result<Expr, String> {
        let mut e = self.term()?;
        loop {
            let op = match &self.peek().kind {
                TokenKind::Gt => Op::Gt,
                TokenKind::Lt => Op::Lt,
                TokenKind::Ge => Op::Ge,
                TokenKind::Le => Op::Le,
                _ => break,
            };
            self.advance();
            let r = self.term()?;
            e = Expr::Binary(Box::new(e), op, Box::new(r));
        }
        Ok(e)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut e = self.factor()?;
        loop {
            let op = match &self.peek().kind {
                TokenKind::Plus => Op::Add,
                TokenKind::Minus => Op::Sub,
                _ => break,
            };
            self.advance();
            let r = self.factor()?;
            e = Expr::Binary(Box::new(e), op, Box::new(r));
        }
        Ok(e)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut e = self.unary()?;
        loop {
            let op = match &self.peek().kind {
                TokenKind::Star => Op::Mul,
                TokenKind::Slash => Op::Div,
                _ => break,
            };
            self.advance();
            let r = self.unary()?;
            e = Expr::Binary(Box::new(e), op, Box::new(r));
        }
        Ok(e)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.match_tok(TokenKind::Minus) {
            Ok(Expr::Unary(Op::Sub, Box::new(self.unary()?)))
        } else if self.match_tok(TokenKind::Bang) {
            Ok(Expr::Unary(Op::Not, Box::new(self.unary()?)))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, String> {
        match &self.advance().kind {
            TokenKind::Number(n) => Ok(Expr::Number(*n)),
            TokenKind::Str(s) => Ok(Expr::Str(s.clone())),
            TokenKind::True => Ok(Expr::Bool(true)),
            TokenKind::False => Ok(Expr::Bool(false)),
            TokenKind::Ident(s) => {
                let mut name = s.clone();
                while self.match_tok(TokenKind::Dot) {
                    if let TokenKind::Ident(n) = &self.advance().kind {
                        name.push('.');
                        name.push_str(n);
                    } else {
                        return self.err("Expected identifier");
                    }
                }
                if self.match_tok(TokenKind::LParen) {
                    let args = self.args()?;
                    self.expect(TokenKind::RParen)?;
                    Ok(Expr::Call { name, args })
                } else {
                    Ok(Expr::Var(name))
                }
            }
            TokenKind::LParen => {
                let e = self.expr()?;
                self.expect(TokenKind::RParen)?;
                Ok(e)
            }
            _ => self.err("Invalid expression"),
        }
    }

    fn args(&mut self) -> Result<Vec<Expr>, String> {
        let mut a = Vec::new();
        if !self.check(TokenKind::RParen) {
            a.push(self.expr()?);
            while self.match_tok(TokenKind::Comma) {
                a.push(self.expr()?);
            }
        }
        Ok(a)
    }

    fn ident(&mut self) -> Result<String, String> {
        match &self.advance().kind {
            TokenKind::Ident(s) => Ok(s.clone()),
            _ => self.err("Expected identifier"),
        }
    }

    fn expect(&mut self, k: TokenKind) -> Result<(), String> {
        if self.match_tok(k.clone()) {
            Ok(())
        } else {
            self.err(&format!("Expected {:?}", k))
        }
    }

    fn peek(&self) -> &Token {
        if self.pos >= self.tokens.len() {
            &self.tokens.last().unwrap()
        } else {
            &self.tokens[self.pos]
        }
    }

    fn advance(&mut self) -> &Token {
        let t = &self.tokens[self.pos];
        self.pos += 1;
        t
    }

    fn check(&self, k: TokenKind) -> bool {
        self.peek().kind == k
    }

    fn match_tok(&mut self, k: TokenKind) -> bool {
        if self.check(k) {
            self.pos += 1;
            true
        } else {
            false
        }
    }
}