use std::path::{Path, PathBuf};
const VALHEIM_DATA_PATH: &str = "LocalLow\\IronGate\\Valheim\\characters";

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let mut default_path = PathBuf::from(std::env::var("AppData").unwrap());
    // $appdata$ returns a subfolder, so remove it from the path
    default_path.pop();
    default_path.push(VALHEIM_DATA_PATH);
    println!("reading files from {:?}", default_path);

    // let app = vhr_egui::FolderViewer::with_data_path(default_path);
    let app = vhr_egui::CharacterEditor::with_data_path(default_path);
    // let app = vhr_egui::MyApp::default();
    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        ..Default::default()
    };
    eframe::run_native(Box::new(app), options);
}
