use std::{f64::consts::TAU, sync::Arc};

use async_trait::async_trait;
use log::warn;
use pumpkin_data::{data_component_impl::EquipmentSlot, entity::EntityType};
use pumpkin_inventory::{
    player::player_inventory::PlayerInventory, screen_handler::InventoryPlayer,
};
use pumpkin_nbt::{compound::NbtCompound, tag::NbtTag};
use pumpkin_protocol::java::client::play::{
    CSetContainerContent, CSetContainerProperty, CSetContainerSlot, CSetCursorItem,
    CSetPlayerInventory, CSetSelectedSlot,
};
use pumpkin_util::{GameMode, math::vector3::Vector3};
use pumpkin_world::{inventory::Inventory, item::ItemStack};
use uuid::Uuid;

use crate::entity::{Entity, NBTStorage, NBTStorageInit, item::ItemEntity, player::Player};

impl Player {
    pub fn inventory(&self) -> &Arc<PlayerInventory> {
        &self.inventory
    }

    pub async fn drop_item(&self, item_stack: ItemStack) {
        let item_pos = self.living_entity.entity.pos.load()
            + Vector3::new(0.0, f64::from(EntityType::PLAYER.eye_height) - 0.3, 0.0);
        let entity = Entity::new(
            Uuid::new_v4(),
            self.world().clone(),
            item_pos,
            &EntityType::ITEM,
            false,
        );

        let pitch = f64::from(self.living_entity.entity.pitch.load()).to_radians();
        let yaw = f64::from(self.living_entity.entity.yaw.load()).to_radians();
        let pitch_sin = pitch.sin();
        let pitch_cos = pitch.cos();
        let yaw_sin = yaw.sin();
        let yaw_cos = yaw.cos();
        let horizontal_offset = rand::random::<f64>() * TAU;
        let l = 0.02 * rand::random::<f64>();

        let velocity = Vector3::new(
            -yaw_sin * pitch_cos * 0.3 + horizontal_offset.cos() * l,
            -pitch_sin * 0.3 + 0.1 + (rand::random::<f64>() - rand::random::<f64>()) * 0.1,
            yaw_cos * pitch_cos * 0.3 + horizontal_offset.sin() * l,
        );

        // TODO: Merge stacks together
        let item_entity =
            Arc::new(ItemEntity::new_with_velocity(entity, item_stack, velocity, 40).await);
        self.world().spawn_entity(item_entity).await;
    }

    pub async fn drop_held_item(&self, drop_stack: bool) {
        // should be locked first otherwise cause deadlock in tick() (this thread lock stack, that thread lock screen_handler)

        let binding = self.inventory.held_item();
        let mut item_stack = binding.lock().await;

        if !item_stack.is_empty() {
            let drop_amount = if drop_stack { item_stack.item_count } else { 1 };
            self.drop_item(item_stack.copy_with_count(drop_amount))
                .await;
            item_stack.decrement(drop_amount);
            let selected_slot = self.inventory.get_selected_slot();
            let inv: Arc<dyn Inventory> = self.inventory.clone();
            let screen_binding = self.current_screen_handler.lock().await;
            let mut screen_handler = screen_binding.lock().await;
            let slot_index = screen_handler
                .get_slot_index(&inv, selected_slot as usize)
                .await;

            if let Some(slot_index) = slot_index {
                screen_handler.set_received_stack(slot_index, item_stack.clone());
            }
        }
    }

    pub async fn swap_item(&self) {
        let (main_hand_item, off_hand_item) = self.inventory.swap_item().await;
        let equipment = &[
            (EquipmentSlot::MAIN_HAND, main_hand_item),
            (EquipmentSlot::OFF_HAND, off_hand_item),
        ];
        self.living_entity.send_equipment_changes(equipment).await;
        // todo this.player.stopUsingItem();
    }
}

#[async_trait]
impl InventoryPlayer for Player {
    async fn drop_item(&self, item: ItemStack, _retain_ownership: bool) {
        self.drop_item(item).await;
    }

    fn has_infinite_materials(&self) -> bool {
        self.gamemode.load() == GameMode::Creative
    }

    fn get_inventory(&self) -> Arc<PlayerInventory> {
        self.inventory.clone()
    }

    async fn enqueue_inventory_packet(&self, packet: &CSetContainerContent) {
        self.client.enqueue_packet(packet).await;
    }

