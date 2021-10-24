use eframe::{egui, epi};
use epi::App;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{Read, Seek, Write};
use std::path::{Path, PathBuf};

use vhr_datatypes::character::*;
use vhr_datatypes::map::{hashed_string::HashedString, Entity, MapDatabaseFile};
use vhr_datatypes::prelude::*;

pub enum EditorEvent {
    SwapState(CharacterEditor),
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub enum CharacterEditor {
    Preload(FileDialog),
    Loading(Loader),
    Loaded(CharacterDialog),
    // Modified(CharacterDialog),
}

impl CharacterEditor {
    pub fn with_data_path(path: PathBuf) -> CharacterEditor {
        CharacterEditor::Preload(FileDialog {
            default_path: Some(path),
            dropped_files: Vec::new(),
            // picked_path: None,
        })
    }
}

impl Default for CharacterEditor {
    fn default() -> CharacterEditor {
        CharacterEditor::Preload(FileDialog {
            default_path: None,
            dropped_files: Vec::new(),
            // picked_path: None,
        })
    }
}

impl epi::App for CharacterEditor {
    fn name(&self) -> &str {
        "VHR_Character_Editor"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        use CharacterEditor::*;
        let event = match self {
            Preload(state) => state.update(ctx, frame),
            Loading(state) => state.update(ctx, frame),
            Loaded(state) => state.update(ctx, frame),
        };
        if let Some(event) = event {
            use EditorEvent::*;
            match event {
                SwapState(new_state) => *self = new_state,
            }
        }
    }
}

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct FileDialog {
    default_path: Option<PathBuf>,
    dropped_files: Vec<egui::DroppedFile>,
    // picked_path: Option<PathBuf>,
}

impl FileDialog {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) -> Option<EditorEvent> {
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

        let bubble_event = egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Drag-and-drop files onto the window!");

            if cfg!(target_os = "macos") {
                // Awaiting fix of winit bug: https://github.com/rust-windowing/winit/pull/2027
            } else if ui.button("Open file…").clicked() {
                let mut file_picker = rfd::FileDialog::new();
                if let Some(path) = &self.default_path {
                    file_picker = file_picker.set_directory(path);
                }
                if let Some(path) = file_picker.pick_file() {
                    let next_state = CharacterEditor::Loading(Loader(path));
                    return Some(EditorEvent::SwapState(next_state));
                }
            }

            // if let Some(picked_path) = &self.picked_path {
            //     ui.horizontal(|ui| {
            //         ui.label("Picked file:");
            //         ui.monospace(picked_path.display().to_string());
            //     });
            // }

            // Show dropped files (if any):
            if !self.dropped_files.is_empty() {
                // todo: dont panic if path is not found, indicates a buf in path resolving
                let path = self.dropped_files.first().unwrap().clone().path.unwrap();
                let next_state = CharacterEditor::Loading(Loader(path));
                return Some(EditorEvent::SwapState(next_state));
                // ui.group(|ui| {
                //     ui.label("Dropped files:");

                //     for file in &self.dropped_files {
                //         let mut info = if let Some(path) = &file.path {
                //             path.display().to_string()
                //         } else if !file.name.is_empty() {
                //             file.name.clone()
                //         } else {
                //             "???".to_owned()
                //         };
                //         if let Some(bytes) = &file.bytes {
                //             info += &format!(" ({} bytes)", bytes.len());
                //         }
                //         ui.label(info);
                //     }
                // });
            }
            return None;
        });

        self.detect_files_being_dropped(ctx);
        return bubble_event.inner;
    }

    fn detect_files_being_dropped(&mut self, ctx: &egui::CtxRef) {
        use egui::*;

        // Preview hovering files:
        if !ctx.input().raw.hovered_files.is_empty() {
            let mut text = "Dropping files:\n".to_owned();
            for file in &ctx.input().raw.hovered_files {
                if let Some(path) = &file.path {
                    text += &format!("\n{}", path.display());
                } else if !file.mime.is_empty() {
                    text += &format!("\n{}", file.mime);
                } else {
                    text += "\n???";
                }
            }

            let painter =
                ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

            let screen_rect = ctx.input().screen_rect();
            painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
            painter.text(
                screen_rect.center(),
                Align2::CENTER_CENTER,
                text,
                TextStyle::Heading,
                Color32::WHITE,
            );
        }

        // Collect dropped files:
        if !ctx.input().raw.dropped_files.is_empty() {
            self.dropped_files = ctx.input().raw.dropped_files.clone();
        }
    }
}

/*

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

*/

pub struct Loader(PathBuf);

impl Loader {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) -> Option<EditorEvent> {
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

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("loading file {}", self.0.display()));
        });

        let mut loaded_file = Vec::new();
        {
            let mut in_file = File::open(&self.0).expect("Failed opening file");
            in_file.read_to_end(&mut loaded_file).ok();
        }

        let character =
            vhr_serde::de::from_bytes::<CharacterFile>(&loaded_file);
        return match character {
            Ok(loaded_file) => {
                let next_state = CharacterEditor::Loaded(CharacterDialog {
                    loaded_file,
                    current_tab: Tabs::Stats,
                    name: None,
                });
                Some(EditorEvent::SwapState(next_state))
            },
            Err(e) => {
                todo!("couldnt load file, try another and revert to file picker state?");
                None
            }
        };

        

        // return None;
    }
}

