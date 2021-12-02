mod rng_helper;

use rng_helper::random;
use std::io::{self, Write};

// example database for items with different rarity
static LEGENDARY_ITEMS: [&str; 15] = [
    "Mona",
    "Diluc",
    "Klee",
    "Jean",
    "Venti",
    "Albedo",
    "Keqing",
    "Xiao",
    "Qiqi",
    "Zhongli",
    "Ganyu",
    "Raiden Shogun",
    "Yomiya",
    "Itto",
    "Kamisato Ayaka",
];
static EPIC_ITEMS: [&str; 20] = [
    "Barbara",
    "Amber",
    "Bennet",
    "Sucross",
    "Razor",
    "Xiangling",
    "Xingqiu",
    "Chongyun",
    "Fischle",
    "Xinyan",
    "Yanfei",
    "Ningguang",
    "Black Sword",
    "The Flute",
    "Stringless",
    "Rust",
    "Prototype Sword",
    "Prototype Great Sword",
    "Prototype Bow",
    "Prototype Spear",
];
static RARE_ITEMS: [&str; 15] = [
    "Black Steel Set - Head",
    "Black Steel Set - Body",
    "Black Steel Set - Torso",
    "Black Steel Set - Foot",
    "Black Steel Set - Cape",
    "Black Steel Sword",
    "Black Steel Shield",
    "Black Steel Sword",
    "Ruby Necklace - ATK%",
    "Sapphire Ring - MAGIC%",
    "Moonlight Wand",
    "The Invader",
    "Debate Club",
    "Alsa's Cape",
    "Staff of Walnut",
];
static SPECIAL_ITEMS: [&str; 15] = [
    "Silver Sword",
    "Silver Helmet",
    "Silver Armor",
    "Silver Shield",
    "Silver Boots",
    "Steel Sword",
    "Steel Shield",
    "Silver Helmet",
    "Silver Armor",
    "Silver Boots",
    "Wand of Gandoff",
    "Hat of Agatha",
    "Cape of Agatha",
    "Boots of Agatha",
    "The Charger",
];
static NORMAL_ITEMS: [&str; 10] = [
    "Sword",
    "Wooden Staff",
    "Magic Wand",
    "Bow",
    "Free Lance",
    "Leather Coat",
    "Leather Pants",
    "Leather Shoes",
    "Leather Hat",
    "Wedding Ring",
];

#[allow(dead_code)]
enum Color {
    Red,
    Yellow,
    Purple,
    Blue,
    Cyan,
    White,
}
enum Rarity {
    Legendary,
    Epic,
    Rare,
    Special,
    Normal,
}

impl Rarity {
    fn stringify(&self) -> &str {
        match self {
            Rarity::Legendary => "Legendary",
            Rarity::Epic => "Epic",
            Rarity::Rare => "Rare",
            Rarity::Special => "Special",
            Rarity::Normal => "Normal",
        }
    }
}

struct Item {
    name: String,
    rarity: Rarity,
}
impl Default for Item {
    fn default() -> Item {
        Item {
            name: String::new(),
            rarity: Rarity::Normal,
        }
    }
}
impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let res_str = match self.rarity {
            Rarity::Legendary => coloring_str(self.name.as_str(), Color::Yellow),
            Rarity::Epic => coloring_str(self.name.as_str(), Color::Purple),
            Rarity::Rare => coloring_str(self.name.as_str(), Color::Blue),
            Rarity::Special => coloring_str(self.name.as_str(), Color::Cyan),
            Rarity::Normal => coloring_str(self.name.as_str(), Color::White),
        };

        write!(f, "{}", res_str)
    }
}

/// Change the color of string literal with specified color in Unix system
/// 
/// # Example
/// ```rust
/// let red_bold_hello = coloring_str("Hello!", Color::Red);
/// assert_eq!(red_bold_hello, "\x1b[31;1mHello!\x1b[0m");
/// println!("{}", red_bold_hello); // prints a "Hello!" message in red
/// ```
fn coloring_str(word: &str, color: Color) -> String {
    match color {
        Color::Red => format!("\x1b[31;1m{}\x1b[0m", word),
        Color::Yellow => format!("\x1b[33;1m{}\x1b[0m", word),
        Color::Purple => format!("\x1b[35;1m{}\x1b[0m", word),
        Color::Blue => format!("\x1b[34;1m{}\x1b[0m", word),
        Color::Cyan => format!("\x1b[36;1m{}\x1b[0m", word),
        Color::White => format!("\x1b[37;1m{}\x1b[0m", word),
    }
}

