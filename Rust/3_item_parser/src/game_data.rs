use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Items {
    pub armours: Option<Vec<Item>>,
    pub artifacts: Option<Vec<Item>>,
    pub characters: Option<Vec<Character>>,
    pub consumable: Option<Vec<Item>>,
    pub materials: Option<Vec<Item>>,
    pub tradable: Option<Vec<Item>>,
    pub weapons: Option<Vec<Item>>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Rarity {
    Normal,
    Special,
    Rare,
    Epic,
    Legendary,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
enum Gender {
    Male,
    Female,
    Undefined,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
enum ChaType {
    Worrior,
    Ranger,
    Mage,
    Tank,
    Demon,
}

#[allow(dead_code)]
#[derive(Default, Serialize, Deserialize, Debug)]
struct WeaponStatus {
    level: u8,
    damage: u32,
    ex_damage: u32,
    ex_duration: u8,
    crit_rate: u8,
    crit_dmg: u16,
}

#[allow(dead_code)]
#[derive(Default, Serialize, Deserialize, Debug)]
struct ArmourStatus {
    level: u8,
    health: u32,
    defence: u32,
    damage: u32,
    stamina: u32,
    mana: u32,
    crit_rate: u8,
    crit_dmg: u16,
    phy_resist: u8,
    mag_resist: u8,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub enum Position {
    Head,
    Chest,
    Leg,
    Shoe,
    Weapon,
    Shield,
    Hand,
    Necklace,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Artifact {
    position: Position,
    rarity: Rarity,
    damage: u32,
    health: u32,
    defence: u32,
    crit_rate: u8,
    crit_dmg: u16,
    phy_resist: u8,
    mag_resist: u8,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    name: String,
    description: String,
    rarity: Rarity,
    weapon_attr: Option<WeaponStatus>,
    armour_attr: Option<ArmourStatus>,
    value: Option<u32>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Character {
    name: String,
    gender: Gender,
    rarity: Rarity,
    description: String,
    cha_type: ChaType,
    level: u8,
    max_health: u32,
    max_stamina: u32,
    max_mana: u32,
    crit_rate: u8,
    crit_dmg: u16,
    phy_resist: u8,
    mag_resist: u8,
}
