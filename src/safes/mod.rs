
use wasm_bindgen::prelude::*;

use crate::{generate_password, generate_safe_id, safes::models::SafeArgsModel, storage::{JsStorageHandler, StorageHandling}};

pub mod models;
pub mod list;
pub mod safe;
pub mod backup;

#[wasm_bindgen]
pub struct SafeArgs {
    model: SafeArgsModel
}

impl SafeArgs {

    pub fn load<T: StorageHandling>(session_storage: &T) -> Self {
        let mut model = SafeArgsModel {
            service: "".to_string(),
            user: "".to_string(),
            master_key: "".to_string()
        };
        if let Some(json) = session_storage.get("safe-args") {
            if let Ok(obj) = serde_json::from_str::<SafeArgsModel>(&json) {
                model = obj;
            }
        }
        Self { 
            model
        }
    }

    pub fn save<T: StorageHandling>(&self, session_storage: &mut T) {
        if let Ok(json) = serde_json::to_string(&self.model) {
            session_storage.set("safe-args", &json);
        }
    }

}


#[wasm_bindgen]
impl SafeArgs {

    #[wasm_bindgen]
    pub fn js_load(session_storage: &JsStorageHandler) -> Self {
        Self::load(session_storage)
    }

    #[wasm_bindgen]
    pub fn set(&mut self, service: &str, user: &str, master_key: &str) {
        self.model = SafeArgsModel { service: service.to_string(), user: user.to_string(), master_key: master_key.to_string() };
    }

    #[wasm_bindgen]
    pub fn get_service(&self) -> String {
        self.model.service.clone()
    }

    #[wasm_bindgen]
    pub fn get_user(&self) -> String {
        self.model.user.clone()
    }

    #[wasm_bindgen]
    pub fn get_master_key(&self) -> String {
        self.model.master_key.clone()
    }

    #[wasm_bindgen]
    pub fn js_save(&self, session_storage: &mut JsStorageHandler) {
        self.save(session_storage);
    }

    #[wasm_bindgen]
    pub fn generate_password(&self, leak_count: usize) -> String {
        generate_password(&self.model.service, &self.model.user, &self.model.master_key, leak_count)
    }

    #[wasm_bindgen]
    pub fn id(&self) -> String {
        generate_safe_id(&self.model.service, &self.model.user, &self.model.master_key)
    }

}





