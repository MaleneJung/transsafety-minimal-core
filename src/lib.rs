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

#[wasm_bindgen]
pub fn u8s_to_hex(v: &[u8]) -> String {
    hex::encode(v)
}

#[wasm_bindgen]
pub fn hex_to_u8s(h: &str) -> Vec<u8> {
    hex::decode(h).unwrap()
}

#[wasm_bindgen]
pub fn cypher_byte(base: u8, add: u8) -> u8 {
    let mut encrypted: u16 = base as u16 + add as u16;
    while encrypted >= 256 {
        encrypted -= 256;
    }
    encrypted as u8
}

#[wasm_bindgen]
pub fn decypher_byte(base: u8, sub: u8) -> u8 {
    let mut decrypted: i16 = base as i16 - sub as i16;
    while decrypted < 0 {
        decrypted += 256;
    }
    decrypted as u8
}

#[wasm_bindgen]
pub fn generate_cypher_key(key: &str, len: usize) -> Vec<u8> {
    let mut cypher_key: Vec<u8> = Vec::new();
    let mut cypher_index: usize = 0;
    while cypher_key.len() < len {
        let mut hasher = Sha256::new();
        let i = cypher_index.to_string();
        hasher.update(format!("{i}-{key}").as_bytes());
        let hash = hasher.finalize();
        cypher_key.append(&mut hash.to_vec());
        cypher_index += 1;
    }
    cypher_key
} 

#[wasm_bindgen]
pub fn encrypt_text(text: &str, key: &str) -> String {
    let mut bytes: Vec<u8> = text.as_bytes().to_vec();
    let cypher_key: Vec<u8> = generate_cypher_key(key, bytes.len());
    for i in 0..(bytes.len() - 1) {
        bytes[i] = cypher_byte(bytes[i], cypher_key[i]);
    }
    u8s_to_hex(&bytes)
}

#[wasm_bindgen]
pub fn decrypt_text(text: &str, key: &str) -> String {
    let mut bytes: Vec<u8> = hex_to_u8s(text);
    let cypher_key: Vec<u8> = generate_cypher_key(key, bytes.len());
    for i in 0..(bytes.len() - 1) {
        bytes[i] = decypher_byte(bytes[i], cypher_key[i]);
    } 
    String::from_utf8_lossy(&bytes).to_string()
}