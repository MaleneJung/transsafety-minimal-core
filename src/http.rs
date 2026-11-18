use js_sys::Function;
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub url: String,
    pub method: String,
    pub headers: Vec<String>,
    pub body: Option<String>  
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub response_code: usize,
    pub headers: Vec<String>,
    pub body: Option<String>
}

pub trait HTTPHandling {
    fn fetch(&self, req: Request) -> Option<Response>;
}

#[wasm_bindgen]
pub struct JsHTTPHandler {
    fetch_callback: Function
}

impl HTTPHandling for JsHTTPHandler {
    fn fetch(&self, req: Request) -> Option<Response> {
        if let Ok(req_json) = serde_json::to_string(&req) {
            if let Ok(resp_json_val) = self.fetch_callback.call1(&JsValue::null(), &JsValue::from_str(&req_json)) {
                if let Some(resp_json) = resp_json_val.as_string() {
                    if let Ok(resp) = serde_json::from_str(&resp_json) {
                        return Some(resp);
                    }
                }
            }
    
        } 
        None
    }
}

#[wasm_bindgen]
impl JsHTTPHandler {

    #[wasm_bindgen]
    pub fn new(fetch: Function) -> Self {
        Self { fetch_callback: fetch }
    }

}
