use godot::builtin::Vector2i;
use godot::prelude::Gd;
use crate::stats::Stats;

//NOTE: every IEntity added to the world HAS to implement AsDyn
// (use macro #[godot_dyn] above Impl blocks.)
pub trait IEntity
{
    fn act(&mut self);
    fn get_stats(&self) -> &Gd<Stats>;
}

#[derive(Eq, PartialEq)]
pub enum TargetAction {
    NONE,
    IDLE,
    MOVE(Vector2i),
    ATTACK,
}


