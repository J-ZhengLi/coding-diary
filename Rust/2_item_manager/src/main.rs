mod game_data;

use game_data::{Item, ItemType, Rarity};

fn main() {
    let normal_item = Item::builder()
        .name(String::from("Rusted Sword"))
        .description(String::from(
            "Not so sharp, but may causing periodic damages.",
        ))
        .item_type(ItemType::Weapon)
        .rarity(Rarity::Normal)
        .value(300)
        .build()
        .unwrap();

    let legend_item = Item::builder()
        .name(String::from("Heart of Elder Dragon"))
        .description(String::from("Legend says, before the dragon slayer killed a dragon, they will ripped the heart of the dragon and make it as souvenir."))
        .item_type(ItemType::Material)
        .rarity(Rarity::Legendary)
        .value(8_000_000)
        .build()
        .unwrap();

    println!("Debug output of normal item: {:?}", normal_item);

    println!("Debug output of legendary item: {:?}", legend_item);

    let sep_line = (0..50).map(|_| "-").collect::<String>();
    println!(
        "Displaying normal item: \n{1}\n{0}{1}",
        normal_item, sep_line
    );

    println!("Displaying Rarity enum: \n{}", Rarity::Legendary);
}
