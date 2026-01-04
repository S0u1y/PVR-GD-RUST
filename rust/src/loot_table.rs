use godot::global::randf_range;
use godot::obj::WithBaseField;
use godot::prelude::{godot_api, Array, Base, Callable, Gd, GodotClass, IResource, Resource};
use crate::item::{SpawnableItem};

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

