use druid::im::Vector;
use druid::lens::{Map, Then};
use druid::text::format::ParseFormatter;
use druid::widget::{
    Align, Axis, Button, CrossAxisAlignment, Flex, Label, LabelText, LineBreaking, List,
    MainAxisAlignment, RadioGroup, Scroll, Slider, Split, TabInfo, Tabs, TabsEdge, TabsPolicy,
    TabsTransition, TextBox, ValueTextBox, ViewSwitcher,
};
use druid::{theme, AppLauncher, Color, Data, Env, Lens, Widget, WidgetExt, WindowDesc};
use instant::Duration;

use std::io::Write;

use vhr_chardata::LoadedCharacter;
use vhr_serde::ser::to_bytes;

fn main() {
    let my_char = LoadedCharacter::default();
    let mut disk_char = my_char.to_disk();
    disk_char.pre_serialize();
    let serialized = to_bytes(&disk_char);
    if let Ok(data) = serialized {
        let filename = "output.fch";
        let mut file = std::fs::File::create(&filename).unwrap();
        match file.write(&data) {
            Ok(num) => println!("wrote {} bytes", num),
            Err(e) => println!("error: {:?}", e),
        }
    } else {
        let e = serialized.unwrap_err();
        println!("error while serializing: {:?}", e);
    }

    let main_window = WindowDesc::new(build_root_widget)
        .title("vhc_reader")
        .window_size((700.0, 600.0));

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(my_char)
        .expect("Failed to launch application.");
}

fn build_root_widget() -> impl Widget<LoadedCharacter> {
    let character_name = Align::left(
        Flex::row()
            .with_child(Label::new("Character Name"))
            .with_child(TextBox::new().lens(LoadedCharacter::name).fix_width(200.0))
            .with_child(
                Label::new(|data: &LoadedCharacter, _env: &_| {
                    if data.name.len() > 16 {
                        format!("name too long by {} characters", data.name.len() - 16)
                    } else if data.name.len() < 1 {
                        "name too short".into()
                    } else {
                        "".into()
                    }
                })
                .fix_width(220.0),
            )
            .with_child(Button::new("Save"))
            .with_child(Button::new("Reset")),
    );
    Flex::column()
        .with_spacer(2.0)
        .with_child(character_name)
        .with_spacer(16.0)
        .with_child(
            Tabs::new()
                .with_axis(Axis::Horizontal)
                .with_edge(TabsEdge::Leading)
                .with_transition(TabsTransition::Instant)
                .with_tab("Warning", build_warning_tab())
                .with_tab("Appearance", build_appearance_tab())
                .with_tab("Stats", build_stats_tab())
                .with_tab("Inventory", build_inventory_tab())
                .with_tab("Maps", build_maps_tab())
                .with_tab("Compendium", build_compendium_tab())
                .with_tab("Backups", build_backups_tab())
                .fix_height(500.0),
        )
        .padding(5.0)
    // .fix_width(700.0),
    // )
}

fn labeled_with_box<T: Data, W: Widget<T> + 'static>(
    text: impl Into<LabelText<T>>,
    w: W,
) -> impl Widget<T> {
    Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(
            Label::new(text).background(theme::PLACEHOLDER_COLOR), // .expand_width(),
        )
        .with_default_spacer()
        .with_child(w)
        .with_default_spacer()
        .border(Color::WHITE, 0.5)
    // .fix_width(200.0)
}

fn build_warning_tab() -> impl Widget<LoadedCharacter> {
    Align::centered(
        Label::new("Please be careful to backup your files before using this utility. Choose from the tabs to edit your character.")
        .with_line_break_mode(LineBreaking::WordWrap)
        .fix_width(600.0)
    )
}

fn build_appearance_tab() -> impl Widget<LoadedCharacter> {
    Label::new("Appearance")
}

fn build_stats_tab() -> impl Widget<LoadedCharacter> {
    Label::new("Stats")
}

fn build_inventory_tab() -> impl Widget<LoadedCharacter> {
    Label::new("Inventory")
}

fn build_maps_tab() -> impl Widget<LoadedCharacter> {
    Label::new("Maps")
}

fn build_compendium_tab() -> impl Widget<LoadedCharacter> {
    Label::new("Compendium")
}

fn build_backups_tab() -> impl Widget<LoadedCharacter> {
    Label::new("Backups")
}
