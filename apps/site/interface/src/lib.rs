use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[must_use]
pub fn rust_generate_button_text(status: &str) -> String {
    match status.to_uppercase().as_str() {
        "PENDING" => "edit".to_string(),
        "DONE" => "delete".to_string(),
        _ => "an error has occured".to_string(),
    }
}
