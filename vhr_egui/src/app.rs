use eframe::{egui, epi};

use std::path::{Path, PathBuf};

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct CharacterEditor;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
#[cfg(not(target_arch = "wasm32"))]
pub struct FolderViewer {
    folder: PathBuf,
    files: Vec<PathBuf>,
    selected: Option<PathBuf>,
}

impl FolderViewer {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn with_data_path(folder: PathBuf) -> FolderViewer {
        use std::ffi::OsStr;
        use std::fs::read_dir;
        let files: Vec<PathBuf> = if let Ok(paths) = read_dir(folder.clone()) {
            paths
                .filter_map(|p| p.ok())
                // .map(|p| {
                //     p.file_name().to_string_lossy().to_string()
                // })
                .filter_map(|de| {
                    let p = de.path();
                    if let Some("fch") = p.extension().and_then(OsStr::to_str) {
                        Some(p)
                    } else {
                        None
                    }
                })
                // .filter(|name|name.ends_with(".fch") )
                // todo: also include .olds? || name.ends_with(".fch.old")
                // .map(|name| {
                //     let parts = name.split('.');
                //     let part_count = parts.clone().count();
                //     let s: Vec<&str> = parts.take(part_count-1).collect();
                //     s.join(".")
                // })
                .collect()
            // println!("populating files list: {:?}", self.files_found);
        } else {
            // println!("couldnt open folder: {:?}", folder_path);
            Vec::new()
        };
        FolderViewer {
            folder,
            files,
            selected: None,
        }
    }
}

impl epi::App for FolderViewer {
    fn name(&self) -> &str {
        "VHR Character Editor"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self {
            folder,
            files,
            selected,
        } = self;
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            // ui.horizontal(|ui| {
            //     ui.label("Write something: ");
            //     ui.text_edit_singleline(label);
            // });

            // ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            // if ui.button("Increment").clicked() {
            //     *value += 1.0;
            // }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    egui::warn_if_debug_build(ui);
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("list of files found");
            ui.label(folder.to_string_lossy().to_string());

            let mut selected_value: PathBuf = selected.clone().unwrap_or(PathBuf::new());

            for path in files.iter() {
                let shortname = path
                    .file_stem()
                    .map(|os| os.to_string_lossy().to_string())
                    .unwrap_or(String::new());
                ui.selectable_value(&mut selected_value, path.clone(), shortname);
            }

            let would_load = selected_value.clone();
            let mut load_button = egui::Button::new("Load File");
            if selected_value != PathBuf::new() {
                *selected = Some(selected_value.clone());
                load_button = load_button.enabled(true);
            } else {
                load_button = load_button.enabled(false);
            }
            let load_button = ui.add(load_button);

            if load_button.clicked() {
                ui.label("loading...".to_string());
            }
            ui.label(format!("file to load: {:?}", would_load));
        });
    }
}
