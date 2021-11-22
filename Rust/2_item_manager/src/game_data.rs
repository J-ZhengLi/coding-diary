use item_macro::Builder;

pub enum ItemType {
    Armour,
    Weapon,
    Character,
    Consumable,
    Material,
    Artifact
}

pub enum Rarity {
    Normal,
    Special,
    Rare,
    Epic,
    Legendary
}

#[derive(Builder)]
pub struct Item {
    name: String,
    description: String,
    item_type: ItemType,
    rarity: Rarity,
    value: Option<u32>
}