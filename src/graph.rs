use std::collections::VecDeque;
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

    fn find_path_length_in_came_from(&self, start: K, target: K, came_from: HashMap<K, K>) -> i32 {
        let mut curr = target;
        let mut total_length = 0;
        while curr != start {
            let last = came_from.get(&curr).unwrap();
            total_length += self.weight_of(*last, curr).unwrap(); 
            curr = *last;
        }
        total_length
    }

    pub fn generate_complete_graph(&self) -> Graph<K> {
        let nodes: Vec<K> = self.adjacency.keys().map(|node| node.clone()).collect();
        let mut new_graph = Graph::new(nodes.clone());

        for nodeA in &nodes {
            for nodeB in &nodes {
                new_graph = new_graph.add_edge(*nodeA, *nodeB, self.dfs(*nodeA, *nodeB));
            }
        }

        new_graph
    }

    pub fn dfs(&self, start: K, target: K) -> Option<i32> {
        if start == target {
            return Some(0);
        }
        let mut visited = vec!(start);
        let mut need_visit = VecDeque::new();
        let mut came_from: HashMap<K, K> = HashMap::new();
        let mut curr = start;

        for node in self.neighbours_of(curr).unwrap() {
            if !visited.contains(&node) && !need_visit.contains(&node) {
                need_visit.push_back(node);
                came_from.insert(node, curr);
            }
        }

        while !need_visit.is_empty() {
           curr = need_visit.pop_front().unwrap();

            if curr == target {
                return Some(self.find_path_length_in_came_from(start, target, came_from));
            }

            for node in self.neighbours_of(curr).unwrap() {
                if !visited.contains(&node) && !need_visit.contains(&node) {
                    need_visit.push_back(node);
                    came_from.insert(node, curr);
                }
            }

            visited.push(curr);
        }

        None
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

    pub fn neighbours_of(&self, node: K) -> Option<Vec<K>> {
        match self.adjacency.get(&node) {
            Some(neighbours) => Some(neighbours.keys().filter(|node| neighbours.get(&node).unwrap().is_some()).map(|node| node.clone()).collect()),
            None => None
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
