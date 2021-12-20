//! This file is a demostration of using mutable global variables,
//! so... in case you are wondering why not keeping all those things in a struct
//! or some type of structure, well... this is the reason.

use std::io::{self, Write};
use std::sync::{RwLock, Mutex, Once};
use std::sync::atomic::{AtomicU16, AtomicU8, AtomicBool, Ordering::SeqCst};
use once_cell::sync::{Lazy, OnceCell};
use lazy_static::lazy_static;

/// Define the current state of player, when this equals to false meaning that the
/// player is in an in-game state.
static IS_SPECTATING: AtomicBool = AtomicBool::new(true);

/// Define how many enemies can be in one level, which will change in each level.
static MAX_ENEMY_COUNT: AtomicU8 = AtomicU8::new(2);

/// Global character level, requires mutability for leveling up.
static CHAR_LEVEL: AtomicU16 = AtomicU16::new(1);

/// Character health will be updated each turn
static CHAR_HEALTH: AtomicU16 = AtomicU16::new(1000);

/// Current number of level.
static LEVEL: AtomicU16 = AtomicU16::new(1);

/// Character title, a static variable with mutable string wrapped in Rwlock,
/// which given the ability of syncronized reading
static CHAR_TITLE: Lazy<RwLock<String>> = Lazy::new(|| {
    RwLock::new("Newbie".to_string())
});

/// Inlines a vector of names of enemies in current level,
/// using Mutex because it does not need syncronized reading.
static ENEMY_NAMES: Lazy<Mutex<Vec<String>>> = Lazy::new(|| {
    Mutex::new(
        Vec::<String>::new()
    )
});

lazy_static! {
    /// Holds the name of enemies that you brutaly killed :(
    static ref KILLED_ENEMIES: Mutex<Vec<String>> = Mutex::new(
        Vec::<String>::new()
    );
}

/// Determain whether is the current game running or not.
/// 
/// OnceCell garantee that the variable is only initialized once,
/// and in this case in order to change the variable we just need to replace it with a new cell.
static GAME_RUNNING: OnceCell<bool> = OnceCell::new();

static mut NOTE: Option<String> = None;
static NOTE_TO_PLAYER: Once = Once::new();

fn get_stats_string() -> String {
    let local_killed_enemies = KILLED_ENEMIES.lock().unwrap();
    format!(
        "Current Level: {} \
        \nPlayer Level: {} - [{}] \
        \nPlayer Health: {} \
        \nEnemy Remaining: {} \
        \nEnemies Killed: {:?}",
        LEVEL.load(SeqCst),
        CHAR_LEVEL.load(SeqCst), CHAR_TITLE.read().unwrap(),
        CHAR_HEALTH.load(SeqCst),
        MAX_ENEMY_COUNT.load(SeqCst) - local_killed_enemies.len() as u8,
        local_killed_enemies
    )
}

/// **[Engine]** Start the game logic
/// 
/// This is a framwork function that takes another function as argument, and execute the content
/// of that function if the game is not started yet, then set the GAME_RUNNING state to true.
fn start(initializaion: fn()) {
    // make sure the logic code of this function will only be executed once
    if GAME_RUNNING.get().is_none() {
        initializaion()
    }

    let _ = GAME_RUNNING.get_or_init(|| true);
}

fn update(body: fn() -> Option<u8>) {
    'game: loop {
        if let Some(res) = body() {
            match res {
                0 => {
                    println!("Game exit successfully!");
                    break 'game;
                },
                _ => unimplemented!()
            };
        }
    }
}

fn main() {
    println!(
        "Welcome to another text based small game~ \
        \nEnter [Q] to quit. \
        \nEnter [S] to show game stats."
    );

    // start function is called before game loop
    start(|| {
        IS_SPECTATING.store(false, SeqCst);

        println!("Game started!");
    });

    // update function called each frame
    update(|| {
        print!("> ");
        let _ = io::stdout().flush();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Fail to read input.");

        match input.trim() {
            "s" | "S" => { println!("{}", get_stats_string()); },
            "q" | "Q" => { return Some(0); },
            _ => {}
        };

        None
    });
}
