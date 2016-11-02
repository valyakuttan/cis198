extern crate hw01;

use hw01::problem3::*;
use hw01::problem4::*;

fn main() {
    let ps = hanoi(3, Peg::A, Peg::C, Peg::B);

    println!("{:?}", ps);

    let mut a = Box::new(5);

    loop {
        let b = a;
        if *b > 10 {
            break
        }
        
    }
}
