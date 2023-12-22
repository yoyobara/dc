use std::io::Read; 
use crate::utils::read_byte;

pub type NumberType = f64;

/* available commands (inputs from user) */
#[derive(Clone, Copy)]
pub enum Op {
    NUMBER(NumberType),
    ADDITION,
    SUBTRACTION,
    MULTIPLICATION,
    DIVISION
}

/* a struct created for parsing the input in real time */
struct OpParser<R: Read> {
    in_stream: R,
    num_buf: String,
    pending: Option<Op>
}

impl<R: Read> OpParser<R> {
    pub fn new(in_stream: R) -> OpParser<R> {
        OpParser { in_stream, num_buf: String::new(), pending: None }
    }

    /* returns the next operation */
    pub fn next(&mut self) -> Result<Op, &str> {
        if self.pending.is_some() {
            return Ok(self.pending.take().unwrap());
        }

        match read_byte(&mut self.in_stream) {
            Some(b'0'..=b'9') => {

            }

            None => todo!()
        }

        Ok(Op::NUMBER(f64::NAN))
    }
}
