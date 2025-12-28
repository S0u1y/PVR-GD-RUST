use crate::entity::{IEntity, TargetAction};
use crate::world::World;
use godot::classes::{ISprite2D, InputEvent, InputEventMouseButton, Sprite2D, TileMapLayer};
use godot::global::{MouseButton};
use godot::obj::{Base, Gd};
use godot::prelude::*;
use my_macros::entity_ready;
use my_macros_decl::base_move;
use crate::stats::Stats;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
pub struct Player {
    base: Base<Sprite2D>,
    #[export]
    stats: OnEditor<Gd<Stats>>,
    pub(crate) target_action: TargetAction,
    attack_target: Option<Gd<Node2D>>,
    world: OnReady<Gd<World>>,
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        Self {
            base,
            stats: OnEditor::default(),
            target_action: TargetAction::NONE,
            attack_target: None,
            world: OnReady::manual(),
        }
    }

    #[entity_ready]
    fn ready(&mut self) {
        self.base_mut().add_to_group("Huntables");
    }

    //doesn't happen if another node above stops the event propagation.
    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
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
                base_move!(self, target_pos);
            }
            TargetAction::ATTACK =>{
                if self.attack_target.is_none() {
                    self.target_action = TargetAction::NONE;
                    return;
                }
                
                let pos = self.attack_target.clone().unwrap().get_position().cast_int();
                let world = self.world.bind();
                let dist = (pos - world.local_to_map(self.to_gd().get_position())).length();
                drop(world);

                if dist > 1. {
                    base_move!(self, pos, 1);
                }
            }
        }
    }
}

