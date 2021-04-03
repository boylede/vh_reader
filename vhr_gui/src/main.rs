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
}
