use std::io::Read;

pub type NumberType = f64;

/* available commands (inputs from user) */
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
        
    }
}
