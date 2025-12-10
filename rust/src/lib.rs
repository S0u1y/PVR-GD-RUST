#[macro_use]
extern crate public;
mod entity;
mod player;
mod stats;
mod world;
mod inventory_slot;

use godot::classes::{ISprite2D, Sprite2D};
use godot::prelude::*;
use my_macros::entity_ready;
use crate::entity::IEntity;
use crate::world::World;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Mob{
    base: Base<Sprite2D>,

    world: OnReady<Gd<World>>
}

#[godot_api]
impl ISprite2D for Mob{
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            world: OnReady::manual()
        }
    }

    #[entity_ready]
    fn ready(&mut self) {

    }

}

#[godot_dyn]
impl IEntity for Mob{
    fn act(&mut self) {

    }
}
