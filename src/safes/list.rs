use wasm_bindgen::prelude::*;

use crate::{safes::models::SafeListModel, storage::{JsStorageHandler, StorageHandling}};


#[wasm_bindgen]
pub struct SafeList {
    model: SafeListModel
}

impl SafeList {

    pub fn load(local_storage: &dyn StorageHandling) -> Self {
        let mut model = SafeListModel::new();
        if let Some(json) = local_storage.get("safe-list") {
            if let Ok(obj) = serde_json::from_str::<SafeListModel>(&json) {
                model = obj;
            }
        }
        Self { model }
    }    

    pub fn save(&self, local_storage: &mut dyn StorageHandling) {
        if let Ok(json) = serde_json::to_string(&self.model) {
            local_storage.set("safe-list", &json);
        }
    }

}

#[wasm_bindgen]
impl SafeList {

    #[wasm_bindgen]
    pub fn js_load(local_storage: &JsStorageHandler) -> Self {
        Self::load(local_storage)
    }

    #[wasm_bindgen]
    pub fn js_save(&self, local_storage: &mut JsStorageHandler) {
        self.save(local_storage);
    } 

    #[wasm_bindgen]
    pub fn get(&self) -> Box<[String]> {
        self.model.clone().into_boxed_slice()
    } 

    #[wasm_bindgen]
    pub fn push(&mut self, val: &str) {
        self.model.push(val.to_string());
    }

    #[wasm_bindgen]
    pub fn remove(&mut self, val: &str) {
        if let Some(index) = self.model.iter().position(|entry| entry.to_string() == val.to_string()) {
            self.model.remove(index);
        }
    }

}


