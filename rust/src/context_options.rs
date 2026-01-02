use godot::classes::{IItemList, ItemList};
use godot::obj::WithBaseField;
use godot::prelude::{godot_api, godot_dyn, godot_print, Base, Gd, GodotClass, INode, Inherits, Node, Vector2};

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
    pub fn show_context<T>(&mut self, position: Vector2, node: &Gd<T>)
    where T: Inherits<Node>
    {
        self.hide_context();

        let mut base = self.base_mut().clone();

        let n = node.clone().upcast();
        self.context_options = Some(n.get_node_as::<Node>("ContextOptions"));
        // if self.context_options.is_some() {return;}

        godot_print!("test");

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
