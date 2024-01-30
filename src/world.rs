

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
    object_manager: ObjectManager,
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

pub struct ObjectManager {
    objects: HashMap<u16, ObjectTemplate>,
}

impl ObjectManager {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }

    pub fn add(&mut self, id: u16, object: ObjectTemplate) {
        self.objects.insert(id, object);
    }

    pub fn get(&self, id: u16) -> Option<&ObjectTemplate> {
        self.objects.get(&id)
    }

    pub fn get_all(&self) -> Vec<&ObjectTemplate> {
        self.objects.values().collect()
    }
}

pub struct ObjectTemplate {
    vnum: u16,
    keywords: String,
    short_desc: String,
    long_desc: String,
    action_desc: String,
    item_type: u32,
    extra_flags: u32,
    wear_flags: u32,
    value_0: u32,
    value_1: u32,
    value_2: u32,
    value_3: u32,
    weight: u16,
    cost: i32,
    max_hitpoints: i32,
}

impl ObjectTemplate {
    pub fn new(vnum: u16, keywords: String, short_desc: String, long_desc: String, action_desc: String, item_type: u32, extra_flags: u32, wear_flags: u32, value_0: u32, value_1: u32, value_2: u32, value_3: u32, weight: u16, cost: i32) -> Self {
        Self {
            vnum,
            keywords,
            short_desc,
            long_desc,
            action_desc,
            item_type,
            extra_flags,
            wear_flags,
            value_0,
            value_1,
            value_2,
            value_3,
            weight,
            cost,
            max_hitpoints: 100,
        }
    }
}

pub struct ResetManager {
    mob_resets: Vec<ResetMob>,
    object_resets: Vec<ResetObject>,
    // door_resets: Vec<ResetDoor>,
    // randomize_doors_resets: Vec<ResetRandomizeDoors>,
    mob_repop_queue: HashSet<ResetMob>,
    obj_repop_queue: HashSet<ResetObject>,
}


impl ResetManager {
    pub fn new() -> Self {
        Self {
            mob_resets: Vec::new(),
            object_resets: Vec::new(),
            // door_resets: Vec::new(),
            // randomize_doors_resets: Vec::new(),
            mob_repop_queue: HashSet::new(),
            obj_repop_queue: HashSet::new(),
        }
    }

    pub fn add_mob_reset(&mut self, reset_mob: ResetMob) {
        self.mob_resets.push(reset_mob);
    }

    pub fn add_object_reset(&mut self, reset_object: ResetObject) {
        self.object_resets.push(reset_object);
    }

    pub fn add_to_mob_repop_queue(&mut self, mob_reset: ResetMob) {
        self.mob_repop_queue.insert(mob_reset);
    }

    pub fn add_to_obj_repop_queue(&mut self, obj_reset: ResetObject) {
        self.obj_repop_queue.insert(obj_reset);
    }

    pub fn process_mob_repop_queue(&mut self) -> bool {
        if self.mob_repop_queue.is_empty() {
            return false;
        }
        // while let Some(mob_reset) = self.mob_repop_queue.take() {
        //     // let mob_template = mob_manager.get(mob_reset.mob_vnum);
        //     // let room = room_manager.get(mob_reset.room_vnum);
        //     // MobInstance(mob_template, mob_reset, room);
            
        //     // todo
        //     // for item in mob_reset.inventory:
        //     //     mob.add_item(item)
        //     // mob.mob_reset = mob_reset
        //     // mob_reset.equipment = Equipment()
        //     // for item in mob_reset.equipment:
        //     //     mob_reset.equipment.equip(item)
        //     // mob_reset.inventory = []
        //     // mob_reset.comment = ""
        // }
        true
    }

    pub fn process_obj_repop_queue(&mut self) -> bool {
        if self.mob_repop_queue.is_empty() && self.obj_repop_queue.is_empty() {
            return false;
        }
        // while let Some(obj_reset) = self.obj_repop_queue.take() {
        //     // ObjectInstance(object_manager.get(obj_reset.obj_vnum), obj_reset)
        //     // todo
        //     // code for objects within containers
        // }
        true
    }

    pub fn process_repop_queue(&mut self) -> bool {
        self.process_mob_repop_queue();
        self.process_obj_repop_queue()
    }

}

#[derive(Eq, PartialEq, Hash)]
pub struct ResetMob {
    mob_vnum: u16,
    max_count: u16,
    room_vnum: u16,
    comment: String,
    // equipment: Equipment,
    // inventory: Inventory,
}

