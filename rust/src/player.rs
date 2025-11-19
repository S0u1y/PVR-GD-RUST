use std::cmp::PartialEq;
use godot::classes::{ISprite2D, InputEvent, InputEventMouseButton, Sprite2D, TileMapLayer};
use godot::global::{godot_print, MouseButton};
use godot::obj::{Base, Gd};
use godot::prelude::*;
pub(crate) use crate::entity::IEntity;
use crate::stats::Stats;
use crate::world::World;

#[derive(Eq, PartialEq)]
enum TargetAction{
    NONE,
    MOVE(i32, i32),
    IDLE,

}

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct Player{
    base: Base<Sprite2D>,
    stats: Stats,
    target_action: TargetAction,
}

#[godot_dyn]
impl IEntity for Player{
    fn act(&self){
        match self.target_action{
            TargetAction::NONE=>{},
            TargetAction::IDLE=>{},
            TargetAction::MOVE(x,y)=>{
                //get current pos and figure out the path to the desired location and move one step
                godot_print!("Player wants to move to {x}, {y}")
            }
        }

    }
}

#[godot_api]
impl ISprite2D for Player{
    fn init(base: Base<Sprite2D>) -> Self {

        godot_print!("Player Init2");

        Self {
            base,
            stats: Stats{
                thirst: 100,
                hunger: 100,
                energy: 100,

                speed: 1,
                strength: 1,
                dexterity: 1,
                endurance: 1,

                ..Default::default()
            },
            target_action: TargetAction::NONE,
        }
    }

    fn ready(&mut self) {
        if let Some(parent) = self.base().get_parent(){
            let mut parent = parent.cast::<World>();
            parent.bind_mut().register_entity(self.to_gd());
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if let Ok(e) = event.try_cast::<InputEventMouseButton>(){
            if e.get_button_index() == MouseButton::LEFT && e.is_pressed(){

                if let Some(parent) = self.base().get_parent(){
                    let roads = parent.get_node_as::<TileMapLayer>("Roads");
                    let tile_coord = roads.local_to_map(roads.get_local_mouse_position());
                    let desired_move_position = roads.to_global(roads.map_to_local(tile_coord));

                    // self.base_mut().set_global_position(desired_move_position);

                    self.target_action = TargetAction::MOVE(
                        desired_move_position.x as i32,
                        desired_move_position.y as i32,
                    );

                }
            }
        }
    }
}
