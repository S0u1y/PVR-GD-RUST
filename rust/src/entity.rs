use godot::builtin::Vector2i;
use crate::stats::Stats;

//NOTE: every IEntity added to the world HAS to implement AsDyn
// (use macro #[godot_dyn] above Impl blocks.)
pub trait IEntity
{
    fn act(&mut self);
}

#[derive(Eq, PartialEq)]
pub enum TargetAction {
    NONE,
    MOVE(Vector2i),
    IDLE,
}

#[public]
struct Entity {
    target_action: TargetAction,
    stats: Stats
}

impl Entity {
    pub fn new() -> Self {
        Self {
            target_action: TargetAction::NONE,
            stats: Stats {
                ..Default::default()
            }
        }
    }

    pub fn from_stats(stats: Stats) -> Self {
        Self {
            target_action: TargetAction::NONE,
            stats
        }
    }

}

