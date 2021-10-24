#![warn(clippy::all, rust_2018_idioms)]
mod app;
pub use app::CharacterEditor;
#[cfg(not(target_arch = "wasm32"))]
pub use app::FolderViewer;

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

/// wasm entry-point
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    let app = CharacterEditor::default();
    eframe::start_web(canvas_id, Box::new(app))
}
