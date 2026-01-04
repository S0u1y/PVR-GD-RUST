use godot::obj::{Singleton, WithUserSignals};
use godot::classes::Texture2D;
use godot::prelude::{godot_api, Base, GString, Gd, GodotClass, IResource, OnEditor, Resource};

#[derive(GodotClass)]
#[class(base=Resource)]
pub struct Item {
    base: Base<Resource>,
    #[export]
    name: GString,
    #[export]
    icon: OnEditor<Gd<Texture2D>>,
    #[export]
    weight: f32,
}

#[godot_api]
impl IResource for Item {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            name: Default::default(),
            weight: 0.0,
            icon: OnEditor::default(),
        }
    }
}

#[derive(GodotClass)]
#[class(tool, base=Resource)]
pub struct SpawnableItem {
    base: Base<Resource>,
    // #[export]
    // item: Option<Gd<Item>>,

    #[export]
    #[var(get, set=_set_spawn_weight)]
    spawn_weight: f64,
    #[export]
    #[var(get)]
    probability: f64,
}

#[godot_api]
impl IResource for SpawnableItem {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            // item: None,
            spawn_weight: 0.,
            probability: 0.,
        }
    }
}

#[godot_api]
impl SpawnableItem {
    #[func]
    pub fn _set_spawn_weight(&mut self, weight: f64) {
        // godot_print!("Spawn Weight setting");
        if self.spawn_weight == weight {
            return;
        }

        self.spawn_weight = weight;

        self.signals().spawn_weight_changed().emit();
    }

    #[signal]
    pub fn spawn_weight_changed();

    pub fn set_probability_from_total_weight(&mut self, total_weight: f64) {
        self.probability = self.spawn_weight / total_weight;
    }

}

