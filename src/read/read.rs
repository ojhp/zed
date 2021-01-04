use crate::read::ReadResult;

pub trait Read {
    type Output;

    fn read(&mut self) -> ReadResult<Self::Output>;
}
