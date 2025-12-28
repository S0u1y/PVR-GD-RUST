use godot::classes::{ITextureButton, ItemList, TextureButton};
use godot::obj::{Base, Gd};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=TextureButton)]
struct InventorySlot {
    base: Base<TextureButton>,
    //TODO: change this into OnReady
    item_options: Option<Gd<ItemList>>,
}

#[godot_api]
impl ITextureButton for InventorySlot {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            item_options: None,
        }
    }

    fn ready(&mut self) {
        self.item_options = Some(self.base().get_node_as::<ItemList>("ItemOptions"));
    }
}

#[godot_api]
impl InventorySlot {
    #[func]
    fn handle_input(&mut self) {
        if self.item_options.is_none() {
            return;
        }
        let mut options = self.item_options.as_ref().unwrap().clone();
        if !options.is_visible() {
            options.set_visible(true);
        }

        let pos = self.base_mut().get_global_mouse_position();
        options.set_global_position(pos);
    }

    #[func]
    fn handle_mouse_exit(&self) {
        if let Some(options) = &self.item_options {
            let mut options = options.clone();

            if options.is_visible() {
                options.set_visible(false);
            }
        }
    }
}