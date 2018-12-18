use std::collections::HashMap;
use crate::ast::Ast;

#[derive(Debug)]
pub struct Forth {
    stack: Vec<i64>,
    dictionary: HashMap<String, Vec<Ast>>,
    // This is a little goofy. Maybe exec should take some kind of context value
    loop_i: Option<i64>,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: vec![],
            dictionary: HashMap::new(),
            loop_i: None,
        }
    }
    pub fn exec(&mut self, ast: Ast) -> Result<(), String> {
        match ast {
            Ast::Phrase(items) => {
                for item in items {
                    match self.exec(item) {
                        Ok(()) => {}
                        Err(msg) => println!("{}", msg)
                    }
                };
                Ok(())
            },
            Ast::Push(value) => self.push(value),
            Ast::DotQuote(value) => {
                println!("{}", value);
                Ok(())
            },
            Ast::Definition(phrase) => {
                if let Ast::Phrase(items) = *phrase {
                    if let Ast::Word(name) = items.get(0).unwrap() {
                        let mut items = items.clone();
                        items.remove(0);
                        self.compile(name, &items)
                    } else {
                        Err(format!("Could unwrap name from {:?}", items))
                    }
                } else {
                    Err(format!("Could not compile {:?}", phrase))
                }
            },
            Ast::Conditional { consequent: c, alternative: a } => {
                if let Some(v) = self.stack.pop() {
                    let phrase = if v == -1 {
                        Some(c)
                    } else {
                        a
                    };
                    if let Some(box Ast::Phrase(phrase)) = phrase {
                        self.exec(Ast::Phrase(phrase.clone()));
                    };
                    Ok(())
                } else {
                    Err("Didn't have a bool at top of stack".to_string())
                }
            },
            Ast::DoLoop(ast) => {
                if let Ast::Phrase(phrase) = *ast {
                    let start = self.stack.pop().unwrap();
                    let end = self.stack.pop().unwrap();
                    for n in start..end {
                        self.loop_i = Some(n);
                        self.exec(Ast::Phrase(phrase.clone()));
                    }
                    self.loop_i = None;
                    Ok(())
                } else {
                    Err("Did not have phrase to exec doloop".to_string())
                }
            },
            Ast::Word(token) => {
                match token.as_ref() {
                    "*" => self.op_two("*", &|v1, v2| v2 * v1),
                    "/" => self.op_two("/", &|v1, v2| v2 / v1),
                    "+" => self.op_two("+", &|v1, v2| v2 + v1),
                    "-" => self.op_two("-", &|v1, v2| v2 - v1),
                    "=" => self.op_two("=", &|v1, v2| if v1 == v2 { -1 } else { 0 }),
                    "mod" => self.op_two("mod", &|v1, v2| v2 % v1),
                    "and" => self.op_two("and", &|v1, v2| if *v2 == -1 && *v1 == -1 { -1 } else { 0 }),
                    "or" => self.op_two("or", &|v1, v2| if *v2 == -1 || *v1 == -1 { -1 } else { 0 }),
                    "invert" => self.op("invert", &|v| if v != -1 { -1 } else { 0 }),
                    "dup" => self.dup(),
                    "swap" => self.swap(),
                    "clear" => self.clear(),
                    "." => self.pop(),
                    ".s" => self.print(),
                    "" => Ok(()),
                    _ => {
                        // if we are in a loop, "i" should resolve to the loop counter
                        if let Some(loop_i) = self.loop_i {
                            if token == "i" {
                                return self.exec(Ast::Push(loop_i))
                            }
                        }
                        if let Some(words) = self.dictionary.get(&token) {
                            self.exec(Ast::Phrase(words.clone()));
                            Ok(())
                        } else {
                            Err(format!("Unknown word {:?}", token))
                        }
                    }
                }
            },
        }
    }
    fn compile(&mut self, name: &str, phrase: &Vec<Ast>) -> Result<(), String> {
        self.dictionary.insert(String::from(name), phrase.clone());
        Ok(())
    }
    fn push(&mut self, item: i64) -> Result<(), String> {
        self.stack.push(item);
        Ok(())
    }
    fn pop(&mut self) -> Result<(), String>{
        let popped = self.stack.pop().unwrap();
        println!("{:?}", popped);
        Ok(())
    }
    fn print(&mut self) -> Result<(), String>{
        println!("{:?}", self.stack);
        Ok(())
    }
    fn dup(&mut self) -> Result<(), String> {
        if let Some(last) = self.stack.last() {
            self.stack.push(last.clone());
        }
        Ok(())
    }
    fn clear(&mut self) -> Result<(), String> {
        self.stack.clear();
        Ok(())
    }
    fn swap(&mut self) -> Result<(), String> {
        let top_one = self.stack.pop().unwrap();
        let top_two = self.stack.pop().unwrap();
        self.stack.push(top_one);
        self.stack.push(top_two);
        Ok(())
    }
    fn op_two(&mut self, op_name: &str, op: &Fn(&i64, &i64) -> i64) -> Result<(), String>{
        let top_one = &self.stack.pop().unwrap();
        let top_two = &self.stack.pop().unwrap();
        self.stack.push(op(top_one, top_two));
        Ok(())
    }
    fn op(&mut self, op_name: &str, op: &Fn(i64) -> i64) -> Result<(), String>{
        let v = self.stack.pop().unwrap();
        self.stack.push(op(v));
        Ok(())
    }
}
