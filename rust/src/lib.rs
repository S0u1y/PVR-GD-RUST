extern crate public;
mod entity;
mod player;
mod stats;
mod world;
mod inventory_slot;
mod mob;
mod base_container;
mod context_options;
mod item;
mod loot_table;

use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

//Below are testing structs for an issue happening on my machine
//potentially a windows related issue
//more in the godot-rust discord thread https://discord.com/channels/723850269347283004/1457101554988613723

/*
#[derive(GodotClass)]
#[class(tool, base=Resource)]
pub struct TestItem {
    base: Base<Resource>,
    #[export]
    #[var(get, set=_set_spawn_weight)]
    weight: f64,
}


#[godot_api]
impl IResource for TestItem {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            weight: 0.,
        }
    }
}

#[godot_api]
impl TestItem {
    #[func]
    pub fn _set_spawn_weight(&mut self, weight: f64) {
        // godot_print!("Spawn Weight setting");
        if self.weight == weight {
            return;
        }

        self.weight = weight;

        self.signals().spawn_weight_changed().emit();
    }

    #[signal]
    pub fn spawn_weight_changed();
}

#[derive(GodotClass)]
#[class(tool, base=Resource)]
pub struct TestList {
    base: Base<Resource>,
    #[export]
    #[var(get)]
    total_weight: f64,
    #[export]
    #[var(get, set=_set_items)]
    list: Array<Option<Gd<TestItem>>>,
}

#[godot_api]
impl IResource for TestList {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            list: Array::new(),
            total_weight: 0.,
        }
    }
}

#[godot_api]
impl TestList {
    #[func]
    pub fn _set_items(&mut self, items: Array<Option<Gd<TestItem>>>) {
        self.total_weight = 0.;

        for item in items.iter_shared() {
            if let Some(mut item) = item {
                self.total_weight += item.bind().get_weight();


                // item.signals().spawn_weight_changed().to_untyped().is_connected(&self.recalculate_chances);
                if !item.is_connected("spawn_weight_changed", &self.base().callable("recalculate_chances")){
                    item.connect("spawn_weight_changed", &self.base().callable("recalculate_chances"));
                }
            }
        }

        self.list = items;
    }

    #[func]
    pub fn recalculate_chances(&mut self) {
        self.total_weight = 0.;

        for item in self.list.iter_shared() {
            if let Some(item) = item {
                self.total_weight += item.bind().get_weight();
            }
        }

        // for item in self.list.iter_shared() {
        //     if let Some(mut item) = item {
        //         item.bind_mut().set_probability_from_total_weight(self.total_weight);
        //     }
        // }
    }

}*/


/*
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

#[derive(GodotClass)]
#[class(tool, base=Resource)]
pub struct LootTable {
    #[export]
    #[var(get)]
    total_weight: f64,
    #[export]
    #[var(get, set=_set_items)]
    items: Array<Option<Gd<SpawnableItem>>>, //When adding in editor, the new pointers are always None


    base: Base<Resource>,
    // recalculate_chances: Callable,
}

#[godot_api]
impl IResource for LootTable {
    fn init(base: Base<Self::Base>) -> Self {

        // let t = base.to_init_gd();

        Self {
            total_weight: 0.,
            items: Array::new(),


            base,
            // recalculate_chances: t.callable("recalculate_chances"),
        }
    }
}

#[godot_api]
impl LootTable {
    #[func]
    pub fn recalculate_chances(&mut self) {
        self.total_weight = 0.;

        for item in self.items.iter_shared() {
            if let Some(item) = item {
                self.total_weight += item.bind().get_spawn_weight();
            }
        }

        for item in self.items.iter_shared() {
            if let Some(mut item) = item {
                item.bind_mut().set_probability_from_total_weight(self.total_weight);
            }
        }
    }

    #[func]
    pub fn _set_items(&mut self, items: Array<Option<Gd<SpawnableItem>>>) {
        self.total_weight = 0.;

        for item in items.iter_shared() {
            if let Some(mut item) = item {
                self.total_weight += item.bind().get_spawn_weight();


                // item.signals().spawn_weight_changed().to_untyped().is_connected(&self.recalculate_chances);
                if !item.is_connected("spawn_weight_changed", &self.base().callable("recalculate_chances")){
                    item.connect("spawn_weight_changed", &self.base().callable("recalculate_chances"));
                }
            }
        }

        self.items = items;
    }

    pub fn spawn_drop(&self) -> Option<Gd<SpawnableItem>> {
        let total_weight = self.total_weight;

        if total_weight == 0. {return None}

        let roll = randf_range(0., total_weight-1.);
        let mut cursor = 0.;

        for item in self.items.iter_shared() {
            if item.is_none() {continue;}

            let item = item.unwrap();
            cursor += item.bind().get_spawn_weight();
            if roll > cursor{
                return Some(item);
            }

        }

        None
    }

}
 */
