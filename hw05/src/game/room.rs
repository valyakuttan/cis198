use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use std::mem;

use super::curio::Curio;
use super::hall::Hall;

pub struct Room {
    pub name: String,
    pub contents: Vec<Curio>,
    pub halls: Vec<Rc<Hall>>,
    pub wumpus: bool,
}

impl PartialEq for Room {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Room {}

impl Room {

    pub fn consume_contents(&mut self) -> Vec<Curio> {
        mem::replace(&mut self.contents, vec![])
    }

    pub fn neighbors(&self) -> Vec<Rc<RefCell<Room>>> {
        let mut neighbors = vec![];

        for hall in self.halls.clone() {
            let left = hall.left.borrow();
            let right = hall.right.borrow();
            
            if *self == *left {
                neighbors.push(hall.right.clone());
            } else if *self == *right {
                neighbors.push(hall.left.clone());
            }
        }

        neighbors
    }
    
    pub fn neighbors_string(&self) -> String {
        self.neighbors().iter().map(|n| {
            let n = n.clone();
            let neighbor = n.borrow();
            neighbor.name.to_lowercase()
        }).collect::<Vec<String>>().join(", ")
    }
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let wumpus = if self.wumpus {
            "Oops you are eaten by Wumpus!!!"
        } else {
            ""
        };
        
        write!(f, "You are in {}.\n\nYour neighbors are {}.\n\n{}",
               self.name, self.neighbors_string(), wumpus)
    }
}
