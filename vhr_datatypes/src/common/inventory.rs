use super::known_size::KnownSize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// the inventory when loaded into memory for display in the gui
#[derive(Clone, Debug)]
pub struct Inventory {
    pub selected_item: Option<(u32, u32)>,
    pub items: Vec<Item>,
}

impl Inventory {
    pub fn default_items() -> Self {
        Inventory {
            selected_item: None,
            items: Item::default_items().into(),
        }
    }
}

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
    pub fn empty(row: u32, column: u32) -> Self {
        let mut item: Item = Default::default();
        item.row = row;
        item.column = column;
        item
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
    pub fn pre_serialize(&mut self) -> usize {
        let mut size = 33; //hand-counted size of struct
        size += self.name.len() + 1;
        size += self.creator_name.len() + 1;
        size
    }
}

impl KnownSize for Item {
    fn count_bytes(&self) -> usize {
        let mut size = 33;
        size += self.name.len() + 1;
        size += self.creator_name.len() + 1;
        size
    }
}

/*
pub struct ItemEquippedLens {}

impl ItemEquippedLens {
    pub fn new() -> Self {
        Self {}
    }
}

impl Lens<Item, bool> for ItemEquippedLens {
    fn with<V, F: FnOnce(&bool) -> V>(&self, data: &Item, f: F) -> V {
        if data.equipped == 0 {
            f(&false)
        } else {
            f(&true)
        }
    }
    fn with_mut<V, F: FnOnce(&mut bool) -> V>(&self, data: &mut Item, f: F) -> V {
        let mut flag = if data.equipped == 0 { false } else { true };
        let v = f(&mut flag);
        if flag {
            data.equipped = 1;
        } else {
            data.equipped = 0;
        }
        v
    }
}

pub struct SelectedItemLens {}

impl SelectedItemLens {
    pub fn new() -> Self {
        Self {}
    }
}

impl Lens<Inventory, Item> for SelectedItemLens {
    fn with<V, F: FnOnce(&Item) -> V>(&self, data: &Inventory, f: F) -> V {
        if let Some((r, c)) = data.selected_item {
            let selected = data
                .items
                .iter()
                .find(|item| item.row == r && item.column == c);
            if let Some(item) = selected {
                f(&item)
            } else {
                // todo: is this reachable, and what to do if it is
                println!("Error - selected item doesn't exist?");
                f(&Default::default())
            }
        } else {
            //todo: should show greyed out ui, use Either widget
            f(&Default::default())
        }
    }
    fn with_mut<V, F: FnOnce(&mut Item) -> V>(&self, data: &mut Inventory, f: F) -> V {
        if let Some((r, c)) = data.selected_item {
            let selected = data
                .items
                .iter_mut()
                .find(|item| item.row == r && item.column == c);
            if let Some(mut item) = selected {
                f(&mut item)
            } else {
                // todo: is this reachable, and what to do if it is
                println!("Error - selected item doesn't exist?");
                f(&mut Default::default())
            }
        } else {
            //todo: should show greyed out ui, use Either widget
            f(&mut Default::default())
        }
    }
}

pub enum InventoryLens {
    Row(u32),
    Column(u32),
}

impl Lens<Inventory, Vector<Item>> for InventoryLens {
    fn with<V, F: FnOnce(&Vector<Item>) -> V>(&self, data: &Inventory, f: F) -> V {
        // println!("looking at items");
        let items = match self {
            InventoryLens::Row(r) => {
                let mut items: Vector<Item> = data
                    .items
                    .iter()
                    .filter(|item| item.row == *r)
                    .cloned()
                    .collect();
                // println!("number of items: {}", items.len());
                for col in 0..8 {
                    if !items.iter().any(|item| item.column == col) {
                        // println!("no items in {},{}", r, col);
                        items.push_back(Item::empty(*r, col))
                    }
                }
                items.sort_by(|a, b| a.column.cmp(&b.column));
                items
            }
            InventoryLens::Column(c) => unimplemented!(),
        };
        f(&items)
    }
    fn with_mut<V, F: FnOnce(&mut Vector<Item>) -> V>(&self, data: &mut Inventory, f: F) -> V {
        let mut items = match self {
            InventoryLens::Row(r) => {
                let mut items: Vector<Item> = data
                    .items
                    .iter()
                    .filter(|item| item.row == *r)
                    .cloned()
                    .collect();
                for col in 0..8 {
                    if !items.iter().any(|item| item.column == col) {
                        items.push_back(Item::empty(*r, col))
                    }
                }
                items.sort_by(|a, b| b.column.cmp(&a.column));
                items
            }
            InventoryLens::Column(c) => unimplemented!(),
        };
        let v = f(&mut items);
        // println!("items.len = {}", items.len());
        // todo: look at all items and check if something exists in that row,col in inventory, if so replace that item with this one
        // items.retain(|item|
        //     item.name.len() > 0 && item.quantity > 0
        // );
        // data.items = items; // todo, write this back
        v
    }
}
 */
