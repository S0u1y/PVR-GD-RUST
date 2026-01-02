use std::ops::Deref;
use godot::classes::{IItemList, ItemList};
use godot::obj::WithBaseField;
use godot::prelude::{godot_api, godot_dyn, godot_print, Base, Gd, GodotClass, INode, Node, OnReady, Vector2};

pub trait IContextOption {
    fn select(&self);
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

impl ContextMenu {
    fn show_context(&mut self, position: Vector2, node: &Gd<Node>) {
        self.hide_context();
        let mut base = self.base_mut().clone();

        self.context_options = Some(base.get_node_as::<Node>("ContextOptions"));
        if self.context_options.is_some() {return;}

        for child in self.context_options.as_ref().unwrap().get_children().iter_shared() {
            base.add_item(&child.get_name().to_string());
        }

        base.reparent(node);
        base.set_global_position(position);

        base.set_visible(true);

    }

    fn hide_context(&mut self) {
        self.context_options.take();
        let mut s = self.base_mut();
        s.set_visible(false);
        s.clear();
    }

}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct LootOption {
    base: Base<Node>
}

#[godot_api]
impl INode for LootOption{
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base
        }
    }
}

#[godot_dyn]
impl IContextOption for LootOption {
    fn select(&self) {
        godot_print!("Looting selected :)");
    }
}
