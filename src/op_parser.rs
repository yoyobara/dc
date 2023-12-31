use std::io::Read; 
use crate::utils::read_byte;

pub type NumberType = f64;

/* available commands (inputs from user) */
#[derive(Debug)]
pub enum Op {
    NUMBER(NumberType),
    ADDITION,
    SUBTRACTION,
    MULTIPLICATION,
    DIVISION,
    TOP,
    CLEAR,
    EOF
}

impl Op {
    fn from_char(ch: char) -> Option<Op> {
        match ch {
            '+' => Some(Op::ADDITION),
            '-' => Some(Op::SUBTRACTION),
            '*' => Some(Op::MULTIPLICATION),
            '/' => Some(Op::DIVISION),
            'p' => Some(Op::TOP),
            'c' => Some(Op::CLEAR),
            _ => None
        }
    }

    /* performs arithmatics with number ops */
    pub fn perform_arithmatic_op(self, arg1: Op, arg2: Op) -> Option<Op> {

        if let (Op::NUMBER(f1), Op::NUMBER(f2)) = (arg1, arg2) {
            Some(Op::NUMBER(
                match self {
                    Op::ADDITION => f1 + f2,
                    Op::SUBTRACTION => f1 - f2,
                    Op::MULTIPLICATION => f1 * f2,
                    Op::DIVISION => f1 / f2,
                    _ => return None
                }
            ))

        } else {
            None
        }
    }
}

/* a struct created for parsing the input in real time */
pub struct OpParser<R: Read> {
    in_stream: R,
    num_buf: String,
    pending: Option<Op>
}

impl<R: Read> OpParser<R> {
    pub fn new(in_stream: R) -> OpParser<R> {
        OpParser { in_stream, num_buf: String::new(), pending: None }
    }

    fn ret_clear_numbuf(&mut self) -> Result<Op, String> {
        let f: NumberType = self.num_buf.parse().map_err(|e| format!("bad parse: {}", e))?;
        self.num_buf.clear();
        Ok(Op::NUMBER(f))
    }

    /* returns the next operation */
    pub fn next(&mut self) -> Result<Op, String> {
        if self.pending.is_some() {
            return Ok(self.pending.take().unwrap());
        }

        loop {
            match read_byte(&mut self.in_stream).map(|b| b as char) {
                Some(ch @ ('0'..='9' | '.')) => {
                    self.num_buf.push(ch)
                }

                Some('\n' | ' ') => {
                    if !self.num_buf.is_empty() {
                        return self.ret_clear_numbuf();
                    }
                }

                Some(operation @ ('+'|'-'|'*'|'/'|'p'|'c')) => {
                    let operation = Op::from_char(operation).unwrap();

                    if self.num_buf.is_empty() {
                        return Ok(operation)
                    } else {
                        self.pending = Some(operation);
                        return self.ret_clear_numbuf();
                    }
                }

                Some(c) => return Err(format!("unknown char: {}", c)),

                None => return Ok(Op::EOF)
            }
        }
    }
}
