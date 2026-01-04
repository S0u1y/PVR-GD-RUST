use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use godot::builtin::Vector2i;
use godot::classes::{InputEvent, InputEventMouseButton, TileMapLayer};
use godot::global::MouseButton;
use godot::obj::WithBaseField;
use godot::prelude::{godot_api, godot_dyn, Base, DynGd, FromGodot, Gd, GodotClass, INode, Node, Node2D, OnReady, Variant};
use crate::entity::TargetAction;
use crate::player::Player;

/// Node based state machine from gd-rust
/// idea was nice, but started devolving into something more of "just pure GDScript"-like
/// so I decided against it and keep using enums

pub trait IStateMachine {
    fn _ready(&mut self){}
    fn _process(&mut self){}
    fn _input(&mut self, event: Gd<InputEvent>){}
    fn _unhandled_input(&mut self, event: Gd<InputEvent>){}

    fn _change_state(&mut self, new_state_name: String, args: &[Variant]);
}

pub trait IState
{
    fn _ready(&mut self){}
    fn _process(&mut self){}
    fn _tick(&mut self){}
    fn _input(&mut self, event: Gd<InputEvent>){}
    fn _unhandled_input(&mut self, event: Gd<InputEvent>){}
    fn _enter(&mut self, args: &[Variant]);
    fn _exit(&mut self);

    fn _set_state_machine(&mut self, state_machine: &Gd<StateMachine>);

}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct StateMachine {
    base: Base<Node>,
    states: HashMap<String, DynGd<Node, dyn IState>>,
    #[export]
    current_state: Option<DynGd<Node, dyn IState>>,
}

#[godot_api]
impl INode for StateMachine {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            states: HashMap::new(),
            current_state: None,
        }
    }

    fn process(&mut self, delta: f64) {
        if let Some(state) = self.current_state.as_mut(){
            state.dyn_bind_mut()._process();
        }
    }

    fn ready(&mut self) {
        for child in self.base().get_children().iter_shared() {
            let mut state: DynGd<Node, dyn IState> = DynGd::from_godot(child.clone());

            // state.dyn_bind_mut()._set_state_machine(&self.to_gd());

            self.states.insert(state.get_name().into(), state);
        }
    }
    fn input(&mut self, event: Gd<InputEvent>) {
        if let Some(state) = self.current_state.as_mut(){
            state.dyn_bind_mut()._input(event);
        }
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        if let Some(state) = self.current_state.as_mut(){
            state.dyn_bind_mut()._unhandled_input(event);
        }
    }
}

#[godot_dyn]
impl IStateMachine for StateMachine {
    fn _change_state(&mut self, new_state_name: String, args: &[Variant]) {
        if !self.current_state.is_none() {
            self.current_state.take().unwrap().dyn_bind_mut()._exit();
        }
        if let Some(state) = self.states.get(&new_state_name) {
            let mut s = state.clone();
            s.dyn_bind_mut()._enter(args);
            self.current_state = Some(s);
        }

    }
}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct PlayerNoneState{
    base: Base<Node>,
    state_machine: OnReady<Gd<StateMachine>>,
    player: OnReady<Gd<Player>>,
}

#[godot_api]
impl INode for PlayerNoneState{
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            state_machine: OnReady::from_base_fn(|b| b.get_parent().unwrap().cast()),
            player: OnReady::manual(),
        }
    }

    fn ready(&mut self) {
        self.player.init(self.state_machine.get_parent().unwrap().cast());
    }
}

#[godot_dyn]
impl IState for PlayerNoneState{
    fn _unhandled_input(&mut self, event: Gd<InputEvent>) {
        // if let Ok(e) = event.try_cast::<InputEventMouseButton>() {
        //     if e.get_button_index() == MouseButton::LEFT && e.is_pressed() {
        //         let ground = self.player.bind().world.get_node_as::<TileMapLayer>("Ground");
        //         let tile_coord = ground.local_to_map(ground.get_local_mouse_position());
        // 
        //         self.player.bind_mut().target_action = TargetAction::MOVE(tile_coord);
        //         self.state_machine.bind_mut()._change_state("Move".into());
        //     }
        // }
    }

    fn _enter(&mut self, args: &[Variant]) {
        todo!()
    }

    fn _exit(&mut self) {
        todo!()
    }

    fn _set_state_machine(&mut self, state_machine: &Gd<StateMachine>) {
        // self.state_machine = Some(state_machine.clone());
        todo!()
    }
}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct MoveState {
    base: Base<Node>,
    state_machine: OnReady<Gd<StateMachine>>,
    target_pos: Vector2i,
    entity: OnReady<Gd<Node2D>>,
}

#[godot_api]
impl INode for MoveState{
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            state_machine: OnReady::from_base_fn(|b| b.get_parent().unwrap().cast()),
            target_pos: Vector2i::ZERO,
            entity: OnReady::manual(),
        }
    }

    fn ready(&mut self) {
        self.entity.init(self.state_machine.get_parent().unwrap().cast());
    }
}

#[godot_dyn]
impl IState for MoveState{
    fn _tick(&mut self) {
        
    }

    fn _enter(&mut self, args: &[Variant]) {
        if args.len() > 0{
            self.target_pos = args[0].to();
        }
    }

    fn _exit(&mut self) {
        todo!()
    }

    fn _set_state_machine(&mut self, state_machine: &Gd<StateMachine>) {
        // self.state_machine = Some(state_machine.clone());
        todo!()
    }
}
