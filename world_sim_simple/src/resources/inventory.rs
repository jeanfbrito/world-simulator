use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use super::{Item, ItemStack, ItemType, ResourceType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventorySlot {
    pub item_stack: Option<ItemStack>,
}

impl InventorySlot {
    pub fn empty() -> Self {
        Self { item_stack: None }
    }
    
    pub fn with_item(item: Item, count: u32) -> Self {
        Self {
            item_stack: Some(ItemStack::new(item, count)),
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.item_stack.is_none() || 
        self.item_stack.as_ref().map_or(false, |s| s.is_empty())
    }
    
    pub fn can_accept(&self, item: &Item) -> bool {
        match &self.item_stack {
            None => true,
            Some(stack) => {
                stack.item.item_type == item.item_type && 
                !stack.is_full()
            }
        }
    }
    
    pub fn add_item(&mut self, item: Item, count: u32) -> u32 {
        match &mut self.item_stack {
            None => {
                self.item_stack = Some(ItemStack::new(item, count));
                0 // No overflow
            }
            Some(stack) => {
                if stack.item.item_type == item.item_type {
                    stack.add(count)
                } else {
                    count // Can't add different item type
                }
            }
        }
    }
    
    pub fn remove_item(&mut self, count: u32) -> Option<(Item, u32)> {
        match &mut self.item_stack {
            None => None,
            Some(stack) => {
                let removed = stack.remove(count);
                let item = stack.item.clone();
                
                if stack.is_empty() {
                    self.item_stack = None;
                }
                
                Some((item, removed))
            }
        }
    }
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub slots: Vec<InventorySlot>,
    pub max_weight: f32,
}

impl Inventory {
    pub fn new(size: usize, max_weight: f32) -> Self {
        Self {
            slots: vec![InventorySlot::empty(); size],
            max_weight,
        }
    }
    
    pub fn add_item(&mut self, item: Item, mut count: u32) -> u32 {
        // First try to add to existing stacks
        for slot in &mut self.slots {
            if count == 0 {
                break;
            }
            
            if slot.can_accept(&item) {
                count = slot.add_item(item.clone(), count);
            }
        }
        
        // Then try to add to empty slots
        while count > 0 {
            if let Some(empty_slot) = self.slots.iter_mut().find(|s| s.is_empty()) {
                count = empty_slot.add_item(item.clone(), count);
            } else {
                break; // No more empty slots
            }
        }
        
        count // Return overflow
    }
    
    pub fn remove_item(&mut self, item_type: ItemType, mut count: u32) -> u32 {
        let mut removed_total = 0;
        
        for slot in &mut self.slots {
            if count == 0 {
                break;
            }
            
            if let Some(stack) = &slot.item_stack {
                if stack.item.item_type == item_type {
                    if let Some((_, removed)) = slot.remove_item(count) {
                        removed_total += removed;
                        count -= removed;
                    }
                }
            }
        }
        
        removed_total
    }
    
    pub fn has_item(&self, item_type: ItemType, count: u32) -> bool {
        let total = self.count_item(item_type);
        total >= count
    }
    
    pub fn count_item(&self, item_type: ItemType) -> u32 {
        self.slots
            .iter()
            .filter_map(|s| s.item_stack.as_ref())
            .filter(|stack| stack.item.item_type == item_type)
            .map(|stack| stack.count)
            .sum()
    }
    
    pub fn total_weight(&self) -> f32 {
        self.slots
            .iter()
            .filter_map(|s| s.item_stack.as_ref())
            .map(|stack| stack.total_weight())
            .sum()
    }
    
    pub fn is_overweight(&self) -> bool {
        self.total_weight() > self.max_weight
    }
    
    pub fn total_value(&self) -> u32 {
        self.slots
            .iter()
            .filter_map(|s| s.item_stack.as_ref())
            .map(|stack| stack.total_value())
            .sum()
    }
    
    pub fn find_slot_with_item(&self, item_type: ItemType) -> Option<usize> {
        self.slots
            .iter()
            .position(|s| {
                s.item_stack
                    .as_ref()
                    .map_or(false, |stack| stack.item.item_type == item_type)
            })
    }
    
    pub fn clear(&mut self) {
        for slot in &mut self.slots {
            slot.item_stack = None;
        }
    }
}

// Helper function to create starter inventory
pub fn create_starter_inventory() -> Inventory {
    let mut inventory = Inventory::new(20, 100.0);
    
    // Add some starter items
    inventory.add_item(Item::new_resource(ResourceType::Wood), 10);
    inventory.add_item(Item::new_resource(ResourceType::Stone), 5);
    inventory.add_item(Item::new_resource(ResourceType::Berries), 20);
    
    inventory
}