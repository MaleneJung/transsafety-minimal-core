use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SafeArgsModel {
    pub service: String,
    pub user: String,
    pub master_key: String    
}

pub type SafeListModel = Vec<String>;

#[derive(Serialize, Deserialize, Clone)]
pub struct SafeModel {
    pub id: String,
    pub leak_count: usize,
    pub notes: String
}