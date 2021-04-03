use druid::{AppLauncher, Widget, WindowDesc, widget::{
    Align, Flex, Label,
}};

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
        .window_size((500.0, 800.0));

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(my_char)
        .expect("Failed to launch application.");

}

fn build_root_widget() -> impl Widget<LoadedCharacter> {
    Align::centered(
        Flex::column()
            .with_child(Label::new("Root")
    ))
}