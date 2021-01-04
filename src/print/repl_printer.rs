use std::io::Write;
use std::fmt::Display;

use crate::print::{Print, PrintResult};

pub struct ReplPrinter<T: Write> {
    output: T
}

impl<T: Write> ReplPrinter<T> {
    pub fn new(output: T) -> Self {
        ReplPrinter { output }
    }
}

impl<O: Write, T: Display> Print<T> for ReplPrinter<O> {
    fn print(&mut self, value: T) -> PrintResult<()> {
        writeln!(self.output, "---> {}", value)?;
        Ok(())
    }
}
