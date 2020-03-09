use crate::graph::Graph;
use crate::behaviour::Behaviour;
use EntityState::Stationary;
use EntityState::Traversing;

use std::fmt;
use std::fmt::Display;

#[derive(Clone, Copy)]
pub enum EntityState<K: Copy> {
    Stationary(K, u32),
    Traversing(K, K, u32),
}

impl<K: Display + Copy> Display for EntityState<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stationary(location, time_spent) => write!(f, "{} {}", location, time_spent),
            Traversing(from, to, time_spent) => write!(f, "{} {} {}", from, to, time_spent)
        }
    }
}

impl<K: Copy> EntityState<K> {
    pub fn apply_timestep(
        &self,
        timestep: u32,
        behaviour: &mut dyn Behaviour<K>,
        graph: &Graph<K>
    ) -> EntityState<K> {
        if timestep == 0 {
            *self
        } else {
            behaviour.next_state(*self, graph).apply_timestep(timestep - 1, behaviour, graph)
        }
    }
}
