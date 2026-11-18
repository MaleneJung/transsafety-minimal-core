
use std::{collections::HashMap, fs, path::Path};

use js_sys::Function;
use wasm_bindgen::prelude::*;

pub trait StorageHandling {
    fn get(&self, key: &str) -> Option<String>;
    fn set(&mut self, key: &str, val: &str);
    fn remove(&mut self, key: &str);
    fn clear(&mut self);
}

#[wasm_bindgen]
pub struct JsStorageHandler {
    get_callback: Function,
    set_callback: Function,
    remove_callback: Function,
    clear_callback: Function
}

impl StorageHandling for JsStorageHandler {

    fn get(&self, key: &str) -> Option<String> {
        if let Ok(returned) = self.get_callback.call1(&JsValue::null(), &JsValue::from_str(key)) {
            return returned.as_string();
        }
        return None
    }

    fn set(&mut self, key: &str, val: &str) {
        self.set_callback.call2(&JsValue::null(), &JsValue::from_str(key), &JsValue::from_str(val)).unwrap();
    }

    fn remove(&mut self, key: &str) {
        self.remove_callback.call1(&JsValue::null(), &JsValue::from_str(key)).unwrap();
    }

    fn clear(&mut self) {
        self.clear_callback.call0(&JsValue::null()).unwrap();
    }

}

#[wasm_bindgen]
impl JsStorageHandler {

    #[wasm_bindgen]
    pub fn new(get: Function, set: Function, remove: Function, clear: Function) -> Self {
        Self { get_callback: get, set_callback: set, remove_callback: remove, clear_callback: clear }
    }

}

pub struct FileStorageHandler {
    base_path: String
}

impl StorageHandling for FileStorageHandler {

    fn get(&self, key: &str) -> Option<String> {
        if let Ok(val) = fs::read_to_string(&self.path(key)) {
            Some(val)
        } else {
            None
        }
    }

    fn set(&mut self, key: &str, val: &str) {
        let path = &self.path(key);
        if let Some(dir) = Path::new(&path).parent() {
            fs::create_dir_all(dir).unwrap();
        }
        fs::write(&path, val).unwrap()
    }

    fn remove(&mut self, key: &str) {
        fs::remove_file(&self.path(key)).unwrap()
    }

    fn clear(&mut self) {
        fs::remove_dir_all(&self.base_path).unwrap()
    }

}

impl FileStorageHandler {

    pub fn new(base_path: &str) -> Self {
        let mut base_path = base_path.to_string();
        if !base_path.ends_with("/") {
            base_path.push_str("/");
        } 
        base_path.push_str(".transsafety-minimal/");
        Self { base_path }
    }

    pub fn path(&self, key: &str) -> String {
        format!("{}{}.json", &self.base_path, key).to_string()
    }

    pub fn get_base_path(&self) -> String {
        self.base_path.to_string()
    }

}

pub struct MemoryStorageHandler {
    entries: HashMap<String, String>
}

impl StorageHandling for MemoryStorageHandler {

    fn get(&self, key: &str) -> Option<String> {
        if let Some(val) = self.entries.get(key) {
            Some(val.to_string())
        } else {
            None
        }
    }

    fn set(&mut self, key: &str, val: &str) {
        if let Some(_) = self.get(key) {
            self.remove(key);
        }
        self.entries.insert(key.to_string(), val.to_string());
    }

    fn remove(&mut self, key: &str) {
        self.entries.remove(key);
    }

    fn clear(&mut self) {
        self.entries = HashMap::new();
    }

}


impl MemoryStorageHandler {

    pub fn new() -> Self {
        Self { entries: HashMap::new() }
    }

}


