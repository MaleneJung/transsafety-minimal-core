use sha2::{Digest, Sha256};
use wasm_bindgen::prelude::*;

const SPECIAL_CHARS: &str = "!_#";

#[wasm_bindgen]
pub fn get_in_range(index: u8, begin: u8, end: u8) -> u8 {
    begin + (index % (end - begin))
}

#[wasm_bindgen]
pub fn get_num_char(from: char) -> char {
    get_in_range(from as u8, 48, 57) as char
}

#[wasm_bindgen]
pub fn get_lower_char(from: char) -> char {
    get_in_range(from as u8, 97, 122) as char
}

#[wasm_bindgen]
pub fn get_higher_char(from: char) -> char {
    get_in_range(from as u8, 65, 90) as char
}

#[wasm_bindgen]
pub fn get_special_char(from: char) -> char {
    let special_char_index = get_in_range(from as u8, 0, SPECIAL_CHARS.len() as u8);
    SPECIAL_CHARS.as_bytes()[special_char_index as usize] as char
}

#[wasm_bindgen]
pub fn generate_password(service: &str, user: &str, master_key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(format!("{service}{user}{master_key}").as_bytes());
    let hash = hasher.finalize();
    let mut password = String::new();
    for i in 0..(hash.len() - 1) {
        let from = hash[i] as char;
        password.push(match i % 4 {
            0_usize => get_lower_char(from),
            1_usize => get_num_char(from),
            2_usize => get_higher_char(from),
            3_usize => get_special_char(from),
            _ => todo!()
        });
    }
    password
}