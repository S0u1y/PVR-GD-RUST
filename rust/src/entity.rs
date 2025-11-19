//NOTE: every IEntity added to the world HAS to implement AsDyn 
// (use macro #[godot_dyn] above Impl blocks.)
pub trait IEntity{
    fn act(&self);
}