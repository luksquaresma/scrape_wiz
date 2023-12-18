use std::any::TypeId;

use serde::{Deserialize, Serialize};

pub trait NamedEnum {
    fn get_name(&self) -> String;
    fn from_name(name: &String) -> Self;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Configs {
    pub search_config_path: &'static str,
}

pub fn compare_types<T: ?Sized + 'static, U: ?Sized + 'static>(_x: &T, _y: &U) -> bool {
    if std::any::TypeId::of::<T>() == std::any::TypeId::of::<U>() {
        return true;
    } else {
        return false;
    }
}

pub fn compare_to_typeid<T: ?Sized + 'static>(_x: &T, u: TypeId) -> bool {
    if std::any::TypeId::of::<T>() == u {
        return true;
    } else {
        return false;
    }
}

pub fn print_separator(level: u32) {
    let separator = match level {
        0 => "###",
        1 => "===",
        2 => "---",
        3 => "...",
        _ => panic!("Impossible separator level found!"),
    };
    println!("\n{}", separator.repeat(SEPARATOR_SIZE));
}

pub const SEPARATOR_SIZE: usize = 20;

pub const CONFIG_FILE_TEST: &'static str = "./src/tests/config.json";

pub const TESTING_SEARCH_KEYWORDS: [&'static str; 5] = [
    "This is the testing array",
    "Each phrase shoud be spaced and clear",
    "No puctuation should be observed",
    "Spaces should be single",
    "Simbols should not be oberved",
];
