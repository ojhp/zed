use std::io::{stdout, Stdout};

use crate::{ZedError, ZedResult};
use crate::eval::{Eval, ExprEvaluator};
use crate::print::{Print, ReplPrinter};
use crate::read::{ExprReader, Read, ReadError, ReplTextReader};

pub struct Repl<R: Read, E: Eval<<R as Read>::Output>, P: Print<<E as Eval<<R as Read>::Output>>::Output>> {
    reader: R,
    evaluator: E,
    printer: P,
}

impl<R: Read, E: Eval<<R as Read>::Output>, P: Print<<E as Eval<<R as Read>::Output>>::Output>> Repl<R, E, P> {
    pub fn new(reader: R, evaluator: E, printer: P) -> Self {
        Repl { reader, evaluator, printer }
    }

    pub fn rep(&mut self) -> ZedResult<()> {
        self.printer.print(self.evaluator.eval(self.reader.read()?)?)?;
        Ok(())
    }

    pub fn repl(&mut self) -> ZedResult<()> {
        use ZedError::*;
        use ReadError::*;

        loop {
            if let Err(err) = self.rep() {
                match err {
                    Read(Eof) => return Ok(()),
                    Read(Parse(_)) | Eval(_) => {
                        self.reader.clear_buffer();
                        eprintln!("ERROR: {}", err);
                    },
                    _ => {
                        eprintln!("ERROR: {}", err);
                        return Err(err);
                    },
                }
            }
        }
    }
}

impl Repl<ExprReader<ReplTextReader>, ExprEvaluator, ReplPrinter<Stdout>> {
    pub fn interactive() -> Self {
        Repl {
            reader: ExprReader::new(ReplTextReader::new()),
            evaluator: ExprEvaluator,
            printer: ReplPrinter::new(stdout()),
        }
    }
}
