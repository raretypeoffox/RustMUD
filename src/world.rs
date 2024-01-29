

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashMap, HashSet};
use regex::Regex;

#[derive(Debug)]
enum State {
    None,
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
    room_manager: RoomManager,
    mob_manager: MobManager,
    // obj_manager: ObjManager,
    // reset_manager: ResetManager,
    // shop_manager: ShopManager,
    // special_manager: SpecialManager,
}

pub struct Area {
    min_vnum: u16,
    max_vnum: u16,
    builder: String,
    name: String,
}

impl Area {
    pub fn new(min_vnum: u16, max_vnum: u16, builder: String, name: String) -> Self {
        Self { min_vnum, max_vnum, builder, name }
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

pub struct MobTemplate {
    vnum: u16,
    keywords: String,
    short_desc: String,
    long_desc: String,
    desc: String,
    act_flags: u32,
    aff_flags: u32,
    align: i16,
    level: u16,
    hitroll: i16,
    ac: i16,
    hitdice_num: i16,
    hitdice_size: i16,
    hitdice_bonus: i16,
    damdice_num: i16,
    damdice_size: i16,
    damdice_bonus: i16,
    gold: i32,
    xp: i32,
    sex: String,
    speed: i8,
}

impl MobTemplate {
    pub fn new(vnum: u16, keywords: String, short_desc: String, long_desc: String, desc: String, act_flags: u32, aff_flags: u32, align: i16, level: u16, hitroll: i16, ac: i16, hitdice_num: i16, hitdice_size: i16, hitdice_bonus: i16, damdice_num: i16, damdice_size: i16, damdice_bonus: i16, gold: i32, xp: i32, sex: String, speed: i8) -> Self {
        Self { vnum, keywords, short_desc, long_desc, desc, act_flags, aff_flags, align, level, hitroll, ac, hitdice_num, hitdice_size, hitdice_bonus, damdice_num, damdice_size, damdice_bonus, gold, xp, sex, speed }
    }
}

pub struct MobManager {
    mobs: HashMap<u16, MobTemplate>,
}


impl MobManager {
    fn new() -> Self {
        Self {
            mobs: HashMap::new(),
        }
    }

    fn add(&mut self, id: u16, mob: MobTemplate) {
        self.mobs.insert(id, mob);
    }

    fn get(&self, id: u16) -> Option<&MobTemplate> {
        self.mobs.get(&id)
    }

    fn get_all(&self) -> Vec<&MobTemplate> {
        self.mobs.values().collect()
    }
}

struct Room {
    vnum: u16,
    name: String,
    description: String,
    area_number: u16,
    room_flags: u32,
    sector_type: u32,
    doors: HashMap<i32, Door>,
    extended_descriptions: Vec<ExtendedDescription>,
    mob_list: HashSet<i32>,
    object_list: HashSet<i32>,
    player_list: HashSet<i32>,
    door_list: HashSet<Door>,
    extended_descriptions_list: HashSet<ExtendedDescription>,
}

impl Room {
    fn new(vnum: u16) -> Self {
        Self {
            vnum,
            name: String::new(),
            description: String::new(),
            area_number: 0,
            room_flags: 0,
            sector_type: 0,
            doors: HashMap::new(),
            extended_descriptions: Vec::new(),
            mob_list: HashSet::new(),
            object_list: HashSet::new(),
            player_list: HashSet::new(),
            door_list: HashSet::new(),
            extended_descriptions_list: HashSet::new(),
        }
    }

    fn add_door(&mut self, door: Door) {
        self.door_list.insert(door);
    }

    fn add_extended_description(&mut self, desc: ExtendedDescription) {
        self.extended_descriptions_list.insert(desc);
    }
}

#[derive(Eq, PartialEq, Hash)]
struct Door {
    door_number: u8,
    description: String,
    keywords: String,
    locks: u16,
    key: u16,
    to_room: u16,
}

impl Door {
    fn new(door_number: u8, description: String, keywords: String, locks: u16, key: u16, to_room: u16) -> Self {
        Self {
            door_number,
            description,
            keywords,
            locks,
            key,
            to_room,
        }
    }

    fn get_keywords(&self) -> Vec<&str> {
        self.keywords.split_whitespace().collect()
    }

    fn get_description(&self) -> &str {
        &self.description
    }
}

#[derive(Eq, PartialEq, Hash)]
struct ExtendedDescription {
    keywords: String,
    description: String,
}

impl ExtendedDescription {
    fn new(keywords: String, description: String) -> Self {
        Self {
            keywords,
            description,
        }
    }

    fn get_keywords(&self) -> Vec<&str> {
        self.keywords.split_whitespace().collect()
    }

    fn get_description(&self) -> &str {
        &self.description
    }
}

struct RoomManager {
    rooms: HashMap<u16, Room>,
}

impl RoomManager {
    fn new() -> Self {
        Self {
            rooms: HashMap::new(),
        }
    }

    fn add(&mut self, id: u16, room: Room) {
        self.rooms.insert(id, room);
    }

    fn get(&self, id: u16) -> Option<&Room> {
        self.rooms.get(&id)
    }

    fn get_all(&self) -> Vec<&Room> {
        self.rooms.values().collect()
    }
}

fn parse_multi_line(lines: &[String]) -> Result<(String, usize), &'static str> {
    let mut ret_string = String::new();
    for (offset, line) in lines.iter().enumerate() {
        if line.ends_with('~') {
            ret_string += &line[..line.len()-1].replace("\\n", "\n");
            return Ok((ret_string, offset + 1));
        } else if line.starts_with('~') {
            return Ok((ret_string, offset + 1));
        } else {
            if !ret_string.is_empty() {
                ret_string.push(' '); // Add a space at the beginning of the line
            }
            ret_string += &line.replace("\\n", "\n");
        }
    }
    Err("No end of multi line found")
}


fn parse_flags(flag_string: &str) -> Result<u32, std::num::ParseIntError> {
    if flag_string.contains('|') {
        let numbers: Result<Vec<u32>, _> = flag_string.split('|').map(str::parse).collect();
        numbers.map(|nums| nums.iter().sum())
    } else {
        flag_string.parse()
    }
}

fn parse_area(game: &mut Game, line: &str) -> io::Result<()> {
    let re = Regex::new(r"#AREA\s*\{\s*?(\d+)\s+(\d+)\}\s*(\w+)\s+(.*)~").unwrap();
    if let Some(caps) = re.captures(line) {
        let min_vnum = caps[1].parse::<u16>().expect("Invalid min vnum");
        let max_vnum = caps[2].parse::<u16>().expect("Invalid max vnum");
        let builder = &caps[3];
        let name = &caps[4];

        // Create a new Area instance and add it to the area manager
        game.area_manager.add_area(Area::new(min_vnum, max_vnum, builder.to_string(), name.to_string()));
        println!("Area: {} {} {} {}", min_vnum, max_vnum, builder, name);
    } else {
        eprintln!("Could not parse #AREA: {}\nShould be in the format of #AREA {{ X Y }} Builder Name~, where X is the lower level and Y is the upper level\n", line);
    }

    Ok(())
}

fn parse_mob(game: &mut Game, lines: &mut Vec<String>) {

    let mob_vnum: u16 = lines.remove(0)[1..].parse().expect("Error parsing mob vnum");

    let (mob_keywords, mut offset) = match parse_multi_line(&lines[1..]) {
        Ok(result) => result,
        Err(_) => {
            eprintln!("Error parsing multi line");
            return;
        }
    };
    let mob_keywords = mob_keywords.to_lowercase();
    
    let (mob_short_desc, offset_add) = match parse_multi_line(&lines[offset..]) {
        Ok(result) => result,
        Err(_) => {
            eprintln!("Error parsing multi line");
            return;
        }
    };
    offset += offset_add;
    
    let (mob_long_desc, offset_add) = match parse_multi_line(&lines[offset..]) {
        Ok(result) => result,
        Err(_) => {
            eprintln!("Error parsing multi line");
            return;
        }
    };
    offset += offset_add;
    
    let (mob_desc, offset_add) = match parse_multi_line(&lines[offset..]) {
        Ok(result) => result,
        Err(_) => {
            eprintln!("Error parsing multi line");
            return;
        }
    };
    offset += offset_add;
    
    let parts: Vec<&str> = lines[offset].split_whitespace().collect();
    let act_flags = parse_flags(parts[0]).expect("Error parsing act flags");
    let aff_flags = parse_flags(parts[1]).expect("Error parsing aff flags");
    let align: i16 = parts[2].parse().expect("Error parsing align");
    let offset = offset + 1;
    
    let parts: Vec<&str> = lines[offset].split_whitespace().collect();
    let level: u16 = parts[0].parse().expect("Error parsing level");
    let hitroll: i16 = parts[1].parse().expect("Error parsing hitroll");
    let ac: i16 = parts[2].parse().expect("Error parsing ac");
    let replaced = parts[3].replace("D", "d");
    let dice: Vec<&str> = replaced.split('d').collect();
    let hitdice_num: i16 = dice[0].parse().expect("Error parsing hitdice num");
    let dice: Vec<&str> = dice[1].split('+').collect();
    let hitdice_size: i16 = dice[0].parse().expect("Error parsing hitdice size");
    let hitdice_bonus: i16 = dice[1].parse().expect("Error parsing hitdice bonus");
    let replaced = parts[4].replace("D", "d");
    let dice: Vec<&str> = replaced.split('d').collect();
    let damdice_num: i16 = dice[0].parse().expect("Error parsing damdice num");
    let dice: Vec<&str> = dice[1].split('+').collect();
    let damdice_size: i16 = dice[0].parse().expect("Error parsing damdice size");
    let damdice_bonus: i16 = dice[1].parse().expect("Error parsing damdice bonus");
    let offset = offset + 1;
    
    let parts: Vec<&str> = lines[offset].split_whitespace().collect();
    let gold: i32 = parts[0].parse().expect("Error parsing gold");
    let xp: i32 = parts[1].parse().expect("Error parsing xp");
    let offset = offset + 1;
    
    let parts: Vec<&str> = lines[offset].split_whitespace().collect();
    let sex: String = parts[2].to_string();
    
    let current_mob = MobTemplate::new(mob_vnum, mob_keywords, mob_short_desc, mob_long_desc, mob_desc, act_flags, aff_flags, align, level, hitroll, ac, hitdice_num, hitdice_size, hitdice_bonus, damdice_num, damdice_size, damdice_bonus, gold, xp, sex, 3);
    game.mob_manager.add(mob_vnum, current_mob);
}


fn parse_room(game: &mut Game, lines: &mut Vec<String>) {

    let vnum: u16 = match lines.remove(0)[1..].parse() {
        Ok(vnum) => vnum,
        Err(_) => {
            eprintln!("Error parsing vnum");
            return;
        }
    };

    let mut current_room = Room::new(vnum);

    let mut name = lines.remove(0);
    if name.ends_with('~') {
        name.pop();
        current_room.name = name.trim_end().to_string();
    } else {
        eprintln!("Error: room name does not end with ~");
        return;
    }

    let (description, offset) = match parse_multi_line(&lines) {
        Ok(result) => result,
        Err(_) => {
            eprintln!("Error parsing multi line");
            return;
        }
    };
    current_room.description = description;

    lines.drain(0..offset);

    let line = lines[0].clone();
    lines.remove(0);
    let parse_line: Vec<&str> = line.split_whitespace().collect();
    if parse_line.len() < 3 {
        eprintln!("Error parsing room - missing room flags or sector type {}", line);
        return;
    }
    current_room.area_number = match parse_line[0].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error parsing area number");
            return;
        }
    };
    current_room.room_flags = match parse_flags(parse_line[1]) {
        Ok(flags) => flags,
        Err(_) => {
            eprintln!("Error parsing room flags");
            return;
        }
    };

    current_room.sector_type = match parse_line[2].parse() {
        Ok(sector_type) => sector_type,
        Err(_) => {
            eprintln!("Error parsing sector type");
            return;
        }
    };

    while !lines[0].starts_with('S') {
        if lines[0].starts_with('D') {
            let door_dir: u8 = match lines.remove(0)[1..].parse() {
                Ok(dir) => dir,
                Err(_) => {
                    eprintln!("Error parsing door direction");
                    return;
                }
            };
            let (door_desc, offset_add) = match parse_multi_line(&lines) {
                Ok(result) => result,
                Err(_) => {
                    eprintln!("Error parsing multi line");
                    return;
                }
            };
            lines.drain(0..offset_add);
            let (door_keywords, offset_add) = match parse_multi_line(&lines) {
                Ok(result) => result,
                Err(_) => {
                    eprintln!("Error parsing multi line");
                    return;
                }
            };
            
            lines.drain(0..offset_add);
            let line = lines[0].clone();
            lines.remove(0);

            let door_info: Vec<&str> = line.split_whitespace().collect();
            if door_info.len() < 3 {
                eprintln!("Error parsing room - missing door info");
                return;
            }
            let door_locks: u16 = match door_info[0].parse() {
                Ok(lock) => lock,
                Err(_) => {
                    eprintln!("Error parsing door locks");
                    return;
                }
            };
            let door_key: u16 = match door_info[1].parse() {
                Ok(key) => key,
                Err(_) => {
                    eprintln!("Error parsing door key");
                    return;
                }
            };
            let door_to_room: u16 = match door_info[2].parse() {
                Ok(room) => room,
                Err(_) => {
                    eprintln!("Error parsing door to room");
                    return;
                }
            };

            let door = Door {
                door_number: door_dir,
                description: door_desc,
                keywords: door_keywords,
                locks: door_locks,
                key: door_key,
                to_room: door_to_room,
            };
            current_room.add_door(door);

        } else if lines[0].starts_with('E') {
            lines.remove(0);

            let (ex_desc_keywords, offset_add) = match parse_multi_line(&lines) {
                Ok(result) => result,
                Err(_) => {
                    eprintln!("Error parsing multi line");
                    return;
                }
            };
            lines.drain(0..offset_add);

            let (ex_desc_desc, offset_add)
             = match parse_multi_line(&lines) {
                Ok(result) => result,
                Err(_) => {
                    eprintln!("Error parsing multi line");
                    return;
                }
            };
            lines.drain(0..offset_add);

            let desc = ExtendedDescription {
                keywords: ex_desc_keywords,
                description: ex_desc_desc,
            };
            current_room.add_extended_description(desc);
            
        }
    }

    game.room_manager.add(vnum, current_room);
}




