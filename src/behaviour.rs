use crate::entity_state::EntityState;

pub trait Behaviour<K: Copy> {
    fn next_state(&mut self, current_state: EntityState<K>) -> EntityState<K>;
}
