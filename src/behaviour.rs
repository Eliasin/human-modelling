use crate::entity_state::EntityState;
use crate::graph::Graph;

pub trait Behaviour<K: Copy> {
    fn next_state(&mut self, current_state: EntityState<K>, graph: &Graph<K>) -> EntityState<K>;
}
