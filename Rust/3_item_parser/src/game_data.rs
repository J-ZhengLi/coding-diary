use serde::{Deserialize, Serialize};

#[allow(dead_code)]
pub enum ItemType {
    Armour(Item),
    Weapon(Item),
    Consumable,
    Material,
    Artifact,
    Tradable,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
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
    Undefined,
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
    level: u8,
    max_health: u32,
    max_stamina: u32,
    max_mage: u32,
    attack: u32,
    defence: u32,
    crit_rate: u8,
    crit_dmg_p: u16,
}

#[allow(dead_code)]
#[derive(Default, Serialize, Deserialize)]
struct WeaponStatus {
    level: u8,
    damage: u32,
    ex_damage: u32,
    ex_duration: u8,
    crit_rate: u8,
    crit_dmg_p: u16
}

#[allow(dead_code)]
#[derive(Default, Serialize, Deserialize)]
struct ArmourStatus {
    level: u8,
    health: u32,
    defence: u32,
    damage: u32,
    stamina: u32,
    mage: u32,
    crit_rate: u8,
    crit_dmg_p: u16,
    phy_resist_p: u8,
    mag_resist_p: u8
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct Item {
    name: String,
    description: String,
    rarity: Rarity,
    weapon_attr: Option<WeaponStatus>,
    armour_attr: Option<ArmourStatus>,
    value: Option<u32>,
}

#[allow(dead_code)]
pub struct Character {
    name: String,
    gender: Gender,
    description: String,
    cha_type: ChaType,
    rarity: Rarity,
    status: ChaStatus,
}
