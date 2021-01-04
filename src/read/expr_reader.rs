use crate::data::Expr;
use crate::read::{parse_exprs, Read, ReadResult};

pub struct ExprReader<T: Read<Output=String>> {
    input: T,
    buffer: Vec<Expr>,
}

impl<T: Read<Output=String>> ExprReader<T> {
    pub fn new(input: T) -> Self {
        let buffer = Vec::new();

        ExprReader { input, buffer }
    }
}

impl<T: Read<Output=String>> Read for ExprReader<T> {
    type Output = Expr;

    fn read(&mut self) -> ReadResult<Expr> {
        loop {
            if let Some(e) = self.buffer.pop() {
                return Ok(e);
            }

            let text = self.input.read()?;
            let exprs = parse_exprs(&text)?;
            self.buffer = exprs.into_iter().rev().collect();
        }
    }
}
