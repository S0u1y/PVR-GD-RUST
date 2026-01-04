use godot::obj::Singleton;
use godot::classes::{ITextureButton, TextureButton};
use godot::obj::{Base, OnReady, WithBaseField};
use godot::prelude::{godot_api, godot_print, Array, Callable, Gd, GodotClass, OnEditor};
use crate::context_options::ContextMenu;
// use crate::{LootTable, SpawnableItem};
use crate::item::SpawnableItem;
use crate::loot_table::LootTable;
// use crate::TestResource;

trait Container {
    fn get_inventory(&self) -> &Array<Gd<SpawnableItem>>;
}

impl Container for BaseContainer {
    fn get_inventory(&self) -> &Array<Gd<SpawnableItem>> {
        &self.inventory
    }
}

#[derive(GodotClass)]
#[class(base=TextureButton)]
pub struct BaseContainer{
    base: Base<TextureButton>,
    context_menu: OnReady<Gd<ContextMenu>>,

    inventory: Array<Gd<SpawnableItem>>,
    #[export]
    loot_table: Option<Gd<LootTable>>,

    // #[export]
    // loot_table: Option<Gd<TestList>>,

    on_mouse_exited: OnReady<Callable>
}

#[godot_api]
impl ITextureButton for BaseContainer{
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            context_menu: OnReady::from_node("../ContextMenu"),
            on_mouse_exited: OnReady::from_base_fn(|b| {
                b.callable("on_mouse_exited")
            }),

            loot_table: None,
            inventory: Array::new(),
        }
    }

    // fn exit_tree(&mut self) {
    //     if let Some(context_parent) = self.context_menu.get_parent() {
    //         if context_parent == self.to_gd().upcast() {
    //             let parent = self.base().get_parent().unwrap();
    //             self.context_menu.reparent(&parent);
    //         }
    //     }
    // }



    #[func(gd_self)]
    fn pressed(mut this: Gd<Self>) {
        let mouse_pos = this.get_global_mouse_position();

        if (mouse_pos - this.get_global_position()).length() > this.get_size().length() { return; }

        let s = this.clone();

        {
            let mut this_bind = this.bind_mut();
            let mut context_bind = this_bind.context_menu.bind_mut();
            context_bind.show_context(mouse_pos, &s);
        }

        if ! this.is_connected("mouse_exited", &this.bind().on_mouse_exited) {
            this.connect("mouse_exited", &s.bind().on_mouse_exited);
        }

    }
}

#[godot_api]
impl BaseContainer{
    #[func]
    fn on_mouse_exited(&mut self) {
        self.context_menu.bind_mut().hide_context();

        let s = &self.to_gd();
        if self.base().is_connected("mouse_exited", &self.on_mouse_exited) {
            self.base_mut().disconnect("mouse_exited", &s.bind().on_mouse_exited);
        }

    }

    pub fn spawn_drops(&mut self){
        if self.loot_table.is_none() {return;}

        godot_print!("test");

        let loot_table = self.loot_table.as_ref().unwrap();
        for i in 0..=100 {
            let spawned_item = loot_table.bind().spawn_drop();
            if spawned_item.is_none() {continue}

            // godot_print!("{}", spawned_item.unwrap().bind().get_item().unwrap().bind().get_name());
        }
    }

}
