#[macro_export]
macro_rules! base_move {
    ($self:ident, $target_pos: expr) => {
        let mut self_base_ref = $self.base_mut().clone();
        let mut world_bind = $self.world.bind_mut();

        let curr_pos = world_bind.local_to_map(self_base_ref.get_position());
        if let Some(next_cell) = world_bind.get_next_path_coord(curr_pos, $target_pos) {
            self_base_ref.set_position(world_bind.map_to_local(next_cell));
            if world_bind.local_to_map(self_base_ref.get_position()) == $target_pos {
                $self.entity.target_action = TargetAction::NONE;
            }
        }
    };
}