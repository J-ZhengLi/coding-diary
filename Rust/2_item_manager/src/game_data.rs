use item_macro::{Builder, Show};

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum ItemType {
    Armour,
    Weapon,
    Consumable,
    Material,
    Artifact,
    Tradable,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Rarity {
    Normal,
    Special,
    Rare,
    Epic,
    Legendary,
}

#[allow(dead_code)]
enum Gender {
    Male,
    Female,
    Undefined
}

#[allow(dead_code)]
enum ChaType {
    Worrior,
    Ranger,
    Mage,
    Tank,
    ExoCreature,
}

#[allow(dead_code)]
#[derive(Default)]
struct ChaStatus {
    max_health: u32,
    max_stamina: u32,
    max_mage: u32,
    attack: u32,
    defence: u32,
    crit_rate: u8,
    crit_dmg: u16,
}

#[allow(dead_code)]
#[derive(Builder, Debug)]
pub struct Item {
    name: String,
    description: String,
    item_type: ItemType,
    rarity: Rarity,
    value: Option<u32>,
}

#[allow(dead_code)]
#[derive(Show)]
pub struct Character {
    name: String,
    gender: Gender,
    description: String,
    cha_type: ChaType,
    rarity: Rarity,
    status: ChaStatus,
}
