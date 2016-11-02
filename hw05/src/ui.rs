use std::io::{self, Write};

use game::player::Player;
use game::player::Command;

#[derive(Debug)]
enum Error {
    Parse,
    Quit,
}

pub fn game_loop(mut player: Player) {
    loop {
        // Print a user input prompt.
        println!("{}\n\nExits are: {}.\n\nWhat wouldst thou deau?",
                 player, player.location.borrow().neighbors_string());
        print!("> ");
        io::stdout().flush().unwrap();

        let mut buf = String::new();
        match io::stdin().read_line(&mut buf) {
            Err(err) => {
                panic!("error: {}", err);
            }
            Ok(0) => {
                break;
            }
            Ok(_) => {
                let parse = parse_line(&buf);
                if let Err(Error::Parse) = parse {
                    println!("I do not know how to {}!", buf.trim());
                } else if let Err(Error::Quit) = parse {
                    break;
                } else if let Ok(cmd) = parse {
                    if let Err(_) = player.act(cmd) {
                        println!("I don't know how to {}!", buf.trim());
                    }
                }

                if player.has_won() {
                    println!("Hurray you won!");
                    return;
                }
                
                if player.hp <= 0 {
                    println!("You try in vain to shovel more wall chicken into \
                              your mouth, but you've been impaled by too many spikes or Wumpi :(");
                    println!("You Lose!");
                    return;
                }
            }
        }
    }
    println!("Score: {}", player.gold * 1000);
}

fn parse_line(buf: &String) -> Result<Command, Error> {
    use game::player::Command::*;

    let tokens = buf.trim().split_whitespace();
    let mut tokens = tokens.map(|t| String::from(t).to_lowercase());

    let cmd = try!(tokens.next().ok_or(Error::Parse));
    if cmd == "go" {
        let room = try!(tokens.next().ok_or(Error::Parse));
        Ok(Go(room))
    } else if cmd == "shoot" {
        let room = try!(tokens.next().ok_or(Error::Parse));
        Ok(Shoot(room))
    } else if cmd == "quit" {
        println!("Bye forever :(");
        Err(Error::Quit)
    } else {
        Err(Error::Parse)
    }
}
