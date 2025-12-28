#[macro_export]
macro_rules! base_move {
    ($self:ident, $target_pos: expr) => {
        let mut self_base_ref = $self.base_mut().clone();
        let mut world_bind = $self.world.bind_mut();

        let curr_pos = world_bind.local_to_map(self_base_ref.get_position());
        if let Some(next_cell) = world_bind.get_next_path_coord(curr_pos, $target_pos) {
            self_base_ref.set_position(world_bind.map_to_local(next_cell));
            if world_bind.local_to_map(self_base_ref.get_position()) == $target_pos {
                $self.target_action = TargetAction::NONE;
            }
        } else {
            $self.target_action = TargetAction::NONE;
        }
    };
    ($self:ident, $target_pos: expr, $dist_from_target: expr) => {
        let mut self_base_ref = $self.base_mut().clone();
        let mut world_bind = $self.world.bind_mut();
        
        let curr_pos = world_bind.local_to_map(self_base_ref.get_position());
        let path = world_bind.get_path(curr_pos, $target_pos);
        if path.len() > 1 + $dist_from_target {
            self_base_ref.set_position(world_bind.map_to_local(path.at(1)));
            if world_bind.local_to_map(self_base_ref.get_position()) == $target_pos {
                $self.target_action = TargetAction::NONE;
            }
        } else {
            $self.target_action = TargetAction::NONE;
        }
    };
}

#[macro_export]
macro_rules! debug_none {
    ($self:ident) => {
        let curr_pos = $self.world.bind().local_to_map($self.base().get_global_position());
        let next_pos = Vector2i::new(curr_pos.x + randi_range(-1, 1) as i32, curr_pos.y + randi_range(-1, 1) as i32);

        $self.target_action = TargetAction::MOVE(next_pos);
    };
}

