use godot::classes::{IResource, Resource};
use godot::obj::Base;
use godot::prelude::{godot_api, GodotClass};

#[derive(GodotClass)]
#[class(base=Resource)]
pub struct Stats{
    base: Base<Resource>,
    #[export]
    max_health: i8,
    #[export]
    health: i8,

    #[export]
    thirst: i8,
    #[export]
    hunger: i8,
    #[export]
    energy: i8,
    #[export]
    radiation: i8,

    #[export]
    speed: i8,
    #[export]
    strength: i8,
    #[export]
    dexterity: i8,
    #[export]
    endurance: i8,
}


#[godot_api]
impl IResource for Stats{
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            max_health: 40,
            health: 40,

            thirst: 100,
            hunger: 100,
            energy: 100,
            radiation: 0,

            speed: 1,
            strength: 1,
            endurance: 1,
            dexterity: 1
        }
    }
}
