// main.rs
enum List {
    Nil,
    Cons(Box<List>),
}

impl List {
    fn append(&mut self) {
        let mut link = self;

        loop {
            let tmp = link;
            if let List::Cons(ref mut rest) = *tmp {
                link = rest;
            } else {
                link = tmp;
                break
            }
        }

        *link = List::Cons(Box::new(List::Nil));
    }

    fn print(&self) {
        let mut link = self;
        
        loop {
            match *link {
                List::Nil => {
                    println!("[]");
                    break
                }

                List::Cons(ref rest) => {
                    print!("() -> ");
                    link = rest;
                }
            }
        }
    }
}

fn main() {
    let mut list = List::Nil;
    list.print();

    list.append();
    list.print();
}
