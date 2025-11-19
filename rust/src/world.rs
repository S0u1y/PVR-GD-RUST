use godot::classes::{INode2D, Node2D, Sprite2D};
use godot::global::godot_print;
use godot::obj::{Base};
use godot::prelude::*;
use crate::player::{IEntity};


#[derive(GodotClass)]
#[class(base=Node2D)]
pub struct World{
    base: Base<Node2D>,
    entities: Vec<Gd<Sprite2D>>,
}

#[godot_api]
pub impl INode2D for World {
    fn init(base: Base<Node2D>) -> Self{
        godot_print!("Hello, world!"); // Prints to the Godot console

        Self {
            base,
            entities: Vec::new(),
        }
    }
}

#[godot_api]
pub impl World {
    //NOTE: Theoretically something similar could be done with signals (observer pattern),
    //      BUT the main reason why I loop through the list is the speed sorting of entities.
    #[func]
    fn on_world_tick(&mut self){
        godot_print!("World ticked.");
        for entity in self.entities.iter_mut(){
            // entity.call("_move", &[]); //call through godot BUT forgo trait safeties..

            if let Ok(mut e) = entity.clone().try_dynify::<dyn IEntity>(){
                //This took foooor eveeer to figure out haha 😄
                e.dyn_bind_mut().act();
            }

        }
    }


    pub fn register_entity<T>(&mut self, entity: Gd<T>)

    where T: IEntity + Inherits<Sprite2D>
    {
        self.entities.push(entity.upcast());
    }

}
