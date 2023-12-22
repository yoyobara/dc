/*
 * this is some sort of my own implemented DC command (see `man dc`)
 */

mod utils;
mod list;
mod stack;
mod op_parser;

use std::{io::stdin, fmt::Debug};

use op_parser::{Op, OpParser};
use stack::Stack;

fn main() -> Result<(), String>{
    let mut parser = OpParser::new(stdin());
    let mut op_stack: Stack<Op> = Stack::new();

    loop {
        let next = parser.next()?;
        match next {
            Op::NUMBER(..) => {
                op_stack.push(next);
            }

            arithmatic @ (Op::ADDITION | Op::SUBTRACTION | Op::DIVISION | Op::MULTIPLICATION)=> {
                let arg2 = op_stack.pop().unwrap();
                let arg1 = op_stack.pop().unwrap();

                op_stack.push(arithmatic.perform_arithmatic_op(arg1, arg2).unwrap());
            }

            Op::EOF => {
                break;
            }
        }

        println!("stack is: {:?}", op_stack);
    }
    Ok(())
}
