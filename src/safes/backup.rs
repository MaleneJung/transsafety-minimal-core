use wasm_bindgen::prelude::*;

use crate::{safes::{list::SafeList, models::SafeModel, safe::Safe}, storage::{JsStorageHandler, StorageHandling}};

pub type Backup = Vec<SafeModel>;

pub fn export_backup<T: StorageHandling>(local_storage: &mut T) -> String {
    let mut backup = Backup::new();
    let safes = SafeList::load(local_storage);
    for safe_id in safes.get().iter() {
        let safe = Safe::load_encrypted(safe_id, local_storage);
        backup.push(safe.get_model_ref().clone());
    }
    if let Ok(json) = serde_json::to_string(&backup) {
        json
    } else {
        String::new()
    }
}

pub fn import_backup<T: StorageHandling>(json: &str, local_storage: &mut T) {
    let mut list = SafeList::load(local_storage);
    if let Ok(backup) = serde_json::from_str::<Backup>(json) {
        for safe in backup.iter() {
            if !list.get().contains(&safe.id) {
                list.push(&safe.id);
            }
            if let Ok(json) = serde_json::to_string(safe) {
                local_storage.set(&format!("safe-list/{}", &safe.id), &json);
            }
        }
    }
    list.save(local_storage);
}


#[wasm_bindgen]
pub fn js_export_backup(local_storage: &mut JsStorageHandler) -> String {
    export_backup(local_storage)
}

#[wasm_bindgen]
pub fn js_import_backup(json: &str, local_storage: &mut JsStorageHandler) {
    import_backup(json, local_storage);
}
