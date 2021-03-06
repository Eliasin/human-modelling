mod entity;
mod graph;
mod schedule;
mod entity_state;
mod behaviour;
mod entity_cluster;
extern crate rand;

use entity_state::EntityState::Stationary;
use entity::Entity;
use entity_cluster::ConnectedEdges;
use entity_cluster::EntityCluster;
use schedule::MarkovChain;
use schedule::Schedule;
use graph::Graph;

use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::fmt::Debug;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Building {
    LectureA,
    LectureB,
    LectureC,
    LabA,
    FoodCourt,
    TimHortons,
    SecondCup,
    Starbucks,
    LibraryFirstFloor
}

impl Display for Building {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Building::LectureA => write!(f, "{}", "LectureA"),
            Building::LectureB => write!(f, "{}", "LectureB"),
            Building::LectureC => write!(f, "{}", "LectureC"),
            Building::LabA => write!(f, "{}", "LabA"),
            Building::FoodCourt => write!(f, "{}", "FoodCourt"),
            Building::TimHortons => write!(f, "{}", "TimHortons"),
            Building::SecondCup => write!(f, "{}", "SecondCup"),
            Building::Starbucks => write!(f, "{}", "Starbucks"),
            Building::LibraryFirstFloor => write!(f, "{}", "LibraryFirstFloor"),
        }
    }
}

fn main() {
    let mut buildings = Graph::<Building>::new(vec!(Building::LectureA, Building::LectureB, Building::LectureC, Building::LabA, Building::FoodCourt, Building::TimHortons, Building::SecondCup, Building::Starbucks, Building::LibraryFirstFloor));

    buildings = buildings.add_undirected_edge(Building::LectureA, Building::FoodCourt, Some(5));
    buildings = buildings.add_undirected_edge(Building::LectureA, Building::LectureB, Some(5));
    buildings = buildings.add_undirected_edge(Building::LectureB, Building::LabA, Some(10));
    buildings = buildings.add_undirected_edge(Building::LectureB, Building::Starbucks, Some(10));
    buildings = buildings.add_undirected_edge(Building::FoodCourt, Building::TimHortons, Some(5));
    buildings = buildings.add_undirected_edge(Building::SecondCup, Building::TimHortons, Some(5));
    buildings = buildings.add_undirected_edge(Building::LibraryFirstFloor, Building::TimHortons, Some(5));

    let mut location_decider_hashmap = HashMap::new();
    location_decider_hashmap.insert(Building::LectureA, (vec!(Building::FoodCourt, Building::LectureB, Building::TimHortons), vec!(1, 1, 1)));
    location_decider_hashmap.insert(Building::FoodCourt, (vec!(Building::LectureA, Building::LectureB), vec!(1, 1)));
    location_decider_hashmap.insert(Building::LectureB, (vec!(Building::LectureA, Building::Starbucks, Building::FoodCourt), vec!(1, 2, 1)));
    location_decider_hashmap.insert(Building::TimHortons, (vec!(Building::LibraryFirstFloor), vec!(1)));
    location_decider_hashmap.insert(Building::Starbucks, (vec!(Building::LibraryFirstFloor), vec!(1)));
    location_decider_hashmap.insert(Building::LibraryFirstFloor, (vec!(Building::LectureA, Building::LectureB), vec!(1, 1)));

    let next_location_decider = MarkovChain::new(location_decider_hashmap);
    let mut when_to_leave_decider = HashMap::new();

    when_to_leave_decider.insert(Building::FoodCourt, 60);
    when_to_leave_decider.insert(Building::LectureA, 90);
    when_to_leave_decider.insert(Building::LectureB, 90);
    when_to_leave_decider.insert(Building::TimHortons, 30);
    when_to_leave_decider.insert(Building::Starbucks, 15);
    when_to_leave_decider.insert(Building::LibraryFirstFloor, 45);

    let typical_student =
        Schedule::new(next_location_decider, when_to_leave_decider);
    let entity = Entity::new(
        Stationary(Building::LectureA, 0),
        typical_student,
    );
    
    let entities = vec!(entity.clone(); 100);
    let lec_a_to_food_court = ConnectedEdges::<Building>{edges: vec!((Building::LectureA, Building::FoodCourt), (Building::FoodCourt, Building::LectureA)), relationship: Box::new(|total| total as f64 / 100. + 1.)};
    let conjestion_relationships = vec!(lec_a_to_food_court);
    let mut entity_cluster = EntityCluster::new(entities, buildings, conjestion_relationships);

    for _ in 0..500 {
        println!("{}", entity_cluster.to_string());
        entity_cluster.apply_timestep(1);
    }
}
