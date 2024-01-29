

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum State {
    None,
    Area,
    Helps,
    Mobiles,
    Objects,
    Rooms,
    Resets,
    Shops,
    Specials,
    End,
}

struct Game {
    area_manager: AreaManager,
    //room_manager: RoomManager,
    // mob_manager: MobManager,
    // obj_manager: ObjManager,
    // reset_manager: ResetManager,
    // shop_manager: ShopManager,
    // special_manager: SpecialManager,
}

pub struct Area {
    min_vnum: i32,
    max_vnum: i32,
    name: String,
}

impl Area {
    pub fn new(min_vnum: i32, max_vnum: i32, name: String) -> Self {
        Self { min_vnum, max_vnum, name }
    }
}

pub struct AreaManager {
    areas: Vec<Area>,
}

impl AreaManager {
    pub fn new() -> Self {
        Self { areas: Vec::new() }
    }

    pub fn add_area(&mut self, area: Area) {
        self.areas.push(area);
    }

    // Other area management methods...
}

fn parse_area(game: &mut Game, line: &str) -> io::Result<()> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 4 || parts[0] != "#AREA" {
        panic!("Invalid area line: {}", line);
    }

    let range: Vec<&str> = parts[1][1..parts[1].len()-1].split(',').collect();
    if range.len() != 2 {
        panic!("Invalid area range: {}", parts[1]);
    }

    let min_vnum = range[0].trim().parse::<i32>().expect("Invalid min vnum");
    let max_vnum = range[1].trim().parse::<i32>().expect("Invalid max vnum");

    let name = parts[2..].join(" ");

    // Create a new Area instance and add it to the area manager
    game.area_manager.add_area(Area::new(min_vnum, max_vnum, name));

    Ok(())
}



fn parse_are_file(game: &Game, filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut state = State::None;

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        match state {
            State::None => {
                if line.starts_with('#') {
                    let keyword = &line[1..];
                    state = match keyword {
                        "AREA" => State::Area,
                        "HELPS" => State::Helps,
                        "MOBILES" => State::Mobiles,
                        "OBJECTS" => State::Objects,
                        "ROOMS" => State::Rooms,
                        "RESETS" => State::Resets,
                        "SHOPS" => State::Shops,
                        "SPECIALS" => State::Specials,
                        "$" => State::End,
                        _ => panic!("Unexpected keyword: {}", keyword),
                    };
                } else {
                    panic!("Unexpected line: {}", line);
                }
            }
            State::Area => {
                // Parse area
                state = State::None;
            }
            State::Helps => {
                // Parse helps
                state = State::None;
            }
            State::Mobiles => {
                // Parse mobiles
                state = State::None;
            }
            State::Objects => {
                // Parse objects
                state = State::None;
            }
            State::Rooms => {
                // Parse rooms
                state = State::None;
            }
            State::Resets => {
                // Parse resets
                state = State::None;
            }
            State::Shops => {
                // Parse shops
                state = State::None;
            }
            State::Specials => {
                // Parse specials
                state = State::None;
            }
            State::End => {
                break;
            }
        }
    }

    Ok(())
}






// def parse_are_file(filename):
//     with open(filename) as f:
//         lines = f.readlines()
        
//     lines = [line.strip() for line in lines]
//     lines = [line for line in lines if line]

//     STATE = None
//     line_index = 0

//     while line_index < len(lines):
//         if STATE == None:      
//             if lines[line_index].startswith('#'):
//                 STATE = lines[line_index][1:].split()[0]
//                 line_index += 1
//             else:
//                 print("Not expected: ", lines[line_index])
//                 line_index += 1
//         elif STATE == "AREA":
//             Area = lines[line_index-1].replace('#AREA', '').strip()[:-1]
//             print("\tArea: ", Area)
//             STATE = None
//         elif STATE == "HELPS":
//             while lines[line_index].startswith('0 $~') == False:
//                 line_index += 1
//             line_index += 1
//             STATE = None
//         elif STATE in ["MOBILES", "OBJECTS", "ROOMS"]:
//             if lines[line_index].startswith('#'):
//                 if lines[line_index].startswith('#0'):
//                     STATE = None
//                     line_index += 1 
//                 else:
//                     offset = 1
//                     while lines[line_index+offset].startswith('#') == False:
//                         offset += 1
//                     parse_func = parse_mob if STATE == "MOBILES" else parse_object if STATE == "OBJECTS" else parse_room
//                     parse_func(lines[line_index:line_index+offset])
//                     line_index += offset
//         elif STATE == "RESETS":
//             # Resets pass the all section as a block on input to parse_reset
//             offset = 0
//             while lines[line_index+offset].startswith('S') == False:
//                 offset += 1
//             parse_reset(lines[line_index:line_index+offset])
//             line_index += offset + 1
//             STATE = None
//         elif STATE in ["SHOPS", "SPECIALS"]:
//             # Shops and specials passes line by line to parse function
//             if lines[line_index].startswith('S') or lines[line_index] == 0:
//                 STATE = None
//                 line_index += 1
//             else:
//                 parse_func = parse_shops if STATE == "SHOPS" else parse_specials
//                 parse_func(lines[line_index])
//                 line_index += 1  
//         elif STATE == "$":
//             return