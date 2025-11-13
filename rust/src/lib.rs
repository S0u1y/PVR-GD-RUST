use std::any::Any;
#[allow(unused)]

use godot::prelude::*;
use godot::classes::{Sprite2D, ISprite2D, Node2D, INode2D, InputEvent, InputEventMouse, InputEventMouseButton, CanvasItem};
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
    fn init(base: Base<Self::Base>) -> Self {

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

