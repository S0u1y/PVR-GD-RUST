#[public]
struct Stats {
    max_health: i8,
    health: i8,
    
    thirst: i8,
    hunger: i8,
    energy: i8,
    radiation: i8,

    speed: i8,
    strength: i8,
    dexterity: i8,
    endurance: i8,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            max_health: 100,
            health: 100,
            
            hunger: 100,
            thirst: 100,
            energy: 100,
            radiation: 0,
            
            speed: 1,
            strength: 1,
            dexterity: 1,
            endurance: 1,
        }
    }
}
