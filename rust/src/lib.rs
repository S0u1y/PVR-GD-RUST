#[allow(unused)]

use godot::prelude::*;
use godot::classes::{Sprite2D, ISprite2D, Node2D, INode2D};


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

        godot_print!("Player Init");

        Self {
            base,
        }
    }
}

