use std::ops::{Deref, DerefMut};
use godot::classes::{ISprite2D, Sprite2D};
use godot::global::{randi_range};
use godot::prelude::*;
use my_macros::entity_ready;
use my_macros_decl::{base_move, debug_none};
use crate::entity::{IEntity, TargetAction};
use crate::stats::Stats;
use crate::world::World;

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Mob{
    base: Base<Sprite2D>,
    world: OnReady<Gd<World>>,
    #[export]
    stats: OnEditor<Gd<Stats>>,
    target_action: TargetAction,
    attack_target: Option<Gd<Node2D>>

}

#[godot_api]
impl ISprite2D for Mob{
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            stats: OnEditor::default(),
            target_action: TargetAction::NONE,
            world: OnReady::manual(),
            attack_target: None,
        }
    }

    #[entity_ready]
    fn ready(&mut self) {}

}

#[godot_dyn]
impl IEntity for Mob{
    fn act(&mut self) {
        match self.target_action {
            TargetAction::NONE => {
                //Decide on other behaviors

                let huntables = self.base().get_tree().unwrap().get_nodes_in_group("Huntables");
                for huntable in huntables.iter_shared() {
                    let huntable: Gd<Node2D> = huntable.cast();

                    let world = self.world.bind();
                    let dist = (world.local_to_map(huntable.get_position()) - world.local_to_map(self.to_gd().get_position())).length();

                    if dist <= 5. {
                        self.attack_target = Option::from(huntable.clone());
                        self.target_action = TargetAction::ATTACK;
                    }
                }

                if self.target_action == TargetAction::NONE {
                    debug_none!(self);
                }
                self.act();
            }
            TargetAction::IDLE => {
                self.target_action = TargetAction::NONE;
            }
            TargetAction::MOVE(target_pos) => {
                base_move!(self, target_pos);
            }
            TargetAction::ATTACK => {
                if self.attack_target.is_none() {
                    self.target_action = TargetAction::NONE;
                    return;
                }

                let world = self.world.bind();
                let pos = world.local_to_map(self.attack_target.as_ref().unwrap().get_position());
                let dist = (pos - world.local_to_map(self.base().get_position())).length();
                drop(world);

                if dist > 1. {
                    base_move!(self, pos, 1);
                } else {
                    godot_print!("Mob is close enough to attack.");
                }
            }
        }
    }
}
