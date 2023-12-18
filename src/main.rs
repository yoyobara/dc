/*
 * this is some sort of my own implemented DC command (see `man dc`)
 */

use std::io::Read;

use stack::Stack;

mod stack;

type NumberType = f64;

/* available commands (inputs from user) */
enum Op {
    NUMBER(NumberType),
    ADDITION,
    SUBTRACTION,
    MULTIPLICATION,
    DIVISION
}

/*
/* returns the next option to process from a given stream */
fn parse_operations<T: Read>(mut stream: T) -> Result<Vec<Op>, &'static str> {
    let mut operations: Vec<Op> = Vec::new();
    let mut num_buff = String::new();

    loop {
        // get next char
        let mut buff = [0u8; 1];
        stream.read_exact(&mut buff);
        let next_char = buff[0] as char;

        // match it
        match next_char {
            '0'..='9' => {
                num_buff.push(next_char);
            }

            '.' => {
                if num_buff.contains('.') {
                    return Err("error parsing number")
                } else {
                    num_buff.push('.');
                }
            }

            '+' => {
                if !num_buff.is_empty() {
                    operations.push(Op::NUMBER(num_buff.parse().unwrap()));
                    num_buff.clear();
                }

                if 
            }


        }

    }
}
*/

fn main() {
    let mut comp_stack: Stack<NumberType> = Stack::new();
    comp_stack.push(5.0);
}
