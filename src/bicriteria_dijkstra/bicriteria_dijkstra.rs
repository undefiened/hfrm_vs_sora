#[cfg(test)]
#[path = "tests.rs"]
mod tests;

use crate::risks::risks::{RiskMap, Coord};
use priority_queue::PriorityQueue;
use hashbrown::{HashMap, HashSet};
use hashbrown::hash_map::DefaultHashBuilder;
use hashbrown::hash_map::Entry::{Occupied, Vacant};
use std::cmp::Reverse;
use std::fmt::{Display, Formatter};
use ordered_float::OrderedFloat;


struct BicriteriaDijkstra <'a> {
    risk_map: &'a RiskMap,
}

pub struct BicriteriaDijkstraInstance <'a> {
    pub risk_map: &'a RiskMap,
    pub from: Coord<i16>,
    pub to: Coord<i16>,
    pub search_limit: i16,
    pub r_m: f64
}

#[derive(Debug)]
pub struct Path {
    pub path: Vec<Coord<i16>>,
    pub linear_combination_weight: f64,
    pub risk: i32,
    pub length_m: f64,
    pub alpha: f64
}

impl <'a> BicriteriaDijkstraInstance <'a> {
    pub fn new(risk_map: &'a RiskMap, from: Coord<i16>, to: Coord<i16>, search_limit: i16, r: f64) -> Self {
        return Self{
            risk_map,
            from,
            to,
            search_limit,
            r_m: r
        }
    }

    pub fn compute_pareto_apx_paths(&self) -> Vec<Path> {
        let mut paths: Vec<Path> = vec![];

        paths.push(self.run_with_alpha(0.0));
        paths.push(self.run_with_alpha(100000.0));

        let mut intervals_queue: PriorityQueue<(usize, usize), i32, _> = PriorityQueue::new();

        intervals_queue.push((0, 1), 0);

        while !intervals_queue.is_empty() {
            let interval = intervals_queue.pop().unwrap().0;

            let path0 = paths.get(interval.0).unwrap();
            let path1 = paths.get(interval.1).unwrap();

            let beta = (path1.risk-path0.risk) as f64/(path1.length_m-path0.length_m);

            if beta < -0.0000001 {
                let new_path = self.run_with_alpha(-1.0/beta);

                if interval.1 - interval.0 != 1 {
                    panic!("An error with intervals!");
                }

                let index = interval.1;

                if new_path.risk < path0.risk && new_path.length_m < path1.length_m {
                    paths.insert(index, new_path);
                    intervals_queue.push((interval.1-1, interval.1), -(interval.1 as i32));
                    intervals_queue.push((interval.1-1, interval.1), -(interval.1 as i32 - 1));
                }
            }
        }

        return paths
    }

    pub fn run_with_alpha(&self, alpha: f64) -> Path {
        println!("Computing for alpha={}", alpha);
        let mut labels: HashMap<Coord<i16>, f64> = HashMap::new();
        let mut previous_nodes: HashMap<Coord<i16>, Coord<i16>> = HashMap::new();
        let mut pq: PriorityQueue<_, Reverse<OrderedFloat<f64>>, DefaultHashBuilder> = PriorityQueue::<_, Reverse<OrderedFloat<f64>>, DefaultHashBuilder>::with_default_hasher();

        pq.push(self.from, Reverse(OrderedFloat(0.0)));
        labels.insert(self.from, 0.0);
        previous_nodes.insert(self.from, self.from);

        while !pq.is_empty() {
            let current_node = pq.pop();
            let current_node = current_node.unwrap().0;
            let current_label = *labels.get(&current_node).unwrap();

            if current_node == self.to {
                break;
            }

            for neighbour in self.risk_map.neighbours_within(current_node, self.search_limit) {
                let weight = self.risk_map.risk(current_node, neighbour, self.r_m) as f64 * alpha + self.risk_map.length_m(current_node, neighbour);
                let new_label = current_label + weight;

                let mut entry = labels.entry(neighbour);

                match entry {
                    Occupied(mut entry) => {
                        if entry.get() > &new_label {
                            entry.insert(new_label);
                            *previous_nodes.entry(neighbour).or_insert(current_node) = current_node;
                            pq.push(neighbour, Reverse(OrderedFloat(new_label)));
                        }
                    },
                    Vacant(entry) => {
                        entry.insert(new_label);
                        *previous_nodes.entry(neighbour).or_insert(current_node) = current_node;
                        pq.push(neighbour, Reverse(OrderedFloat(new_label)));
                    }
                }
            }
        }

        return self.unwrap_path(&previous_nodes, &labels, alpha);
    }

    fn unwrap_path(&self, nodes_previous: &HashMap<Coord<i16>, Coord<i16>>, nodes_labels: &HashMap<Coord<i16>, f64>, alpha: f64) -> Path {
        let mut path = vec![];
        let mut total_risk = 0;
        let mut total_length = 0.0;

        let mut previous_node = &self.to;

        while previous_node != &self.from {
            path.push(*previous_node);

            let new_previous_node = nodes_previous.get(previous_node).unwrap();

            total_risk += self.risk_map.risk(*previous_node, *new_previous_node, self.r_m);
            total_length += self.risk_map.length_m(*previous_node, *new_previous_node);

            previous_node = new_previous_node;
        }

        path.push(self.from);
        total_risk += self.risk_map.risk(*previous_node, self.from, self.r_m);
        total_length += self.risk_map.length_m(*previous_node, self.from);

        return Path{
            path,
            linear_combination_weight: *nodes_labels.get(&self.to).unwrap(),
            risk: total_risk,
            length_m: total_length,
            alpha: alpha
        }
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "linear_weight: {}, risk: {}, length: {}, path: {:?}", &self.linear_combination_weight, &self.risk, &self.length_m, &self.path)
    }
}