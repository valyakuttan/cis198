extern crate rand;

use std::io;
use std::num;

use self::rand::distributions::{IndependentSample, Range};

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
/// An element of the stack. May be either integer or boolean.
pub enum Elt {
    Int(i32),
    Bool(bool),
}

#[derive(Debug)]
/// An RPN calculator error.
pub enum Error {
    /// Tried to pop from an empty stack.
    Underflow,
    /// Tried to operate on invalid types (e.g. 4 + true)
    Type,
    /// Unable to parse the input.
    Syntax(num::ParseIntError),
    /// Some IO error occurred.
    IO(io::Error),
    /// The user quit the program (with `quit`).
    Quit,
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error { Error::IO(err) }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Error { Error::Syntax(err) }
}

#[derive(Debug)]
/// Types of RPN calculator operations.
pub enum Op {
    /// Adds two numbers: pop x, pop y, push x + y.
    Add,
    /// Checks equality of two values: pop x, pop y, push x == y.
    Eq,
    /// Negates a value: pop x, push ~x.
    Neg,
    /// Swaps two values: pop x, pop y, push x, push y.
    Swap,
    /// Computes a random number: pop x, push random number in [0, x).
    Rand,
    /// Quit the calculator.
    Quit,
}

pub struct Stack {
    stack: Vec<Elt>
}

pub type Result<T> = ::std::result::Result<T, Error>;

impl Stack {
    /// Creates a new Stack
    pub fn new() -> Stack {
        Stack{ stack: Vec::new() }
    }

    /// Pushes a value onto the stack.
    pub fn push(&mut self, val: Elt) -> Result<()> {
        Ok(self.stack.push(val))
    }

    /// Tries to pop a value off of the stack.
    pub fn pop(&mut self) -> Result<Elt> {
        self.stack.pop().ok_or(Error::Underflow)
    }

    /// Tries to evaluate an operator using values on the stack.
    pub fn eval(&mut self, op: Op) -> Result<()> {
         match op {
            Op::Add => {
                let x = try!(self.pop());
                let y = try!(self.pop());
                match (x, y) {
                    (Elt::Int(a), Elt::Int(b)) =>
                        self.push(Elt::Int(a +b)),
                    _ => Err(Error::Type)
                }
            }

             Op::Eq => {
                 let x = try!(self.pop());
                 let y = try!(self.pop());

                 if x == y {
                     self.push(Elt::Bool(true))
                 } else {
                     self.push(Elt::Bool(false))
                 }
             }

             Op::Neg => {
                 let x = try!(self.pop());
                 match x {
                     Elt::Int(a) => self.push(Elt::Int(-a)),
                     Elt::Bool(b) => self.push(Elt::Bool(!b))
                 }
             }

             Op::Swap => {
                 let x = try!(self.pop());
                 let y = try!(self.pop());

                 try!(self.push(x));
                 self.push(y)
             }

             Op::Rand => {
                 let x = try!(self.pop());

                 match x {
                     Elt::Int(a) => {
                         let between = Range::new(0, a);
                         let mut rng = rand::thread_rng();
                         self.push(Elt::Int(
                             between.ind_sample(&mut rng)))
                     }

                     _ => Err(Error::Type)
                 }
             }
             
             Op::Quit => Err(Error::Quit)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pop_empty1() {
        let mut s = Stack::new();

        let res = s.pop();
        assert!(res.is_err());
        if let Err(Error::Underflow) = res { } else { assert!(false); }
    }

    #[test]
    fn test_pop_empty2() {
        let mut s = Stack::new();
        s.push(Elt::Int(0)).unwrap();

        let res = s.pop();
        assert!(res.is_ok());

        let res = s.pop();
        assert!(res.is_err());
        if let Err(Error::Underflow) = res { } else { assert!(false); }
    }

    #[test]
    fn test_eval_add1() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Int(1)).unwrap();

        assert!(s.eval(Op::Add).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Int(2));
    }

    #[test]
    fn test_eval_add2() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Add);
        assert!(res.is_err());
        if let Err(Error::Type) = res { } else { assert!(false); }
    }

    #[test]
    fn test_eval_add3() {
        let mut s = Stack::new();
        s.push(Elt::Bool(true)).unwrap();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Add);
        assert!(res.is_err());
        if let Err(Error::Type) = res { } else { assert!(false); }
    }

    #[test]
    fn test_eval_eq1() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Int(1)).unwrap();

        assert!(s.eval(Op::Eq).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Bool(true));
    }

    #[test]
    fn test_eval_eq2() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Add);
        assert!(res.is_err());
        if let Err(Error::Type) = res { } else { assert!(false); }
    }

    #[test]
    fn test_eval_neg1() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        assert!(s.eval(Op::Neg).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Int(-1));
    }

    #[test]
    fn test_eval_neg2() {
        let mut s = Stack::new();
        s.push(Elt::Bool(false)).unwrap();
        assert!(s.eval(Op::Neg).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Bool(true));
    }

    #[test]
    fn test_eval_swap1() {
        let mut s = Stack::new();
        s.push(Elt::Int(1)).unwrap();
        s.push(Elt::Bool(false)).unwrap();

        assert!(s.eval(Op::Swap).is_ok());
        assert_eq!(s.pop().unwrap(), Elt::Int(1));
        assert_eq!(s.pop().unwrap(), Elt::Bool(false));

        let res = s.pop();
        assert!(res.is_err());
        if let Err(Error::Underflow) = res { } else { assert!(false); }
    }

    #[test]
    fn test_eval_swap2() {
        let mut s = Stack::new();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Swap);
        assert!(res.is_err());
        if let Err(Error::Underflow) = res { } else { assert!(false); }
    }

    #[test]
    fn test_eval_rand1() {
        let mut s = Stack::new();
        let i = 20;
        s.push(Elt::Int(i)).unwrap();

        assert!(s.eval(Op::Rand).is_ok());

        let rand_val = s.pop().unwrap();
        assert!(rand_val >= Elt::Int(0));
        assert!(rand_val < Elt::Int(i));
    }

    #[test]
    fn test_eval_rand2() {
        let mut s = Stack::new();
        s.push(Elt::Bool(false)).unwrap();

        let res = s.eval(Op::Rand);
        assert!(res.is_err());
        if let Err(Error::Type) = res { } else { assert!(false); }
    }

    #[test]
    fn test_eval_quit() {
        let mut s = Stack::new();

        let res = s.eval(Op::Quit);
        assert!(res.is_err());
        if let Err(Error::Quit) = res { } else { assert!(false); }
    }
}