    async fn enqueue_slot_packet(&self, packet: &CSetContainerSlot) {
        self.client.enqueue_packet(packet).await;
    }

    async fn enqueue_cursor_packet(&self, packet: &CSetCursorItem) {
        self.client.enqueue_packet(packet).await;
    }

    async fn enqueue_property_packet(&self, packet: &CSetContainerProperty) {
        self.client.enqueue_packet(packet).await;
    }

    async fn enqueue_slot_set_packet(&self, packet: &CSetPlayerInventory) {
        self.client.enqueue_packet(packet).await;
    }

    async fn enqueue_set_held_item_packet(&self, packet: &CSetSelectedSlot) {
        self.client.enqueue_packet(packet).await;
    }
}

impl NBTStorageInit for PlayerInventory {}

#[async_trait]
impl NBTStorage for PlayerInventory {
    async fn write_nbt(&self, nbt: &mut NbtCompound) {
        // Save the selected slot (hotbar)
        nbt.put_int("SelectedItemSlot", i32::from(self.get_selected_slot()));

        // Create inventory list with the correct capacity (inventory size)
        let mut vec: Vec<NbtTag> = Vec::with_capacity(41);
        for (i, item) in self.main_inventory.iter().enumerate() {
            let stack = item.lock().await;
            if !stack.is_empty() {
                let mut item_compound = NbtCompound::new();
                item_compound.put_byte("Slot", i as i8);
                stack.write_item_stack(&mut item_compound);
                drop(stack);
                vec.push(NbtTag::Compound(item_compound));
            }
        }

        let mut equipment_compound = NbtCompound::new();
        for slot in self.equipment_slots.values() {
            let stack_binding = self.entity_equipment.lock().await.get(slot);
            let stack = stack_binding.lock().await;
            if !stack.is_empty() {
                let mut item_compound = NbtCompound::new();
                stack.write_item_stack(&mut item_compound);
                drop(stack);
                match slot {
                    EquipmentSlot::OffHand(_) => {
                        equipment_compound.put_component("offhand", item_compound);
                    }
                    EquipmentSlot::Feet(_) => {
                        equipment_compound.put_component("feet", item_compound);
                    }
                    EquipmentSlot::Legs(_) => {
                        equipment_compound.put_component("legs", item_compound);
                    }
                    EquipmentSlot::Chest(_) => {
                        equipment_compound.put_component("chest", item_compound);
                    }
                    EquipmentSlot::Head(_) => {
                        equipment_compound.put_component("head", item_compound);
                    }
                    _ => {
                        warn!("Invalid equipment slot for a player {slot:?}");
                    }
                }
            }
        }
        nbt.put_component("equipment", equipment_compound);
        nbt.put("Inventory", NbtTag::List(vec));
    }

    async fn read_nbt_non_mut(&self, nbt: &NbtCompound) {
        // Read selected hotbar slot
        self.set_selected_slot(nbt.get_int("SelectedItemSlot").unwrap_or(0) as u8);
        // Process inventory list
        if let Some(inventory_list) = nbt.get_list("Inventory") {
            for tag in inventory_list {
                if let Some(item_compound) = tag.extract_compound()
                    && let Some(slot_byte) = item_compound.get_byte("Slot")
                {
                    let slot = slot_byte as usize;
                    if let Some(item_stack) = ItemStack::read_item_stack(item_compound) {
                        self.set_stack(slot, item_stack).await;
                    }
                }
            }
        }

        if let Some(equipment) = nbt.get_compound("equipment") {
            if let Some(offhand) = equipment.get_compound("offhand")
                && let Some(item_stack) = ItemStack::read_item_stack(offhand)
            {
                self.set_stack(40, item_stack).await;
            }

            if let Some(head) = equipment.get_compound("head")
                && let Some(item_stack) = ItemStack::read_item_stack(head)
            {
                self.set_stack(39, item_stack).await;
            }

            if let Some(chest) = equipment.get_compound("chest")
                && let Some(item_stack) = ItemStack::read_item_stack(chest)
            {
                self.set_stack(38, item_stack).await;
            }

            if let Some(legs) = equipment.get_compound("legs")
                && let Some(item_stack) = ItemStack::read_item_stack(legs)
            {
                self.set_stack(37, item_stack).await;
            }

            if let Some(feet) = equipment.get_compound("feet")
                && let Some(item_stack) = ItemStack::read_item_stack(feet)
            {
                self.set_stack(36, item_stack).await;
            }
        }
    }
}