/// Pull `count` times and return the item being pulled along
/// with how many resource refunded.
fn pull(count: u8) -> (Vec<Item>, u8) {
    const MAX_OF_LEGENDARY: f64 = 0.05;
    const MAX_OF_EPIC: f64 = 0.08 + MAX_OF_LEGENDARY;
    const MAX_OF_RARE: f64 = 0.12 + MAX_OF_EPIC;
    const MAX_OF_SPECIAL: f64 = 0.15 + MAX_OF_RARE;

    const CHANCE_OF_REFUND: f64 = 0.3;

    let mut refund_count: u8 = 0;
    let mut items_rolled: Vec<Item> = vec![];

    for _ in 0..count {
        let mut item = Item::default();

        // generate a float between 0-1
        let item_roll_num: f64 = random::float();
        
        if item_roll_num < MAX_OF_LEGENDARY {
            item.rarity = Rarity::Legendary;
            item.name = random::element_in_slice(&LEGENDARY_ITEMS)
                .unwrap()
                .to_string();
        } else if item_roll_num < MAX_OF_EPIC {
            item.rarity = Rarity::Epic;
            item.name = random::element_in_slice(&EPIC_ITEMS).unwrap().to_string();
        } else if item_roll_num < MAX_OF_RARE {
            item.rarity = Rarity::Rare;
            item.name = random::element_in_slice(&RARE_ITEMS).unwrap().to_string();
        } else if item_roll_num < MAX_OF_SPECIAL {
            item.rarity = Rarity::Special;
            item.name = random::element_in_slice(&SPECIAL_ITEMS)
                .unwrap()
                .to_string();
        } else {
            item.rarity = Rarity::Normal;
            item.name = random::element_in_slice(&NORMAL_ITEMS).unwrap().to_string();
        }

        // generate a float between 0 - 1 for wether refund the material used or not
        if random::float() < CHANCE_OF_REFUND {
            refund_count += 1;
        }

        items_rolled.push(item);
    }

    (items_rolled, refund_count)
}

fn main() {
    const NAME_OF_TICKET: &str = "Ticket";

    let mut ticket_count: u32 = 100;

    let rule_msg: String = format!(
        "
-----------------------------------------------------------------------------
            Hello! Welcome to my very boring gotcha game.
    You have {0} {1}s left to play, each pull uses one {1}
    for a chance to get one {2}/{3}/{4}/{5}/{6} item.
    Additionaly, you'll have a slight changce getting your {1} back~
    Enjoy!
-----------------------------------------------------------------------------",
        ticket_count,
        NAME_OF_TICKET,
        coloring_str(Rarity::Legendary.stringify(), Color::Yellow),
        coloring_str(Rarity::Epic.stringify(), Color::Purple),
        coloring_str(Rarity::Rare.stringify(), Color::Blue),
        coloring_str(Rarity::Special.stringify(), Color::Cyan),
        coloring_str(Rarity::Normal.stringify(), Color::White)
    );

    let help_msg: String = format!(
        "
    Enter 
        \"{0}\": to show game rules.
        \"{1}\": to show this help message.
        \"{2}\": to do ten pulls at once.
        \"{3}\": to exit the game.
    Or enter nothing for a single pull.
        ",
        "r", "h", "10", "e"
    );

    println!("{}", rule_msg);
    println!("{}", help_msg);

    // main game loop
    'game: loop {
        let mut input = String::new();

        print!("{0} {1}(s) left: ", ticket_count, NAME_OF_TICKET);
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut input)
            .expect("Fail to read input, try again.");

        match input.trim() {
            "" => {
                if ticket_count > 0 {
                    let (res, refund) = pull(1);
                    if refund > 0 {
                        println!(
                            "Pulled: [{0}] (You got a {1} back!)",
                            res[0],
                            NAME_OF_TICKET
                        );
                    } else {
                        ticket_count -= 1;
                        println!("Pulled: [{}]", res[0]);
                    }
                } else {
                    println!(
                        "{}",
                        coloring_str(
                            format!("Sorry, you ran out of {}s", NAME_OF_TICKET).as_str(),
                            Color::Red
                        )
                    );
                    break 'game;
                }
            }
            "r" => {
                println!("{}", rule_msg);
            }
            "h" => {
                println!("{}", help_msg);
            }
            "10" => {
                if ticket_count >= 10 {
                    let (res, refund) = pull(10);
                    let mut res_str = String::new();
                    for r in &res {
                        res_str = format!("{}, {}", res_str, &r);
                    }

                    if refund > 0 {
                        println!(
                            "Pulled: [{0}] (You got {2} {1}s back!)",
                            res_str, NAME_OF_TICKET, refund
                        );
                        ticket_count = ticket_count - 10 + refund as u32;
                    } else {
                        println!("Pulled: [{}]", res_str);
                        ticket_count -= 10;
                    }
                } else {
                    println!(
                        "{}",
                        coloring_str(
                            format!("You dont have enough {}s", NAME_OF_TICKET).as_str(),
                            Color::Red
                        )
                    );
                }
            }
            "e" => {
                break 'game;
            }
            _ => {
                println!("Not a valid option, try again.");
            }
        }
    }
}
