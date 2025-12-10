use crate::entity::IEntity;
use godot::classes::{AStarGrid2D, INode2D, Node2D, Sprite2D, TileMapLayer};
use godot::obj::{Base, Bounds, bounds};
use godot::prelude::*;

//Mainly to not use "Magic numbers" for tileset atlas ids.
//If I change it in the editor - I have to change it in here
enum BuildingTiles {
    Wall,
    Floor,
}

#[derive(GodotClass)]
#[class(init, base=Node2D)]
pub struct World {
    base: Base<Node2D>,
    entities: Vec<DynGd<Sprite2D, dyn IEntity>>,
    #[init(node = "Ground")]
    ground: OnReady<Gd<TileMapLayer>>,
    #[init(node = "Buildings")]
    buildings: OnReady<Gd<TileMapLayer>>,
    astar_grid2d: Gd<AStarGrid2D>,
}

#[godot_api]
pub impl INode2D for World {
    // Automatically handled by class(init)
    // fn init(base: Base<Node2D>) -> Self{
    //     godot_print!("Hello, world!");
    //
    //     Self {
    //         base,
    //         entities: Vec::new(),
    //         // ref_count should be reduced after destructor is called automatically
    //         astar_grid2d: AStarGrid2D::new_gd(),
    //     }
    // }

    fn ready(&mut self) {
        let ground = self.ground.clone();
        let buildings = self.buildings.clone();

        let ground_rect = ground.get_used_rect();

        self.astar_grid2d.set_region(ground_rect.grow(1));

        let tile_set = ground.get_tile_set().unwrap();
        self.astar_grid2d.set_cell_size(Vector2::new(
            tile_set.get_tile_size().x as real,
            tile_set.get_tile_size().y as real,
        ));
        self.astar_grid2d.update();

        let walls = buildings
            .get_used_cells_by_id_ex()
            .source_id(BuildingTiles::Wall as i32)
            .done();
        for wall in walls.iter_shared() {
            self.astar_grid2d.set_point_solid(wall);
        }
    }
}

#[godot_api]
pub impl World {
    #[func(gd_self)]
    fn on_world_tick(mut this: Gd<Self>) {
        let s = this.bind_mut();
        let mut entities = (&s).entities.clone();
        drop(s); //Drop self bind, so entities can use this object
        for entity in entities.iter_mut() {
            entity.dyn_bind_mut().act();
        }
    }

    //TODO: sort entities by speed, in the beginning just use Vec, but later switch to a mix of B-Tree and Linked List.
    pub fn register_entity<T>(&mut self, entity: Gd<T>)
    where
        T: AsDyn<dyn IEntity> + Inherits<Sprite2D> + Bounds<Declarer = bounds::DeclUser>,
    {
        self.entities.push(entity.into_dyn().upcast());
    }

    pub fn map_to_local(&self, map_coord: Vector2i) -> Vector2 {
        self.ground.map_to_local(map_coord)
    }

    pub fn local_to_map(&self, local_coord: Vector2) -> Vector2i {
        self.ground.local_to_map(local_coord)
    }

    pub fn get_next_entity_path_coord(
        &mut self,
        entity: Gd<Sprite2D>,
        target_pos: Vector2i,
    ) -> Option<Vector2i> {
        let curr_pos = self.local_to_map(entity.get_position());
        self.get_next_path_coord(curr_pos, target_pos)
    }

    pub fn get_next_path_coord(
        &mut self,
        current_pos: Vector2i,
        target_pos: Vector2i,
    ) -> Option<Vector2i> {
        let path = self.astar_grid2d.get_id_path(current_pos, target_pos);

        if path.len() < 2 {
            None
        } else {
            Some(path.at(1)) //0 is current cell
        }
    }
}
