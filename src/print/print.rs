use crate::print::PrintResult;

pub trait Print<T> {
    fn print(&mut self, value: T) -> PrintResult<()>;
}
