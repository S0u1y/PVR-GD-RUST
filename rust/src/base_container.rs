use godot::classes::{ITextureButton, ItemList, TextureButton};
use godot::obj::{Base, OnReady, WithBaseField};
use godot::prelude::{godot_api, DynGd, Gd, GodotClass, Node};
use crate::context_options::IContextOption;

#[derive(GodotClass)]
#[class(base=TextureButton)]
struct BaseContainer{
    base: Base<TextureButton>,
    context_menu: OnReady<Gd<ItemList>>,
}

#[godot_api]
impl ITextureButton for BaseContainer{
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            context_menu: OnReady::from_node("../ItemList"),

        }
    }

    fn pressed(&mut self) {
        // self.context_menu.into_dyn()
    }

    fn ready(&mut self) {}
}

