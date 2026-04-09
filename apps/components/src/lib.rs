use wasm_bindgen::prelude::*;

#[inline]
#[must_use]
#[wasm_bindgen]
pub fn rust_generate_button_text(status: &str) -> String {
    match status.to_uppercase().as_str() {
        "PENDING" => "edit".to_owned(),
        "DONE" => "delete".to_owned(),
        _ => "an error has occured".to_owned(),
    }
}
