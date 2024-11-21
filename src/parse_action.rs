use std::{env, fs::File};

use crate::action::{Action, Movement};

pub fn parse_action(prev_action: &Action) -> Option<Action> {
    let file = File::open(env::current_dir().unwrap().join("json/cohe.json")).unwrap();
    let file: Action = serde_json::from_reader(file).unwrap();
    if prev_action == &file {
        None
    } else {
        println!("{:?}", file);
        Some(file)
    }
}
pub fn serialize() {
    //this was used to make the json so i knew how to format it lol
    let a = Action::Move(Movement::BACKWARD);
    //let a = Action::Chat("hi".to_string());
    let slice_string_in_json_format = serde_json::to_string(&a).unwrap();
    println!("{}", &slice_string_in_json_format);
    let des: Action = serde_json::from_str(&slice_string_in_json_format).unwrap();
    println!("{:?}", des);
}
