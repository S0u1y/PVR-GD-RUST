use godot::builtin::Vector2i;

//NOTE: every IEntity added to the world HAS to implement AsDyn
// (use macro #[godot_dyn] above Impl blocks.)
pub trait IEntity
{
    fn act(&mut self);
}

#[derive(Eq, PartialEq)]
pub enum TargetAction {
    NONE,
    IDLE,
    MOVE(Vector2i),
    ATTACK,
}


