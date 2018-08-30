use std::collections::HashMap;
use specs_derive::Component;
use game_engine::prelude::*;
use crate::model::item::Item;

#[derive(Component, Clone, Default, Debug)]
pub struct Inventory(HashMap<Item, u32>);

impl Inventory {
    pub fn count(&self, item: &Item) -> u32 {
        *self.0.get(item).unwrap_or(&0)
    }

    pub fn has(&self, item: &Item, count: u32) -> bool {
        self.count(item) >= count
    }

    pub fn add(&mut self, item: &Item, count: u32) {
        self.0.entry(*item)
            .and_modify(|prev| { *prev += count; })
            .or_insert(count);
    }

    pub fn take(&mut self, item: &Item, count: u32) -> u32 {
        let prev = self.count(item);
        self.0.entry(*item).and_modify(|prev| { *prev -= count; });
        u32::min(prev, count)
    }

    pub fn take_contents(&mut self) -> Inventory {
        let mut contents = HashMap::default();
        std::mem::swap(&mut contents, &mut self.0);
        Inventory(contents)
    }

    pub fn merge(&mut self, other: Inventory) {
        for (item, count) in other.0.into_iter() {
            self.add(&item, count);
        }
    }
}