fn parse_are_file(game: &mut Game, filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut state = State::None;

    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        //println!("State: {:#?}, Line: {}", state, line);

        match state {
            State::None => {
                if line.starts_with('#') {
                    let keyword = line.split_whitespace().next().unwrap().get(1..).unwrap_or("");
                    state = match keyword {
                        "AREA" => { 
                            parse_area(game, line)?;
                            State::None
                        },
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
            State::Helps => {
                // Parse helps
                state = State::None;
            }
            State::Mobiles => {
                if line == "#0" {
                    if !lines.is_empty() {
                        parse_mob(game, &mut lines);  // Parse the last room
                    }
                    lines.clear();
                    state = State::None;
                } else if line.starts_with('#') {
                    if !lines.is_empty() {
                        parse_mob(game, &mut lines);  // Parse the current room
                        lines.clear();  // Clear the vector for the next room
                    }
                    lines.push(line.to_string());  // Start collecting lines for the next room
                } else {
                    lines.push(line.to_string());  // Continue collecting lines for the current room
                }
            }
            State::Objects => {
                // Parse objects
                state = State::None;
            }
            State::Rooms => {
                if line == "#0" {
                    if !lines.is_empty() {
                        parse_room(game, &mut lines);  // Parse the last room
                    }
                    lines.clear();
                    state = State::None;
                } else if line.starts_with('#') {
                    if !lines.is_empty() {
                        parse_room(game, &mut lines);  // Parse the current room
                        lines.clear();  // Clear the vector for the next room
                    }
                    lines.push(line.to_string());  // Start collecting lines for the next room
                } else {
                    lines.push(line.to_string());  // Continue collecting lines for the current room
                }
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