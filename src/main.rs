#![feature(box_patterns)]
#[macro_use]
extern crate lalrpop_util;
use std::io;
pub mod ast;
mod vm;

lalrpop_mod!(pub parser);

fn main() -> io::Result<()> {
    let mut ev = vm::Forth::new();

    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer)?;
        let input = buffer.trim_right();
        if input == "quit" {
            break
        }
        let expr = parser::PhraseParser::new()
            .parse(input)
            .unwrap();
        ev.exec(expr);
        //println!("AST: {:?}", expr);
    }

    Ok(())
}

