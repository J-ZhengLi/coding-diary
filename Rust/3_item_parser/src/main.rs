mod game_data;

use game_data::Items;
use serde_json::from_str;

fn main() {
    let item_json_raw = include_str!("../res/items.json");
    //println!("Item Json: {}", item_json_raw);

    let item_struct: Items = from_str(item_json_raw).unwrap();
    println!("Item Struct:\n{:#?}", item_struct);

    let num_armor = if item_struct.armours.is_none() {
        0
    } else {
        item_struct.armours.unwrap().len()
    };
    let num_artfs = if item_struct.artifacts.is_none() {
        0
    } else {
        item_struct.artifacts.unwrap().len()
    };
    let num_chars = if item_struct.characters.is_none() {
        0
    } else {
        item_struct.characters.unwrap().len()
    };
    let num_consm = if item_struct.consumable.is_none() {
        0
    } else {
        item_struct.consumable.unwrap().len()
    };
    let num_mats = if item_struct.materials.is_none() {
        0
    } else {
        item_struct.materials.unwrap().len()
    };
    let num_tra = if item_struct.tradable.is_none() {
        0
    } else {
        item_struct.tradable.unwrap().len()
    };
    let num_weap = if item_struct.weapons.is_none() {
        0
    } else {
        item_struct.weapons.unwrap().len()
    };

    println!(
        "There are {0} Armour pieces, {1} Artifacts, {2} Characters, {3} Consumables, 
    {4} Materials, {5} Tradables, {6} Weapons in the Item data file. ",
        num_armor, num_artfs, num_chars, num_consm, num_mats, num_tra, num_weap
    );
}
