use wasm_bindgen::prelude::*;

use crate::{decrypt_text, encrypt_text, safes::{SafeArgs, list::SafeList, models::SafeModel}, storage::{JsStorageHandler, StorageHandling}};

#[wasm_bindgen]
pub struct Safe {
    model: SafeModel
}

impl Safe {

    pub fn get_model_ref(&self) -> &SafeModel {
        &self.model
    }

    pub fn load_encrypted<T: StorageHandling>(id: &str, local_storage: &mut T) -> Self {
        let key = format!("safe-list/{id}").to_string();
        let mut model = SafeModel {
            id: id.to_string(),
            leak_count: 0,
            notes: String::new()
        };
        if let Some(json) = local_storage.get(&key) {
            if let Ok(obj) = serde_json::from_str::<SafeModel>(&json) {
                model = obj;
            }
        }
        Self { model }
    }

    pub fn load<T: StorageHandling>(safe_args: &SafeArgs, local_storage: &mut T) -> Self {
        let id = safe_args.id();
        let key = format!("safe-list/{id}").to_string();
        let mut model = SafeModel {
            id: id.clone(),
            leak_count: 0,
            notes: String::new()
        };
        if let Some(json) = local_storage.get(&key) {
            if let Ok(obj) = serde_json::from_str::<SafeModel>(&json) {
                model = SafeModel { id: obj.id, leak_count: obj.leak_count, notes: decrypt_text(&obj.notes, &safe_args.generate_password(0)) };
            }
        }
        Self { model }
    }

    pub fn save<T: StorageHandling>(&self, safe_args: &SafeArgs, local_storage: &mut T, list: &mut SafeList) {
        let id = safe_args.id();
        let key = format!("safe-list/{id}").to_string();
        let model = SafeModel {
            id: self.model.id.clone(),
            leak_count: self.model.leak_count.clone(),
            notes: encrypt_text(&self.model.notes, &safe_args.generate_password(0))
        };
        if let Ok(json) = serde_json::to_string(&model) {
            local_storage.set(&key, &json);
        }
        if !list.get().contains(&id) {
            list.push(&id);
            list.save(local_storage);
        }
    }

}

#[wasm_bindgen]
impl Safe {

    #[wasm_bindgen]
    pub fn js_load(safe_args: &SafeArgs, local_storage: &mut JsStorageHandler) -> Self {
        Self::load(safe_args, local_storage)
    }

    #[wasm_bindgen]
    pub fn js_load_encrypŧed(id: &str, local_storage: &mut JsStorageHandler) -> Self {
        Self::load_encrypted(id, local_storage)
    }

    #[wasm_bindgen]
    pub fn js_save(&self, safe_args: &SafeArgs, local_storage: &mut JsStorageHandler, list: &mut SafeList) {
        self.save(safe_args, local_storage, list);
    }

    #[wasm_bindgen]
    pub fn id(&self) -> String {
        self.model.id.clone()
    }

    #[wasm_bindgen]
    pub fn set_notes(&mut self, notes: &str) {
        self.model.notes = notes.to_string();
    }

    #[wasm_bindgen]
    pub fn get_notes(&self) -> String {
        self.model.notes.clone()
    }

    #[wasm_bindgen]
    pub fn set_leak_count(&mut self, leak_count: usize) {
        self.model.leak_count = leak_count
    }

    #[wasm_bindgen]
    pub fn get_leak_count(&self) -> usize {
        self.model.leak_count.clone()
    }

}

