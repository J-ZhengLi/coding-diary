mod game_data;

use game_data::{Item, ItemType};
use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Deserialize, Serialize)]
struct Data {
    weapons: Vec<Item>
}

fn main() {
    let items_json_raw = include_str!("../res/items.json");

    println!("Raw: {}", items_json_raw);

    // Error here
    let item_struct: Data = from_str(items_json_raw).unwrap();
}
