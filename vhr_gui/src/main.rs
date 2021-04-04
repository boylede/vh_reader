use druid::text::format::ParseFormatter;
use druid::widget::{
    Align, Axis, Button, Container, Flex, Label, LineBreaking, List, Radio, Scroll, Slider, Tabs, Checkbox,
    TabsEdge, TabsTransition, TextBox,
};
use druid::{AppLauncher, Widget, WidgetExt, WindowDesc};

use std::io::Write;
use vhr_chardata::{Gender, Inventory, Item, LoadedCharacter, Skill, SelectedItemLens, inventory::{ItemEquippedLens}};
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
            // todo: when validating names, change to title case.
            // allow only alphabetic and spaces
            .with_child(
                Label::new(|data: &LoadedCharacter, _env: &_| {
                    if data.name.len() > 15 {
                        format!("name too long by {} characters", data.name.len() - 15)
                    } else if data.name.len() < 3 {
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

fn build_warning_tab() -> impl Widget<LoadedCharacter> {
    Align::centered(
        Flex::column()
            .with_default_spacer()
            .with_child(
                Label::new("Please be careful to backup your files before using this utility. Choose from the tabs to edit your character.")
                .with_line_break_mode(LineBreaking::WordWrap)
                .fix_width(600.0))
            .with_flex_spacer(1.0),
    )
}

fn build_appearance_tab() -> impl Widget<LoadedCharacter> {
    let gender_picker = Flex::row()
        .with_child(Label::new("Gender"))
        .with_default_spacer()
        .with_child(Radio::new("Male", Gender::Male))
        .with_default_spacer()
        .with_child(Radio::new("Female", Gender::Female))
        .lens(LoadedCharacter::gender)
        .fix_width(200.0);

    let hair_picker = Flex::row()
        .with_child(Label::new("Hair Style"))
        .with_default_spacer()
        .with_child(
            TextBox::new(), // todo: attempt to nudge input towards known-correct values
        )
        .lens(LoadedCharacter::hair_type)
        .fix_width(200.0);
    let beard_picker = Flex::row()
        .with_child(Label::new("Beard Style"))
        .with_default_spacer()
        .with_child(
            TextBox::new(), // todo: attempt to nudge input towards known-correct values
        )
        .lens(LoadedCharacter::beard_type)
        .fix_width(200.0);

    let hair_color = Flex::row()
        .with_child(Label::new("Hair Color"))
        .with_default_spacer()
        .with_child(
            Flex::column()
                .with_child(Slider::new().lens(vhr_chardata::color::ColorLens::Red))
                .with_child(Slider::new().lens(vhr_chardata::color::ColorLens::Green))
                .with_child(Slider::new().lens(vhr_chardata::color::ColorLens::Blue)),
        )
        .lens(LoadedCharacter::hair)
        .fix_width(200.0);

    let skin_color = Flex::row()
        .with_child(Label::new("Skin Color"))
        .with_default_spacer()
        .with_child(
            Flex::column()
                .with_child(Slider::new().lens(vhr_chardata::color::ColorLens::Red))
                .with_child(Slider::new().lens(vhr_chardata::color::ColorLens::Green))
                .with_child(Slider::new().lens(vhr_chardata::color::ColorLens::Blue)),
        )
        .lens(LoadedCharacter::skin)
        .fix_width(200.0);

    Align::left(
        Flex::column()
            .with_default_spacer()
            .with_child(gender_picker)
            .with_default_spacer()
            .with_child(hair_picker)
            .with_default_spacer()
            .with_child(beard_picker)
            .with_default_spacer()
            .with_child(hair_color)
            .with_default_spacer()
            .with_child(skin_color)
            .with_flex_spacer(1.0),
    )
}

fn build_stats_tab() -> impl Widget<LoadedCharacter> {
    let kill_count = Flex::row()
        .with_child(Label::new("Kill Count"))
        .with_default_spacer()
        .with_child(TextBox::new().with_formatter(ParseFormatter::default()))
        .lens(LoadedCharacter::kill_count)
        .fix_width(200.0);
    let death_count = Flex::row()
        .with_child(Label::new("Death Count"))
        .with_default_spacer()
        .with_child(TextBox::new().with_formatter(ParseFormatter::default()))
        .lens(LoadedCharacter::death_count)
        .fix_width(200.0);
    let craft_count = Flex::row()
        .with_child(Label::new("Crafts Count"))
        .with_default_spacer()
        .with_child(TextBox::new().with_formatter(ParseFormatter::default()))
        .lens(LoadedCharacter::crafting_count)
        .fix_width(200.0);
    let build_count = Flex::row()
        .with_child(Label::new("Builds Count"))
        .with_default_spacer()
        .with_child(TextBox::new().with_formatter(ParseFormatter::default()))
        .lens(LoadedCharacter::building_count)
        .fix_width(200.0);

    Align::left(
        Flex::row()
            .with_child(
                Flex::column()
                    .with_default_spacer()
                    .with_child(kill_count)
                    .with_default_spacer()
                    .with_child(death_count)
                    .with_default_spacer()
                    .with_child(craft_count)
                    .with_default_spacer()
                    .with_child(build_count)
                    .with_flex_spacer(1.0),
            )
            .with_child(
                Flex::column()
                    .with_child(Align::left(Button::new("Add Skill").on_click(
                        |_, data: &mut LoadedCharacter, _| data.skills.push_back(Skill::NONE),
                    )))
                    .with_child(
                        Scroll::new(List::new(|| {
                            Flex::row()
                                .with_child(Label::new(|skill: &Skill, _env: &_| {
                                    format!("Skill {:?}", skill.id)
                                }))
                                .with_default_spacer()
                                .with_child(
                                    TextBox::new()
                                        .with_formatter(ParseFormatter::new())
                                        .lens(Skill::id)
                                        .padding(2.0),
                                )
                                .with_default_spacer()
                                .with_child(
                                    TextBox::new()
                                        .with_formatter(ParseFormatter::new())
                                        .lens(Skill::level),
                                )
                                .with_default_spacer()
                                .with_child(
                                    Slider::new()
                                        .with_range(0.0, 100.0)
                                        .lens(vhr_chardata::skill::Lensf32)
                                        .lens(Skill::progress),
                                )
                                .with_child(
                                    Button::new("Delete").on_click(|_, _data: &mut Skill, _| {
                                        // todo: we can't actually delete this item here.
                                        // need to use like below
                                    }), // .on_click(|_ctx, (shared, item): &mut (Arc<Vec<Skill>>, Skill), _env| {
                                        // })
                                )
                        }))
                        .lens(LoadedCharacter::skills),
                        // todo: lens this for shared access to the vec
                    )
                    .with_flex_spacer(1.0),
            ),
    )
}

fn inventory_item_row(row: u32) -> impl Widget<Inventory> {
    List::new(|| {
        Flex::row().with_child(
            Button::new(|item: &Item, _env: &_| format!("{}", item.name))
                .on_click(|_ctx, data: &mut Item, _env| {
                    // todo: pass in shared inventory data to manipulate selected item
                })
                .fix_width(64.0)
                .fix_height(64.0)
                .padding(2.0),
        )
    })
    .horizontal()
    .lens(vhr_chardata::inventory::InventoryLens::Row(row))
}

fn build_inventory_tab() -> impl Widget<LoadedCharacter> {
    Container::new(
        Flex::column()
            .with_child(inventory_item_row(0))
            .with_child(inventory_item_row(1))
            .with_child(inventory_item_row(2))
            .with_child(inventory_item_row(3))
            .with_child(
                Flex::row()
                .with_child(TextBox::new().with_formatter(ParseFormatter::default()).lens(Item::name))
                .with_child(TextBox::new().with_formatter(ParseFormatter::default()).lens(Item::quantity))
                .with_child(TextBox::new().with_formatter(ParseFormatter::default()).lens(Item::durability))
                .with_child(Checkbox::new("Equipped").lens(ItemEquippedLens::new()))
                .with_child(TextBox::new().with_formatter(ParseFormatter::default()).lens(Item::variant))
                .with_child(TextBox::new().with_formatter(ParseFormatter::default()).lens(Item::creator_id))
                .with_child(TextBox::new().with_formatter(ParseFormatter::default()).lens(Item::creator_name))
                .lens(SelectedItemLens::new())
            )
    )
    .lens(LoadedCharacter::inventory)
}

/*
    pub name: String,
    pub quantity: u32,
    pub durability: f32,
    pub column: u32,
    pub row: u32,
    pub equipped: u8,
    pub quality: u32,
    pub variant: u32,
    pub creator_id: u64,
    pub creator_name: String,

*/

fn build_maps_tab() -> impl Widget<LoadedCharacter> {
    Label::new("Maps")
}

fn build_compendium_tab() -> impl Widget<LoadedCharacter> {
    Label::new("Compendium")
}

fn build_backups_tab() -> impl Widget<LoadedCharacter> {
    Label::new("Backups")
}