impl ResetMob {
    pub fn new(mob_vnum: u16, max_count: u16, room_vnum: u16, comment: String) -> Self {
        Self {
            mob_vnum,
            max_count,
            room_vnum,
            comment,
            // equipment: Equipment::new(),
            // inventory: Inventory::new(),
        }
    }

    // pub fn add_item(&mut self, item: u16) {
    //     self.inventory.add_item(item);
    // }
}

#[derive(Eq, PartialEq, Hash)]
enum ObjLocationType {
    Player,
    Room,
    Mob,
    PcContainer,
    OtherContainer,
}

#[derive(Eq, PartialEq, Hash)]
pub struct ResetObject {
    obj_vnum: u16,
    location_vnum: u16,
    location_type: ObjLocationType,
}

impl ResetObject {
    pub fn new(obj_vnum: u16, location_vnum: u16, location_type: ObjLocationType) -> Self {
        Self {
            obj_vnum,
            location_vnum,
            location_type,
        }
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

fn parse_object(game: &mut Game, lines: &mut Vec<String>) {
    let obj_vnum: u16 = lines.remove(0)[1..].parse().expect("Error parsing object vnum");
    let mut offset = 0;

    let (keywords, offset_add) = match parse_multi_line(&lines[offset..]) {
        Ok(result) => result,
        Err(_) => {
            eprintln!("Error parsing multi line");
            return;
        }
    };
    let keywords = keywords.to_lowercase();
    offset += offset_add;

    let (short_desc, offset_add) = match parse_multi_line(&lines[offset..]) {
        Ok(result) => result,
        Err(_) => {
            eprintln!("Error parsing multi line");
            return;
        }
    };
    offset += offset_add;

    let (long_desc, offset_add) = match parse_multi_line(&lines[offset..]) {
        Ok(result) => result,
        Err(_) => {
            eprintln!("Error parsing multi line");
            return;
        }
    };
    offset += offset_add;

    let (action_desc, offset_add) = match parse_multi_line(&lines[offset..]) {
        Ok(result) => result,
        Err(_) => {
            eprintln!("Error parsing multi line");
            return;
        }
    };
    offset += offset_add;

    let parts: Vec<&str> = lines[offset].split_whitespace().collect();
    let item_type = parts[0].parse().expect("Error parsing item type");
    let extra_flags = parse_flags(parts[1]).expect("Error parsing extra flags");
    let wear_flags = parse_flags(parts[2]).expect("Error parsing wear flags");
    offset += 1;

    let parts: Vec<&str> = lines[offset].split_whitespace().collect();
    let value_0 = parse_flags(parts[0]).expect("Error parsing value 0");
    let value_1 = parse_flags(parts[1]).expect("Error parsing value 1");
    let value_2 = parse_flags(parts[2]).expect("Error parsing value 2");
    let value_3 = parse_flags(parts[3]).expect("Error parsing value 3");
    offset += 1;

    let parts: Vec<&str> = lines[offset].split_whitespace().collect();
    let weight = parts[0].parse().expect("Error parsing weight");
    let cost = parts[1].parse().expect("Error parsing cost");

    let current_object = ObjectTemplate::new(obj_vnum, keywords, short_desc, long_desc, action_desc, item_type, extra_flags, wear_flags, value_0, value_1, value_2, value_3, weight, cost);

    game.object_manager.add(obj_vnum, current_object);
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
                if line == "#0" {
                    if !lines.is_empty() {
                        parse_object(game, &mut lines);  // Parse the last room
                    }
                    lines.clear();
                    state = State::None;
                } else if line.starts_with('#') {
                    if !lines.is_empty() {
                        parse_object(game, &mut lines);  // Parse the current room
                        lines.clear();  // Clear the vector for the next room
                    }
                    lines.push(line.to_string());  // Start collecting lines for the next room
                } else {
                    lines.push(line.to_string());  // Continue collecting lines for the current room
                }
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

fn main() {
    let mut game = Game {
        area_manager: AreaManager::new(),
        room_manager: RoomManager::new(),
        mob_manager: MobManager::new(),
        object_manager: ObjectManager::new(),
    };

    parse_are_file(&mut game, "beach.are").expect("Failed to parse area file");


    // write me code that iterates over the rooms and prints out the room name and description
    // for room in game.room_manager.get_all() {
    //     println!("Room: {} {}", room.vnum, room.name);
    //     println!("Description: {}", room.description);
    //     println!("Area Number: {}", room.area_number);
    //     println!("Room Flags: {}", room.room_flags);
    //     println!("Sector Type: {}", room.sector_type);
    //     println!("Doors: ");
    //     for door in &room.door_list {
    //         println!("Door: {} {}", door.door_number, door.description);
    //         println!("Keywords: {}", door.keywords);
    //         println!("Locks: {}", door.locks);
    //         println!("Key: {}", door.key);
    //         println!("To Room: {}", door.to_room);
    //     }
    //     println!("Extended Descriptions: ");
    //     for desc in &room.extended_descriptions_list {
    //         println!("Keywords: {}", desc.keywords);
    //         println!("Description: {}", desc.description);
    //     }
    //     println!("\n\n");
    // }

    // for mob in game.mob_manager.get_all() {
    //     println!("Mob Vnum: {}", mob.vnum);
    //     println!("Keywords: {}", mob.keywords);
    //     println!("Short Description: {}", mob.short_desc);
    //     println!("Long Description: {}", mob.long_desc);
    //     println!("Act Flags: {}", mob.act_flags);
    //     println!("Affected By: {}", mob.aff_flags);
    //     println!("Alignment: {}", mob.align);
    //     println!("Level: {}", mob.level);
    //     println!("Hitroll: {}", mob.hitroll);
    //     println!("AC: {}", mob.ac);
    //     println!("Hit Dice: {}d{}+{}", mob.hitdice_num, mob.hitdice_size, mob.hitdice_bonus);
    //     println!("Damage Dice: {}d{}+{}", mob.damdice_num, mob.damdice_size, mob.damdice_bonus);
    //     println!("Gold: {}", mob.gold);
    //     println!("Experience: {}", mob.xp);
    //     println!("Sex: {}", mob.sex);
    //     println!("\n\n");
    // }
    for object in game.object_manager.get_all() {
        println!("Object Vnum: {}", object.vnum);
        println!("Keywords: {}", object.keywords);
        println!("Short Description: {}", object.short_desc);
        println!("Long Description: {}", object.long_desc);
        println!("Action Description: {}", object.action_desc);
        println!("Item Type: {}", object.item_type);
        println!("Extra Flags: {}", object.extra_flags);
        println!("Wear Flags: {}", object.wear_flags);
        println!("Value 0: {}", object.value_0);
        println!("Value 1: {}", object.value_1);
        println!("Value 2: {}", object.value_2);
        println!("Value 3: {}", object.value_3);
        println!("Weight: {}", object.weight);
        println!("Cost: {}", object.cost);
        println!("\n\n");
    }

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


// class Room:
//     def __init__(self, vnum):
//         self.vnum = vnum
//         self.name = ""
//         self.description = ""
//         self.area_number = 0
//         self.room_flags = 0
//         self.sector_type = 0
        
//         self.doors = {}
//         self.extended_descriptions = []
    
//         self.mob_list = set()
//         self.object_list = set()
//         self.player_list = set()
//         self.door_list = set()
//         self.extended_descriptions_list = set()

//     def add_door(self, door_number, door_description, keywords, locks, key, to_room):
//         self.doors[door_number] = {
//             "description": door_description,
//             "keywords": keywords,
//             "locks": locks,
//             "key": key,
//             "to_room": to_room
//         }
//         self.door_list.add(Door(door_number, door_description, keywords, locks, key, to_room))

//     def add_extended_description(self, keywords, description):
//         self.extended_descriptions.append({
//             "keywords": keywords,
//             "description": description
//         })
//         self.extended_descriptions_list.add(ExtendedDescription(keywords, description))


// class Door:
//     def __init__(self, door_number, door_description, keywords, locks, key, to_room):
//         self.door_number = door_number
//         self.description = door_description
//         self.keywords = keywords
//         self.locks = locks
//         self.key = key
//         self.to_room = to_room
        
//     def get_keywords(self):
//         return self.keywords.split()
    
//     def get_description(self):
//         return self.description

// class ExtendedDescription:
//     def __init__(self, keywords, description):
//         self.keywords = keywords
//         self.description = description
        
//     def get_keywords(self):
//         return self.keywords.split()
    
//     def get_description(self):
//         return self.description 

// class RoomManager:
//     def __init__(self):
//         self.items = {}
        
//     def add(self, id, item):
//         self.items[id] = item
    
//     def get(self, id):
//         return self.items.get(id)
    
//     def remove(self, id):
//         if id in self.items:
//             del self.items[id]
            
//     def get_all(self):
//         return self.items.values()

