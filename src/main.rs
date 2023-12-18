/*
 * this is some sort of my own implemented DC command (see `man dc`)
 */

mod stack;

type number_type = f64;

/* available commands (inputs from user) */
enum Command {
    NUMBER(number_type),
    ADDITION,
    SUBTRACTION,
    MULTIPLICATION,
    DIVISION
}

fn main() {
    let comp_stack = 
}
