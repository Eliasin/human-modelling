use std::collections::hash_map;
use std::collections::HashMap;
use std::hash::Hash;

type AdjacencyMatrix<K> = HashMap<K, HashMap<K, Option<i32>>>;

#[derive(Clone)]
pub struct Graph<K> {
    adjacency: AdjacencyMatrix<K>,
}

impl<K: Eq + Hash + Copy> Graph<K> {
    pub fn new(nodes: Vec<K>) -> Graph<K> {
        let mut empty_adjacency = HashMap::new();
        for node in &nodes {
            empty_adjacency.insert(*node, None);
        }

        let mut hash_map = HashMap::new();
        for node in &nodes {
            hash_map.insert(*node, empty_adjacency.clone());
        }

        Graph {
            adjacency: hash_map,
        }
    }

    pub fn add_node(&self, new_node: K) -> Graph<K> {
        let mut new_hash_map = self.adjacency.clone();
        new_hash_map.insert(new_node, HashMap::new());
        Graph {
            adjacency: new_hash_map,
        }
    }

    pub fn add_edge(&self, from: K, to: K, weight: Option<i32>) -> Graph<K> {
        match self.adjacency.get(&from) {
            Some(nodes) => {
                let mut new_nodes = nodes.clone();
                new_nodes.insert(to, weight);

                let mut new_hash_map = self.adjacency.clone();
                *(new_hash_map.get_mut(&from).unwrap()) = new_nodes;

                Graph {
                    adjacency: new_hash_map,
                }
            }
            None => Graph {
                adjacency: self.adjacency.clone(),
            },
        }
    }

    pub fn add_undirected_edge(&self, from: K, to: K, weight: Option<i32>) -> Graph<K> {
        self.add_edge(from, to, weight).add_edge(to, from, weight)
    }

    pub fn contains(&self, target_node: K) -> bool {
        self.adjacency.contains_key(&target_node)
    }

    pub fn remove_node(&self, target_node: K) -> Graph<K> {
        let mut new_hash_map = self.adjacency.clone();
        new_hash_map.remove(&target_node);
        Graph {
            adjacency: new_hash_map,
        }
    }

    pub fn neighbours_of(&self, node: K) -> Option<hash_map::Keys<'_, K, Option<i32>>> {
        match self.adjacency.get(&node) {
            Some(neighbours) => Some(neighbours.keys()),
            None => None,
        }
    }

    pub fn weight_of(&self, from: K, to: K) -> Option<i32> {
        match self
            .adjacency
            .get(&from)
            .map(|weight_map| weight_map.get(&to))
            .flatten()
        {
            Some(weight) => Option::clone(weight),
            None => None,
        }
    }
}
