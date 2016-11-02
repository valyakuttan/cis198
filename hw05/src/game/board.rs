use std::cell::RefCell;
use std::rc::Rc;
use std::result;
use std::io::Read;

use rustc_serialize::json::Json;

use super::curio::Curio;
use super::hall::Hall;
use super::room::Room;

pub type Result<T> = result::Result<T, String>;

pub struct Board {
    pub rooms: Vec<Rc<RefCell<Room>>>,
}

impl Board {
    pub fn build_board(reader: &mut Read) -> Result<Board> {
        let mut board = Board { rooms: Vec::new(), };

        let board_json: Json = try!(
            Json::from_reader(reader)
                .map_err(|_| "Unable to create JSON reader".to_string())
        );

        try!(board.parse_rooms(&board_json)
             .map_err(|_| "Unable to parse rooms".to_string()));
        
        try!(board.parse_halls(&board_json)
             .map_err(|_| "Unable to parse halls".to_string()));

        Ok(board)
    }

    fn parse_rooms(&mut self, json: &Json) -> Result<()> {
        // Find room list
        let rooms_opt: Option<&Json> = json.find("rooms");

        let json_rooms: &Json = try!(
            rooms_opt.ok_or("Unable to parse rooms".to_string())
        );

        // Parse each room
        let rooms: &Vec<Json> = try!(
            json_rooms.as_array()
                .ok_or("Unable to parse name".to_string())
        );
        
        for r in rooms {

            // Parse room name
            let name_opt: Option<&Json> = r.find("name");
            let json_name: &Json = try!(
                name_opt.ok_or("Unable to parse name".to_string())
            );
            
            let name: &str = try!(
                json_name.as_string()
                    .ok_or("Unable to parse name".to_string())
            );

            // Parse curios
            let curios_opt: Option<&Json> = r.find("curios");

            let json_curios: &Json = try!(
                curios_opt.ok_or("Unable to parse curio".to_string())
            );

            let n: u64 = try!(
                json_curios.as_u64()
                    .ok_or("Unable to parse curio".to_string())
            );

            let curios: Vec<Curio> = Curio::generate_n(n as usize);

            // Wumpus?
            let wumpus: bool = {
                if let Some(json_wumpus) = r.find("wumpus") {
                    try!(
                        json_wumpus.as_boolean()
                            .ok_or("Unable to parse Wumpus".to_string())
                    )
                } else {
                    false
                }
            };
            
            self.rooms.push(Rc::new(RefCell::new(Room{
                name: name.to_string(),
                contents: curios,
                halls: vec![],
                wumpus: wumpus
            })))
        }

        Ok(())
    }

    fn parse_halls(&mut self, json: &Json) -> Result<()> {
        // Find hall list
        let halls_opt: Option<&Json> = json.find("halls");
        
        let json_halls: &Json = try!(
            halls_opt.ok_or("Unable to parse halls".to_string())
        );

        // Parse each hall
        let halls: &Vec<Json> = try!(
            json_halls.as_array()
                .ok_or("Unable to parse halls".to_string())
        );
        
        for h in halls {
            let h: &Vec<Json> = try!(
                h.as_array().ok_or("Unable to parse halls".to_string())
            );

            if h.len() > 2 {
                return Err(
                    "Invalid number of rooms per hall".to_string()
                );
            }
            
            let left_u64: u64 = try!(
                h[0].as_u64()
                    .ok_or(
                        "Unable to parse left room of hall way"
                            .to_string()
                    )
            );

            let right_u64: u64 = try!(
                h[1].as_u64()
                    .ok_or(
                        "Unable to parse right room of hall way"
                            .to_string()
                    )
            );

            let left_rc = self.rooms[left_u64 as usize].clone();
            let right_rc = self.rooms[right_u64 as usize].clone();

            let mut hall = Hall::new();

            hall.left = left_rc.clone();
            hall.right = right_rc.clone();
                        
            let hall_rc = Rc::new(hall);

            let mut left_room = left_rc.borrow_mut();
            left_room.halls.push(hall_rc.clone());

            let mut right_room = right_rc.borrow_mut();
            right_room.halls.push(hall_rc.clone());
        }
        
        Ok(())
    }
    
    pub fn spawn_location(&self) -> Rc<RefCell<Room>> {
        self.rooms[0].clone()
    }
}
