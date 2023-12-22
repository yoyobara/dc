/*
 * this is some sort of my own implemented DC command (see `man dc`)
 */

mod utils;
mod list;
mod stack;
mod op_parser;

use std::io::stdin;

use op_parser::OpParser;

fn main() {
    let mut parser = OpParser::new(stdin());
    loop {
        println!("op is: {:?}", parser.next());
    }
}
