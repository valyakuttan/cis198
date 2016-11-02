use std::cell::RefCell;
use std::rc::Rc;

use super::room::Room;

pub struct Hall {
    pub left: Rc<RefCell<Room>>,
    pub right: Rc<RefCell<Room>>,
}

impl Hall {

    pub fn new() -> Hall {
        Hall {
            left: Rc::new(RefCell::new(new_room())),
            right: Rc::new(RefCell::new(new_room()))
        }
    }
    
    /// Given a Room `room`, find the room at the other end of
    /// Hall `self`.
    pub fn other(&self, room: &Room) -> Rc<RefCell<Room>> {
        let left = self.left.borrow();

        if *left == *room {
            self.right.clone()
        } else {
            self.left.clone()
        }
    }

}

fn new_room() -> Room {
    Room {
        name: "name".to_string(),
        contents: vec![],
        halls: vec![],
        wumpus: false
    }
}