#[derive(PartialEq, Copy, Clone)]
enum Tabs {
    Stats,
    Appearance,
    Inventory,
    Maps,
    Compendium,
    HexView,
}

pub struct CharacterDialog {
    loaded_file: CharacterFile,
    current_tab: Tabs,
    name: Option<String>,
}

impl CharacterDialog {
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) -> Option<EditorEvent> {
        
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

        let character = &mut self.loaded_file;
        if let Some(profile) = match &mut character.inner {
            PlayerProfile::ThirtyThree(profile) => Some(profile),
            PlayerProfile::ThirtySix(profile) => Some(profile),
            _ => None,
        } {
            // stats tab
            // let mut stats: Vec<(String, String)> = Vec::new();
            // stats.push(("Kill Count".to_string(), profile.kill_count.to_string()));
            // stats.push(("Death Count".to_string(), profile.death_count.to_string()));
            // stats.push(("Crafting Count".to_string(), profile.crafting_count.to_string()));
            // stats.push(("Building Count".to_string(), profile.building_count.to_string()));
            // stats.push(("Name".to_string(), profile.name.to_owned()));
            // stats.push(("Id".to_string(), profile.player_id.to_string()));
        
        if let None = self.name {
            if profile.name.len() > 0 {
                self.name = Some(profile.name.to_owned())
            }
        }
        let mut c_tab = self.current_tab;
        egui::CentralPanel::default().show(ctx,  |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut c_tab, Tabs::Stats, "Stats");
                ui.selectable_value(
                    &mut c_tab,
                    Tabs::Appearance,
                    "Appearance",
                );
                ui.selectable_value(
                    &mut c_tab,
                    Tabs::Inventory,
                    "Inventory",
                );
                ui.selectable_value(
                    &mut c_tab,
                    Tabs::Maps,
                    "Maps",
                );
                ui.selectable_value(
                    &mut c_tab,
                    Tabs::Compendium,
                    "Compendium",
                );
                ui.selectable_value(&mut c_tab, Tabs::HexView, "Hex View");
            });
            
            ui.separator();
            // match self.demo {
            //     ScrollDemo::ScrollTo => {
            //         self.scroll_to.ui(ui);
            //     }
            //     ScrollDemo::ManyLines => {
            //         huge_content_lines(ui);
            //     }
            //     ScrollDemo::LargeCanvas => {
            //         huge_content_painter(ui);
            //     }
            //     ScrollDemo::StickToEnd => {
            //         self.scroll_stick_to.ui(ui);
            //     }
            // }

            egui::Grid::new("my_grid_id")
                .num_columns(2)
                .spacing([40.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.add( egui::Label::new("Name"));
                    // todo: add validation, limit length, minimum length, etc
                    ui.add(egui::TextEdit::singleline(&mut profile.name).hint_text("Character's Name"));
                    ui.end_row();

                    ui.add( egui::Label::new("Id"));
                    // todo: add validation, must be integer, etc
                    let mut id_string = profile.player_id.to_string();
                    ui.add(egui::TextEdit::singleline(&mut id_string).hint_text("Character's ID"));
                    if let Some(id) = id_string.parse::<u64>().ok() {
                        profile.player_id = id;
                    }
                    ui.end_row();

                    ui.add( egui::Label::new("Kill Count"));
                    ui.add(egui::Slider::new(&mut profile.kill_count, 0..=100));
                    ui.end_row();

                    ui.add( egui::Label::new("Death Count"));
                    ui.add(egui::Slider::new(&mut profile.death_count, 0..=1000));
                    ui.end_row();

                    ui.add( egui::Label::new("Craft Count"));
                    ui.add(egui::Slider::new(&mut profile.crafting_count, 0..=5000));
                    ui.end_row();
                    ui.add( egui::Label::new("Build Count"));
                    ui.add(egui::Slider::new(&mut profile.building_count, 0..=20000));
                    ui.end_row();

                    
                });
        });
        if c_tab != self.current_tab {
                self.current_tab = c_tab;
            }
        // maps tab
            // let maps: &Vec<Map> = &profile.maps;
            // for (i, map) in maps.iter().enumerate() {
            //     println!("Map # {}", i);
            //     println!("\tid: {}", map.id);
            //     println!("\tspawn point: {:?}", map.spawn);
            //     println!("\tcurrent pos: {:?}", map.position);
            //     println!("\tdeath pos: {:?}", map.death);
            //     println!("\thome: {:?}", map.home);
            //     if let Some(mini) = &map.mini_map {
            //         println!("\tmini map size: {}", mini.len());
            //     }
            // }
        } else {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.label("I didn't understand this file? The profile was an unexpected version");
            });
        }


        None
    }
}