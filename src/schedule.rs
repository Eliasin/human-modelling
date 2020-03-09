use crate::entity_state::EntityState;
use EntityState::Stationary;
use EntityState::Traversing;
use crate::behaviour::Behaviour;
use crate::graph::Graph;

use rand::distributions::WeightedIndex;
use rand::distributions::Distribution;
use rand::rngs::ThreadRng;

use std::hash::Hash;
use std::collections::HashMap;

#[derive(Clone)]
pub struct MarkovChain<K> {
    chain: HashMap<K, (Vec<K>, Vec<u32>)>,
    rand: ThreadRng,
}

impl<K: Eq + Hash + Copy> MarkovChain<K> {
    pub fn next_state(&mut self, current_state: K) -> Option<K> {
        match self.chain.get(&current_state) {
            Some(adjacency) => {
                let (next_states, weights) = adjacency;
                match WeightedIndex::new(weights) {
                    Ok(dist) => Some(next_states[dist.sample(&mut self.rand)]),
                    Err(_str) => None,
                }
            }
            None => None,
        }
    }

    pub fn new(chain: HashMap<K, (Vec<K>, Vec<u32>)>) -> MarkovChain<K> {
        MarkovChain {
            chain: chain,
            rand: rand::thread_rng(),
        }
    }
}

#[derive(Clone)]
pub struct Schedule<K> {
    next_location_decider: MarkovChain<K>,
    when_to_leave_decider: HashMap<K, u32>,
}

impl<K: Eq + Hash> Schedule<K> {
    pub fn should_leave(&self, location: K, time_spent: u32) -> Option<bool> {
        match self.when_to_leave_decider.get(&location) {
            Some(max_time) => Some(time_spent >= *max_time),
            None => None,
        }
    }

    pub fn new(
        next_location_decider: MarkovChain<K>,
        when_to_leave_decider: HashMap<K, u32>,
    ) -> Schedule<K> {
        Schedule {
            next_location_decider: next_location_decider,
            when_to_leave_decider: when_to_leave_decider,
        }
    }
}

impl<K: Eq + Hash + Copy> Behaviour<K> for Schedule<K> {
    fn next_state(&mut self, current_state: EntityState<K>, graph: &Graph<K>) -> EntityState<K> {
        match current_state {
            Stationary(location, time_spent) => {
                if self.should_leave(location, time_spent).unwrap() {
                    let next_location = self.next_location_decider.next_state(location).unwrap();
                    Traversing(location, next_location, 0)
                } else {
                    Stationary(location, time_spent + 1)
                }
            }
            Traversing(from, to, time_spent) => {
                let traversal_time = graph.weight_of(from, to).unwrap() as u32;
                if time_spent >= traversal_time {
                    Stationary(to, 0)
                } else {
                    Traversing(from, to, time_spent + 1)
                }
            }
        }
    }
}
