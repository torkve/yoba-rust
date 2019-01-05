use std::io::Write;
use std::collections::hash_map::{HashMap, Entry};

use crate::grammar::{Statement, Expression};
use rug::Integer;

pub struct State<'a, T: Write> {
    pub variables: HashMap<&'a str, Integer>,
    pub functions: HashMap<&'a str, &'a Vec<Statement<'a>>>,
    pub out: T,
}

pub type Result = std::result::Result<(), String>;

impl<'a> State<'a, Vec<u8>> {
    pub fn new() -> Self {
        State {
            variables: HashMap::new(),
            functions: HashMap::new(),
            out: Vec::new(),
        }
    }
}

impl<'a, T: Write> State<'a, T> {
    pub fn new_with_out(out: T) -> Self {
        State {
            variables: HashMap::new(),
            functions: HashMap::new(),
            out: out,
        }
    }

    fn eval_arithmetics(&self, expr: &Expression<'a>) -> std::result::Result<Integer, String> {
        match expr {
            Expression::Integer(x) => Ok(x.clone()),
            Expression::Literal(x) => {
                match self.variables.get(x) {
                    None => Err(format!("не пасу чо за {}", x)),
                    Some(val) => Ok(val.clone())
                }
            },
            Expression::Add(left, right) => Ok(self.eval_arithmetics(&*left)? + self.eval_arithmetics(&*right)?),
            Expression::Sub(left, right) => Ok(self.eval_arithmetics(&*left)? - self.eval_arithmetics(&*right)?),
        }
    }

    fn define_variable(&mut self, x: &'a str) -> Result {
        match self.variables.entry(x) {
            Entry::Occupied(_) => Err(format!("я уже знаю за {}", x)),
            entry => {
                entry.or_insert_with(Integer::new);
                Ok(())
            }
        }
    }

    fn assign_variable(&mut self, x: &'a str, expr: &Expression<'a>, incremental: bool) -> Result {
        let result = self.eval_arithmetics(expr)?;
        match self.variables.entry(x) {
            Entry::Vacant(_) => Err(format!("не пасу чо за {}", x)),
            Entry::Occupied(e) => {
                if incremental {
                    *e.into_mut() += result;
                } else {
                    *e.into_mut() = result;
                }
                Ok(())
            }
        }
    }

    fn print_variable(&mut self, x: &'a str) -> Result {
        match self.variables.entry(x) {
            Entry::Vacant(_) => Err(format!("не пасу чо за {}", x)),
            Entry::Occupied(e) =>
                write!(self.out, "{}: {}\n", x, e.get()).map_err(|err| format!("{}", err))
        }
    }

    fn print_stats(&mut self) -> Result {
        for (k, v) in self.variables.iter() {
            write!(self.out, "{}: {}", k, v).map_err(|err| format!("{}", err))?
        }
        Ok(())
    }
    
    fn condition(&mut self, x: &'a str, val: Integer, cond_if: &'a Statement<'a>, cond_else: &'a Statement<'a>) -> Result {
        match self.variables.entry(x) {
            Entry::Vacant(_) => Err(format!("не пасу чо за {}", x)),
            Entry::Occupied(e) => {
                if *e.get() >= val {
                    self.eval_statement(cond_if)
                } else {
                    self.eval_statement(cond_else)
                }
            }
        }
    }

    fn declare_function(&mut self, x: &'a str, body: &'a Vec<Statement<'a>>) -> Result {
        match self.functions.entry(x) {
            Entry::Occupied(_) => Err(format!("я ващет уже усёк за {}", x)),
            Entry::Vacant(e) => {
                e.insert(body);
                Ok(())
            },
        }
    }

    fn call_function(&mut self, x: &'a str) -> Result {
        match self.functions.get(x) {
            None => Err(format!("я чот не усёк за {}", x)),
            Some(e) => self.eval(&e[..])
        }
    }

    fn eval_statement(&mut self, statement: &'a Statement<'a>) -> Result {
        match statement {
            Statement::Noop => Ok(()),
            Statement::Stats => self.print_stats(),
            Statement::Define(x) => self.define_variable(x),
            Statement::Assign(x, expr) => self.assign_variable(x, expr, false),
            Statement::ModAssign(x, expr) => self.assign_variable(x, expr, true),
            Statement::Print(x) => self.print_variable(x),
            Statement::Call(x) => self.call_function(x),
            Statement::Condition(x, val, cond_if, cond_else) => self.condition(x, val.clone(), &*cond_if, &*cond_else),
            Statement::Function(x, body) => self.declare_function(x, body),
        }
    }
    pub fn eval(&mut self, program: &'a [Statement<'a>]) -> Result {
        for statement in program {
            self.eval_statement(statement)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::tests::FIBONACCI;
    use crate::grammar::parse_program;

    #[test]
    fn test_fibonacci_program() {
        let program = parse_program(FIBONACCI).unwrap();
        let mut state = State::new();
        state.eval(&program[..]).unwrap();
        assert_eq!(&state.out[..], "итерации: 50\nсэмки: 53316291173\n".as_bytes());
    }
}
