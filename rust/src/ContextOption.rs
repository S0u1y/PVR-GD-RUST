use godot::prelude::{godot_api, Base, GodotClass, INode, Node};

pub trait IContextOption {
    fn select();
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

impl IContextOption for LootOption {
    fn select() {
        
    }
}
