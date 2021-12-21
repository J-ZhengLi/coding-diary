//! This file is a demostration of using mutable global variables,
//! so... in case you are wondering why not keeping all those things in a struct
//! or some type of structure, well... this is the reason.

use std::borrow::BorrowMut;
use std::io::{self, Write};
use std::sync::{RwLock, Mutex, Once};
use std::sync::atomic::{AtomicU16, AtomicU8, AtomicBool, Ordering::SeqCst};
use once_cell::sync::{Lazy, OnceCell};
use lazy_static::lazy_static;
use rand::prelude::SliceRandom;
use rand::thread_rng;

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
static LEVEL: AtomicU16 = AtomicU16::new(0);

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

/// A static cell that holds all names from name_list file,
/// using OnceCell can garantee the file will only be loaded once
static ALL_NAMES: OnceCell<Vec<&str>> = OnceCell::new();

static mut NOTE: Option<Mutex<String>> = None;
static NOTE_TO_PLAYER: Once = Once::new();

/// # Engine
/// 
/// `start` function is called once before game loop
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

/// # Engine
/// 
/// `update` function is called in every frame
fn update(body: fn() -> Option<i32>) {
    'game: loop {
        if let Some(res) = body() {
            match res {
                0 => { println!("Game exit successfully!"); },
                _ => { println!("Game exit with error code: {}", res); }
            };
            break 'game;
        }
    }
}

fn note_to_player<'a>(msg: &str) -> &'a Mutex<String> {
    unsafe {
        NOTE_TO_PLAYER.call_once(|| {
            *NOTE.borrow_mut() = Some(Mutex::new(msg.to_string()));
        });

        NOTE.as_ref().unwrap()
    }
}

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

/// Get a random name from name_list
fn random_name() -> String {
    let names = ALL_NAMES.get_or_init(|| {
        include_str!("../res/name_list").split(' ').collect()
    });

    match names.choose(&mut thread_rng()) {
        Some(name) => name.to_string(),
        None => String::from("???")
    }
}

fn attack_enemy() {
    let mut enemies = ENEMY_NAMES.lock().unwrap();
    if !enemies.is_empty() {
        let killed = enemies.pop().unwrap();
        println!("You killed \'{}\'!", killed);
        KILLED_ENEMIES.lock().unwrap().push(killed);
    }
}

fn main() {
    println!(
        "Welcome to another text based small game~ \
        \nEnter [A] to attack. \
        \nEnter [Q] to quit. \
        \nEnter [S] to show game stats."
    );

    // start function is called before game loop
    start(|| {
        IS_SPECTATING.store(false, SeqCst);

        println!("{}", note_to_player("Game Started!").lock().unwrap());
    });

    // update function called each frame
    update(|| {
        let mut enemies = ENEMY_NAMES.lock().unwrap();
        if enemies.is_empty() {
            LEVEL.fetch_add(1, SeqCst);
            for _ in 0..MAX_ENEMY_COUNT.load(SeqCst) {
                let enemy = random_name();
                println!("Spawning enemy: {}", enemy);
                enemies.push(enemy);
            }
        }

        let _ = io::stdout().flush();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Fail to read input.");

        // deal with input
        match input.trim() {
            "a" | "A" => { attack_enemy(); },
            "s" | "S" => { println!("{}", get_stats_string()); },
            "q" | "Q" => { return Some(0); },
            _ => {}
        };

        None
    });
}
