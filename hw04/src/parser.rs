use std::io::{self, Write};
use std::str::FromStr;

use rpn::{self, Stack};

/// Start a read-eval-print loop, which runs until an error or `quit`.
pub fn read_eval_print_loop() -> rpn::Result<()> {
    // Create a stack to work on.
    let mut stack = Stack::new();
    
    loop {
        // Print a user input prompt.
        print!("> ");
        try!(io::stdout().flush());

        let mut buf = String::new();
        try!(io::stdin().read_line(&mut buf));

        try!(evaluate_line(&mut stack, &buf));
    
        let result = try!(stack.pop());
        match result {
            rpn::Elt::Bool(b) => {
                println!("> {}", b);
                try!(stack.push(rpn::Elt::Bool(b)));
            }
            
            rpn::Elt::Int(i)  => {
                println!("> {}", i);
                try!(stack.push(rpn::Elt::Int(i)));
            }
        }
    }
}

pub fn evaluate_line(stack: &mut Stack, buf: &str) -> rpn::Result<()> {
    // Create an iterator over the tokens.
    let tokens = buf.trim().split_whitespace();

    for token in tokens {
        match token {
            "true"  => try!(stack.push(rpn::Elt::Bool(true))),

            "false" => try!(stack.push(rpn::Elt::Bool(false))),
                           
            "+"     => try!(stack.eval(rpn::Op::Add)),

            "="     => try!(stack.eval(rpn::Op::Eq)),

            "~"     => try!(stack.eval(rpn::Op::Neg)),

            "<->"   => try!(stack.eval(rpn::Op::Swap)),

            "#"     => try!(stack.eval(rpn::Op::Rand)),

            "quit"  => try!(stack.eval(rpn::Op::Quit)),
            
            x       => {
                let num = try!(i32::from_str(x));
                try!(stack.push(rpn::Elt::Int(num)))
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use rpn::{Stack, Error, Elt};
    use parser::evaluate_line;

    #[test]
    fn test_evaluate_line_bool() {
        let mut stack = Stack::new();
        let s = "true".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        assert_eq!(stack.pop().unwrap(), Elt::Bool(true));
        let s = "false".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        assert_eq!(stack.pop().unwrap(), Elt::Bool(false));
    }

    #[test]
    fn test_evaluate_line_int() {
        let mut stack = Stack::new();
        let s = "12".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        assert_eq!(stack.pop().unwrap(), Elt::Int(12));
    }

    #[test]
    fn test_evaluate_line_plus() {
        let mut stack = Stack::new();
        let s = "12".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "13".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "+".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        assert_eq!(stack.pop().unwrap(), Elt::Int(25));
    }

    #[test]
    fn test_evaluate_line_neg() {
        let mut stack = Stack::new();
        let s = "false".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "~".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        assert_eq!(stack.pop().unwrap(), Elt::Bool(true));
    }

    #[test]
    fn test_evaluate_line_swap() {
        let mut stack = Stack::new();
        let s = "false".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "15".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "<->".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        assert_eq!(stack.pop().unwrap(), Elt::Bool(false));
        assert_eq!(stack.pop().unwrap(), Elt::Int(15));
    }

    #[test]
    fn test_evaluate_line_eq() {
        let mut stack = Stack::new();
        let s = "12".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "15".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "=".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        assert_eq!(stack.pop().unwrap(), Elt::Bool(false));
    }

    #[test]
    fn test_evaluate_line_rand() {
        let mut stack = Stack::new();
        let s = "12".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let s = "#".to_string();
        assert!(evaluate_line(&mut stack, &s).is_ok());
        let res = stack.pop();
        assert!(res.is_ok());
        let res = res.unwrap();
        assert!(res >= Elt::Int(0));
        assert!(res < Elt::Int(12));
    }

    #[test]
    fn test_evaluate_line_quit() {
        let mut stack = Stack::new();
        let s = "quit".to_string();
        let res = evaluate_line(&mut stack, &s);
        assert!(res.is_err());
        if let Err(Error::Quit) = res {
        } else { assert!(false); }
    }

    #[test]
    fn test_evaluate_line_bad_parse() {
        let mut stack = Stack::new();
        let s = "~false".to_string();
        let res = evaluate_line(&mut stack, &s);
        assert!(res.is_err());
        if let Err(Error::Syntax(_)) = res {
        } else { assert!(false); }
    }
}
