use crate::entity::IEntity;
use crate::stats::Stats;
use crate::world::World;
use godot::classes::{ISprite2D, InputEvent, InputEventMouseButton, Sprite2D, TileMapLayer};
use godot::global::{MouseButton, godot_print};
use godot::obj::{Base, Gd};
use godot::prelude::*;
use std::cmp::PartialEq;

#[derive(Eq, PartialEq)]
enum TargetAction {
    NONE,
    MOVE(Vector2i),
    IDLE,
}

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct Player {
    base: Base<Sprite2D>,
    stats: Stats,
    target_action: TargetAction,
    world: OnReady<Gd<World>>,
}

#[godot_dyn]
impl IEntity for Player {
    fn act(&mut self) {
        match self.target_action {
            TargetAction::NONE => {}
            TargetAction::IDLE => {
                self.target_action = TargetAction::NONE;
            }
            TargetAction::MOVE(target_pos) => {
                //get current pos and figure out the path to the desired location and move one step

                let mut world = self.base().get_parent().unwrap().cast::<World>();
                let mut self_base_ref = self.base_mut().clone();
                let mut self_ref = self.to_gd(); //Potentially not a sound approach if bind_mut is called on it (?)

                world.run_deferred(move |s| {
                    let curr_pos = s.local_to_map(self_base_ref.get_position());
                    if let Some(next_cell) = s.get_next_path_coord(curr_pos, target_pos){
                        self_base_ref.set_position(s.map_to_local(next_cell));
                            if s.local_to_map(self_base_ref.get_position()) == target_pos{
                                self_ref.bind_mut().target_action = TargetAction::NONE;
                            }
                    }
                });
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
            stats: Stats {
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
            world: OnReady::manual(),
        }
    }

    fn ready(&mut self) {
        let s = self.to_gd().clone();
        self.world.init(s.get_owner().unwrap().cast());
        self.world.bind_mut().register_entity(s);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if let Ok(e) = event.try_cast::<InputEventMouseButton>() {
            if e.get_button_index() == MouseButton::LEFT && e.is_pressed() {
                if let Some(parent) = self.base().get_parent() {
                    let ground = parent.get_node_as::<TileMapLayer>("Ground");
                    let tile_coord = ground.local_to_map(ground.get_local_mouse_position());

                    self.target_action = TargetAction::MOVE(tile_coord);
                }
            }
        }
    }
}
