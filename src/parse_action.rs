use std::{env, fs::File};

use crate::action::{Action, Movement};

pub fn parse_action() -> Action {
    let file = File::open(
        env::current_dir()
            .unwrap()
            .join("json/example_movement.json"),
    )
    .unwrap();
    let file: Action = serde_json::from_reader(file).unwrap();
    println!("{:?}", file);
    file
}
pub fn serialize() {
    let a = Action::Action(Movement::BACKWARD);
    //let a = Action::Chat("hi".to_string());
    let slice_string_in_json_format = serde_json::to_string(&a).unwrap();
    println!("{}", &slice_string_in_json_format);
    let des: Action = serde_json::from_str(&slice_string_in_json_format).unwrap();
    println!("{:?}", des);
}
