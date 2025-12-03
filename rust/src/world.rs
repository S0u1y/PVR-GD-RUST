use godot::classes::{AStarGrid2D, INode2D, Node2D, Sprite2D, TileMapLayer};
use godot::obj::{Base, WithBaseField};
use godot::prelude::*;
use crate::player::{IEntity};

//Mainly to not use "Magic numbers" for tileset atlas ids.
//If I change it in the editor - I have to change it in here
enum BuildingTiles {
    Wall,
    Floor,
}

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct World{
    base: Base<Node2D>,
    entities: Vec<Gd<Sprite2D>>,
    ground: Option<Gd<TileMapLayer>>,
    buildings: Option<Gd<TileMapLayer>>,
    astar_grid2d: Gd<AStarGrid2D>
}

#[godot_api]
pub impl INode2D for World {
    // fn init(base: Base<Node2D>) -> Self{
    //     godot_print!("Hello, world!");
    //
    //     Self {
    //         base,
    //         entities: Vec::new(),
    //         // ref_count should be reduced after destructor is called automatically
    //         astar_grid2d: AStarGrid2D::new_gd(),
    //         ground: None,
    //         buildings: None,
    //     }
    // }

    fn ready(&mut self) {
        self.ground = Some(self.base().get_node_as::<TileMapLayer>("Ground"));
        self.buildings = Some(self.base().get_node_as("Buildings"));

        let ground = self.ground.as_ref().unwrap();
        let buildings = self.buildings.as_ref().unwrap();

        let ground_rect = ground.get_used_rect();

        self.astar_grid2d.set_region(ground_rect.grow(1));

        let tile_set = ground.get_tile_set().unwrap();
        self.astar_grid2d.set_cell_size(Vector2::new(tile_set.get_tile_size().x as real, tile_set.get_tile_size().y as real));
        self.astar_grid2d.update();

        let walls = buildings.get_used_cells_by_id_ex().source_id(BuildingTiles::Wall as i32).done();
        for wall in walls.iter_shared() {
            self.astar_grid2d.set_point_solid(wall);
        }

    }

}

#[godot_api]
pub impl World {
    //NOTE: Theoretically something similar could be done with signals (observer pattern),
    //      BUT the main reason why I loop through the list is the speed sorting of entities.
    #[func]
    fn on_world_tick(&mut self){
        // godot_print!("World ticked. {}", self.entities.len());
        for entity in self.entities.iter_mut(){
            // entity.call("_move", &[]); //call through godot BUT forgo trait safeties..

            if let Ok(mut e) = entity.clone().try_dynify::<dyn IEntity>(){
                //This took foooor eveeer to figure out haha 😄
                e.dyn_bind_mut().act();
            }

        }
    }

    pub fn register_entity<T>(&mut self, entity: Gd<T>)
    where T: IEntity + AsDyn<dyn IEntity> + Inherits<Sprite2D>
    {
        self.entities.push(entity.upcast());
    }

    pub fn map_to_local(&self, map_coord: Vector2i) -> Vector2{
        self.ground.as_ref().unwrap().map_to_local(map_coord)
    }

    pub fn local_to_map(&self, local_coord: Vector2) -> Vector2i{
        self.ground.as_ref().unwrap().local_to_map(local_coord)
    }

    pub fn get_next_entity_path_coord(&mut self, entity: Gd<Sprite2D>, target_pos: Vector2i) -> Option<Vector2i>
    {
        let curr_pos = self.ground.as_ref().unwrap().local_to_map(entity.get_position());
        self.get_next_path_coord(curr_pos, target_pos)
    }

    pub fn get_next_path_coord(&mut self, current_pos: Vector2i, target_pos: Vector2i) -> Option<Vector2i>{
        let path = self.astar_grid2d.get_id_path(current_pos, target_pos);

        if path.len() < 2{
            None
        } else {
            Some(path.at(1)) //0 is current cell
        }
    }

}
