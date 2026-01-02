use godot::classes::{ITextureButton, TextureButton};
use godot::obj::{Base, OnReady, WithBaseField};
use godot::prelude::{godot_api, Callable, Gd, GdMut, GodotClass};
use crate::context_options::ContextMenu;

#[derive(GodotClass)]
#[class(base=TextureButton)]
struct BaseContainer{
    base: Base<TextureButton>,
    context_menu: OnReady<Gd<ContextMenu>>,

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
            })
        }
    }

    fn ready(&mut self) {}

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

    fn exit_tree(&mut self) {
        if self.context_menu.get_parent() == Option::from(self.to_gd().upcast()) {
            let parent = self.base().get_parent().unwrap();
            self.context_menu.reparent(&parent);
        }
    }

}

impl BaseContainer{
    fn on_mouse_exited(&mut self) {
        self.context_menu.bind_mut().hide_context();

        let s = &self.to_gd();
        if self.base().is_connected("mouse_exited", &Callable::from_object_method(s, "on_mouse_exited")) {
            self.base_mut().disconnect("mouse_exited", &Callable::from_object_method(s, "on_mouse_exited"));
        }

    }
}
