use crate::behaviour::Behaviour;
use crate::entity_state::EntityState;
use crate::graph::Graph;
use EntityState::Stationary;
use EntityState::Traversing;

use std::fmt::Display;

pub struct Entity<K: Copy, B: Behaviour<K>> {
    state: EntityState<K>,
    behaviour: Box<B>,
}

impl<K: Copy, B: Behaviour<K> + Clone> Clone for Entity<K, B> {
    fn clone(&self) -> Entity<K, B> {
        Entity{state: self.state, behaviour: Box::new((*self.behaviour).clone())}
    }
}

impl<K: Display + Copy, B: Behaviour<K>> Entity<K, B> {
    pub fn to_string(&self) -> String {
        self.state.to_string()
    }
}

impl<K: Copy + Eq, B: Behaviour<K>> Entity<K, B> {
    pub fn new(state: EntityState<K>, behaviour: B) -> Entity<K, B> {
        Entity {
            state: state,
            behaviour: Box::new(behaviour),
        }
    }

    pub fn apply_timestep(&mut self, timestep: u32, graph: &Graph<K>) {
        self.state = self.state.apply_timestep(timestep, &mut *self.behaviour, graph);
    }

    pub fn is_traversing_path(&self, from: K, to: K) -> bool {
        match self.state {
            Stationary(_, _) => false,
            Traversing(from_current, to_current, _) => from == from_current && to == to_current
        }
    }
}
