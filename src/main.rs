mod entity;
mod graph;
mod schedule;
mod entity_state;
mod behaviour;
extern crate rand;

use entity_state::EntityState::Stationary;
use entity::Entity;
use schedule::MarkovChain;
use schedule::Schedule;
use graph::Graph;

use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Building {
    LectureA,
    FoodCourt,
}

impl Display for Building {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Building::LectureA => write!(f, "{}", "LectureA"),
            Building::FoodCourt => write!(f, "{}", "FoodCourt"),
        }
    }
}

fn main() {
    let mut buildings = Graph::<Building>::new(vec![Building::LectureA, Building::FoodCourt]);

    buildings = buildings.add_undirected_edge(Building::LectureA, Building::FoodCourt, Some(5));

    let mut location_decider_hashmap = HashMap::new();
    location_decider_hashmap.insert(Building::LectureA, (vec![Building::FoodCourt], vec![1]));
    location_decider_hashmap.insert(Building::FoodCourt, (vec![Building::LectureA], vec![1]));

    let next_location_decider = MarkovChain::new(location_decider_hashmap);
    let mut when_to_leave_decider = HashMap::new();

    when_to_leave_decider.insert(Building::FoodCourt, 60);
    when_to_leave_decider.insert(Building::LectureA, 90);

    let mut lecture_food_court_loop =
        Schedule::new(buildings, next_location_decider, when_to_leave_decider);
    let mut entity = Entity::new(
        Stationary(Building::LectureA, 0),
        &mut lecture_food_court_loop,
    );

    for _ in 0..300 {
        println!("{}", entity.to_string());
        entity.apply_timestep(1);
    }

}
