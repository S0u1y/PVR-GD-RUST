#[allow(unused)]

use godot::prelude::*;
use godot::classes::{Sprite2D, ISprite2D, Node2D, INode2D, InputEvent, InputEventMouse, InputEventMouseButton, CanvasItem, TextureRect, ITextureRect, ItemList, TextureButton, ITextureButton};
use godot::global::MouseButton;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}


#[derive(GodotClass)]
#[class(base=Node2D)]
struct World{
    base: Base<Node2D>

}

#[godot_api]
impl INode2D for World {
    fn init(base: Base<Node2D>) -> Self{
        godot_print!("Hello, world!"); // Prints to the Godot console

        Self {
            base,
        }
    }
}

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    base: Base<Sprite2D>
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {

        godot_print!("Player Init2");

        Self {
            base,
        }
    }
    fn input(&mut self, event: Gd<InputEvent>) {
        if let Ok(e) = event.try_cast::<InputEventMouseButton>(){
            if e.get_button_index() == MouseButton::LEFT && e.is_pressed(){
                godot_print!("{}", self.base().get_global_mouse_position());
            }
        }
    }
}

#[derive(GodotClass)]
#[class(base=TextureButton)]
struct InventorySlot{
    base: Base<TextureButton>,
    item_options: Option<Gd<ItemList>>
}

#[godot_api]
impl ITextureButton for InventorySlot {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            item_options: None
        }
    }

    fn ready(&mut self) {
        self.item_options = Some(self.base().get_node_as::<ItemList>("ItemOptions"));
    }

}

#[godot_api]
impl InventorySlot {

    #[func]
    fn handle_input(&mut self){
        if self.item_options == None {
            return
        }
        let mut options = self.item_options.as_ref().unwrap().clone();
        if !options.is_visible(){
            options.set_visible(true);
        }

        let pos = self.base_mut().get_global_mouse_position();
        options.set_global_position(pos);
    }

    #[func]
    fn handle_mouse_exit(&self){
        if let Some(options) = &self.item_options {
            let mut options = options.clone();

            if options.is_visible(){
                options.set_visible(false);
            }

        }
    }

}


