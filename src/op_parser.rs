use std::io::Read; 
use crate::utils::read_byte;

pub type NumberType = f64;

/* available commands (inputs from user) */
pub enum Op {
    NUMBER(NumberType),
    ADDITION,
    SUBTRACTION,
    MULTIPLICATION,
    DIVISION,
    EOF
}

impl Op {
    fn from_char(ch: char) -> Option<Op> {
        match ch {
            '+' => Some(Op::ADDITION),
            '-' => Some(Op::SUBTRACTION),
            '*' => Some(Op::MULTIPLICATION),
            '/' => Some(Op::DIVISION),
            _ => None
        }
    }
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
    pub fn next(&mut self) -> Result<Op, String> {
        if self.pending.is_some() {
            return Ok(self.pending.take().unwrap());
        }

        loop {
            match read_byte(&mut self.in_stream).map(|b| b as char) {
                Some(ch @ '0'..='9') => {
                    self.num_buf.push(ch)
                }

                Some('.') if !self.num_buf.contains('.') => {
                    self.num_buf.push('.')
                }

                Some('\n' | ' ') => {
                    if !self.num_buf.is_empty() {
                        let f: NumberType = self.num_buf.parse().unwrap();
                        self.num_buf.clear();
                        return Ok(Op::NUMBER(f))
                    }
                }

                Some(arithmatic @ ('+'|'-'|'*'|'/')) => {
                    let operation = Op::from_char(arithmatic).unwrap();

                    if self.num_buf.is_empty() {
                        return Ok(operation)
                    } else {
                        self.pending = Some(operation);

                        let f: NumberType = self.num_buf.parse().unwrap();
                        self.num_buf.clear();
                        return Ok(Op::NUMBER(f))
                    }
                }

                Some(c) => return Err(format!("unknown char: {}", c)),

                None => return Ok(Op::EOF)
            }
        }
    }
}
