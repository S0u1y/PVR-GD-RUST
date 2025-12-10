use crate::entity::{Entity, IEntity, TargetAction};
use crate::stats::Stats;
use crate::world::World;
use godot::classes::{ISprite2D, InputEvent, InputEventMouseButton, Sprite2D, TileMapLayer};
use godot::global::{MouseButton, godot_print};
use godot::obj::{Base, Gd};
use godot::prelude::*;
use my_macros::entity_ready;
use my_macros_decl::base_move;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct Player {
    base: Base<Sprite2D>,
    entity: Entity,
    world: OnReady<Gd<World>>,
}

#[godot_dyn]
impl IEntity for Player {
    fn act(&mut self) {
        match self.entity.target_action {
            TargetAction::NONE => {}
            TargetAction::IDLE => {
                self.entity.target_action = TargetAction::NONE;
            }
            TargetAction::MOVE(target_pos) => {
                //get current pos and figure out the path to the desired location and move one step

                base_move!(self, target_pos);

                // let mut self_base_ref = self.base_mut().clone();
                // let mut world_bind = self.world.bind_mut();
                //
                // let curr_pos = world_bind.local_to_map(self_base_ref.get_position());
                // if let Some(next_cell) = world_bind.get_next_path_coord(curr_pos, target_pos) {
                //     self_base_ref.set_position(world_bind.map_to_local(next_cell));
                //     if world_bind.local_to_map(self_base_ref.get_position()) == target_pos {
                //         self.entity.target_action = TargetAction::NONE;
                //     }
                // }
            }
        }
    }
}



#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Player Init");

        Self {
            base,
            entity: Entity::new(),
            world: OnReady::manual(),
        }
    }

    #[entity_ready]
    fn ready(&mut self) {
        godot_print!("Player ready");
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if let Ok(e) = event.try_cast::<InputEventMouseButton>() {
            if e.get_button_index() == MouseButton::LEFT && e.is_pressed() {
                if let Some(parent) = self.base().get_parent() {
                    let ground = parent.get_node_as::<TileMapLayer>("Ground");
                    let tile_coord = ground.local_to_map(ground.get_local_mouse_position());

                    self.entity.target_action = TargetAction::MOVE(tile_coord);
                }
            }
        }
    }
}
