use serde::{Deserialize, Serialize};

/// an inventory item, found in chests and the player's inventory
#[derive(Default, Clone, PartialEq, Debug, Serialize, Deserialize, PartialOrd)]
pub struct Item {
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
}

impl Item {
    /// an item with all zero values, a const function for use in initializing a blank item.
    pub const fn null_item() -> Item {
        Item {
            name: String::new(),
            quantity: 0,
            durability: 0.0,
            column: 0,
            row: 0,
            equipped: 0,
            quality: 0,
            variant: 0,
            creator_id: 0,
            creator_name: String::new(),
        }
    }
    pub fn default_items() -> Vec<Item> {
        let mut v = vec![];

        v.push(Item {
            name: String::from("MaceIron"),
            quantity: 1,
            durability: 221.0,
            column: 0,
            row: 0,
            equipped: 1,
            quality: 2,
            variant: 0,
            creator_id: 0,
            creator_name: String::new(),
        });
        v.push(Item {
            name: String::from("ShieldBronzeBuckler"),
            quantity: 1,
            durability: 200.0,
            column: 1,
            row: 0,
            equipped: 0,
            quality: 1,
            variant: 0,
            creator_id: 0,
            creator_name: String::new(),
        });
        v.push(Item {
            name: String::from("Wishbone"),
            quantity: 1,
            durability: 100.0,
            column: 7,
            row: 1,
            equipped: 1,
            quality: 1,
            variant: 0,
            creator_id: 0,
            creator_name: String::new(),
        });
        v.push(Item {
            name: String::from("PickaxeIron"),
            quantity: 1,
            durability: 225.0,
            column: 2,
            row: 0,
            equipped: 0,
            quality: 3,
            variant: 0,
            creator_id: 0,
            creator_name: String::new(),
        });
        v.push(Item {
            name: String::from("Sausages"),
            quantity: 17,
            durability: 100.0,
            column: 0,
            row: 3,
            equipped: 0,
            quality: 1,
            variant: 0,
            creator_id: 0,
            creator_name: String::new(),
        });
        v.push(Item {
            name: String::from("TurnupStew"),
            quantity: 8,
            durability: 100.0,
            column: 0,
            row: 2,
            equipped: 0,
            quality: 1,
            variant: 0,
            creator_id: 0,
            creator_name: String::new(),
        });
        v
    }
}
