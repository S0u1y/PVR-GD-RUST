use godot::classes::{IItemList, ItemList};
use godot::obj::{OnReady, WithBaseField};
use godot::prelude::{godot_api, godot_dyn, godot_print, Base, DynGd, FromGodot, Gd, GodotClass, INode, Inherits, Node, OnEditor, Vector2};
use crate::base_container::BaseContainer;

pub trait IContextOption {
    fn select(&mut self);
}

#[derive(GodotClass)]
#[class(base=ItemList)]
pub struct ContextMenu {
    base: Base<ItemList>,
    context_options: Option<Gd<Node>>
}

#[godot_api]
impl IItemList for ContextMenu{
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            context_options: None
        }
    }
}

#[godot_api]
impl ContextMenu {
    pub fn show_context<T>(&mut self, position: Vector2, node: &Gd<T>)
    where T: Inherits<Node>
    {
        self.hide_context();

        let n = node.clone().upcast();
        self.context_options = Some(n.get_node_as::<Node>("ContextOptions"));
        // if self.context_options.is_none() {return;}

        let mut base = self.base_mut().clone();

        for child in self.context_options.as_ref().unwrap().get_children().iter_shared() {
            base.add_item(&child.get_name().to_string());
        }

        base.reparent(node);
        base.set_global_position(position);

        base.set_visible(true);

    }

    pub fn hide_context(&mut self) {
        self.context_options.take();
        let mut s = self.base_mut();
        s.set_visible(false);
        s.clear();
    }

    #[func]
    fn item_selected(&mut self, index: i32) {
        let node = self.context_options.as_ref().unwrap().get_child(index).unwrap();
        let mut n = DynGd::<Node, dyn IContextOption>::from_godot(node);

        n.dyn_bind_mut().select();

        self.hide_context();
    }

}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct LootOption {
    base: Base<Node>,
    base_container: OnReady<Gd<BaseContainer>>,
}

#[godot_api]
impl INode for LootOption{
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            base_container: OnReady::manual(),
        }
    }

    fn ready(&mut self) {
        if let Some(owner) = self.base().get_owner() {
            self.base_container.init(owner.cast());
        }
    }
}

#[godot_dyn]
impl IContextOption for LootOption {
    fn select(&mut self) {

        self.base_container.bind_mut().spawn_drops();

        godot_print!("Looting selected :)aaaaaaaaaaaaaaaaa");
    }
}
