use druid::{Data, Lens};
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::sync::Arc;
use std::rc::Rc;

#[derive(Default, Data, Clone, Lens, PartialEq, Debug, Serialize, Deserialize)]
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
    pub fn default_items() -> Vec<Item> {
        let mut v = vec![];

        v.push( Item {
            name: String::from("Bow"),
            quantity: 1,
            durability: 100.0,
            column: 0,
            row: 0,
            equipped: 1,
            quality: 1,
            variant: 0,
            creator_id: 0,
            creator_name: String::new(),
        });
        v.push( Item {
            name: String::from("Armor"),
            quantity: 1,
            durability: 100.0,
            column: 1,
            row: 0,
            equipped: 1,
            quality: 1,
            variant: 0,
            creator_id: 0,
            creator_name: String::new(),
        });
        v.push( Item {
            name: String::from("Axe"),
            quantity: 1,
            durability: 100.0,
            column: 2,
            row: 0,
            equipped: 0,
            quality: 1,
            variant: 0,
            creator_id: 0,
            creator_name: String::new(),
        });
        v.push( Item {
            name: String::from("Food"),
            quantity: 1,
            durability: 100.0,
            column: 0,
            row: 1,
            equipped: 0,
            quality: 1,
            variant: 0,
            creator_id: 0,
            creator_name: String::new(),
        });
        v.push( Item {
            name: String::from("Food2"),
            quantity: 1,
            durability: 100.0,
            column: 1,
            row: 1,
            equipped: 0,
            quality: 1,
            variant: 0,
            creator_id: 0,
            creator_name: String::new(),
        });
        v.push( Item {
            name: String::from("Food3"),
            quantity: 1,
            durability: 100.0,
            column: 2,
            row: 1,
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

#[derive(Default, Clone, Data, Lens, PartialEq, Debug, Serialize, Deserialize)]
pub struct ItemView {
    pub inner: Item,
}

impl ItemView {
    fn new(inner: Item) -> Self {
        ItemView { inner }
    }
}

pub struct InventoryGridLens {
    width: u8,
    height: u8,
}

impl Default for InventoryGridLens {
    fn default() -> Self {
        InventoryGridLens {
            width: 8,
            height: 4,
        }
    }
}

// Lens<appstate's type, widget's type>
// F: fn(& widget's type) -> V
// data: appstate's type:
impl Lens<Arc<Vec<Item>>, Arc<Vec<ItemView>>> for InventoryGridLens {
    fn with<V, F: FnOnce(&Arc<Vec<ItemView>>) -> V>(&self, data: &Arc<Vec<Item>>, f: F) -> V {
        let mut map: HashMap<(u32, u32), Item> = data
            .iter()
            .map(|item| ((item.row, item.column), item.clone()))
            .collect();
        (0..self.width)
            .flat_map(|x| (0..self.height).map(move |y| (x, y)))
            .for_each(|(x, y)| {
                let pos: (u32, u32) = (x as u32, y as u32);
                map.entry(pos).or_insert(Item::default());
            });
        let view = map.drain().map(|(k, v)| ItemView::new(v)).collect();
        f(&Arc::new(view))
    }
    fn with_mut<V, F: FnOnce(&mut Arc<Vec<ItemView>>) -> V>(
        &self,
        mut data: &mut Arc<Vec<Item>>,
        f: F,
    ) -> V {
        let mut map: HashMap<(u32, u32), Item> = data
            .iter()
            .map(|item| ((item.row, item.column), item.clone()))
            .collect();
        (0..self.width)
            .flat_map(|x| (0..self.height).map(move |y| (x, y)))
            .for_each(|(x, y)| {
                let pos: (u32, u32) = (x as u32, y as u32);
                map.entry(pos).or_insert(Item::default());
            });
        let view = map.drain().map(|(k, v)| ItemView::new(v)).collect();
        let mut view_arc = Arc::new(view);
        let value = f(&mut view_arc);

        //todo: is this correct?
        let data = if let Some(d) = Arc::get_mut(&mut data) {
            d
        } else {
            let d = Arc::make_mut(&mut data);
            d
        };
        let view = Arc::try_unwrap(view_arc).unwrap();
        *data = view.into_iter().map(|iv| iv.inner).collect();
        // .inner
        // .drain()
        // .filter(|(k, v)| v.quantity != 0 && v.name.len() != 0)
        // .map(|(k, v)| v)
        // .collect();
        value
    }
}

// impl Lens<Arc<Vec<Item>>, Rc<Vec<ItemView>>> for InventoryGridLens {
//     fn with<V, F: FnOnce(&Rc<Vec<ItemView>>) -> V>(&self, data: &Arc<Vec<Item>>, f: F) -> V {
//         let mut map: HashMap<(u32, u32), Item> = data
//             .iter()
//             .map(|item| ((item.row, item.column), item.clone()))
//             .collect();
//         (0..self.width)
//             .flat_map(|x| (0..self.height).map(move |y| (x, y)))
//             .for_each(|(x, y)| {
//                 let pos: (u32, u32) = (x as u32, y as u32);
//                 map.entry(pos).or_insert(Item::default());
//             });
//         let view = map.drain().map(|(k, v)| ItemView::new(v)).collect();
//         f(&Rc::new(view))
//     }
//     fn with_mut<V, F: FnOnce(&mut Rc<Vec<ItemView>>) -> V>(
//         &self,
//         mut data: &mut Arc<Vec<Item>>,
//         f: F,
//     ) -> V {
//         let mut map: HashMap<(u32, u32), Item> = data
//             .iter()
//             .map(|item| ((item.row, item.column), item.clone()))
//             .collect();
//         (0..self.width)
//             .flat_map(|x| (0..self.height).map(move |y| (x, y)))
//             .for_each(|(x, y)| {
//                 let pos: (u32, u32) = (x as u32, y as u32);
//                 map.entry(pos).or_insert(Item::default());
//             });
//         let view = map.drain().map(|(k, v)| ItemView::new(v)).collect();
//         let mut view_rc = Rc::new(view);
//         let value = f(&mut view_rc);

//         //todo: is this correct?
//         let data = if let Some(d) = Arc::get_mut(&mut data) {
//             d
//         } else {
//             let d = Arc::make_mut(&mut data);
//             d
//         };
//         let view = Rc::try_unwrap(view_rc).unwrap();
//         *data = view.into_iter().map(|iv| iv.inner).collect();
//         // .inner
//         // .drain()
//         // .filter(|(k, v)| v.quantity != 0 && v.name.len() != 0)
//         // .map(|(k, v)| v)
//         // .collect();
//         value
//     }
// }


// impl Lens<Arc<Vec<Item>>, Vec<ItemView>> for InventoryGridLens {
//     fn with<V, F: FnOnce(&Vec<ItemView>) -> V>(&self, data: &Arc<Vec<Item>>, f: F) -> V {
//         let mut map: HashMap<(u32, u32), Item> = data
//             .iter()
//             .map(|item| ((item.row, item.column), item.clone()))
//             .collect();
//         (0..self.width)
//             .flat_map(|x| (0..self.height).map(move |y| (x, y)))
//             .for_each(|(x, y)| {
//                 let pos: (u32, u32) = (x as u32, y as u32);
//                 map.entry(pos).or_insert(Item::default());
//             });
//         let view = map.drain().map(|(k, v)| ItemView::new(v)).collect();
//         f(&view)
//     }
//     fn with_mut<V, F: FnOnce(&mut Vec<ItemView>) -> V>(
//         &self,
//         mut data: &mut Arc<Vec<Item>>,
//         f: F,
//     ) -> V {
//         let mut map: HashMap<(u32, u32), Item> = data
//             .iter()
//             .map(|item| ((item.row, item.column), item.clone()))
//             .collect();
//         (0..self.width)
//             .flat_map(|x| (0..self.height).map(move |y| (x, y)))
//             .for_each(|(x, y)| {
//                 let pos: (u32, u32) = (x as u32, y as u32);
//                 map.entry(pos).or_insert(Item::default());
//             });
//         let mut view = map.drain().map(|(k, v)| ItemView::new(v)).collect();
//         let value = f(&mut view);

//         //todo: is this correct?
//         let data = if let Some(d) = Arc::get_mut(&mut data) {
//             d
//         } else {
//             let d = Arc::make_mut(&mut data);
//             d
//         };

//         *data = view.into_iter().map(|iv| iv.inner).collect();
//         // .inner
//         // .drain()
//         // .filter(|(k, v)| v.quantity != 0 && v.name.len() != 0)
//         // .map(|(k, v)| v)
//         // .collect();
//         value
//     }
// }