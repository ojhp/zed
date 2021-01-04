use std::process;

use zed::Repl;

fn main() {
    let mut repl = Repl::interactive();
    if let Err(_) = repl.repl() {
        process::exit(1);
    }
}