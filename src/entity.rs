use crate::behaviour::Behaviour;
use crate::entity_state::EntityState;
use EntityState::Stationary;
use EntityState::Traversing;

use std::fmt::Display;
use std::fmt;

pub struct Entity<'a, K: Copy, B: Behaviour<K>> {
    state: EntityState<K>,
    behaviour: &'a mut B,
}

impl<'a, K: Display + Copy, B: Behaviour<K>> Entity<'a, K, B> {
    pub fn to_string(&self) -> String {
        self.state.to_string()
    }
}

impl<'a, K: Copy, B: Behaviour<K>> Entity<'a, K, B> {
    pub fn new(state: EntityState<K>, behaviour: &'a mut B) -> Entity<'a, K, B> {
        Entity {
            state: state,
            behaviour: behaviour,
        }
    }

    pub fn apply_timestep(&mut self, timestep: u32) {
        self.state = self.state.apply_timestep(timestep, self.behaviour);
    }
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
    ) -> EntityState<K> {
        if timestep == 0 {
            *self
        } else {
            behaviour.next_state(*self).apply_timestep(timestep - 1, behaviour)
        }
    }
}
