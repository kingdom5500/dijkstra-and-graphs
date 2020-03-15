use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::ops::Add;

use std::fmt::Debug;

macro_rules! graph {
    (
        $(
            $start_vertex: expr => [
                $( $edge_value:expr => $end_vertex:expr ),*
            ]
        ),*
    ) => {{
        let mut graph = Graph::empty();

        // add all of the vertices
        $( graph.add_vertex($start_vertex); )*

        // then make all of the connections
        $( $( graph.connect_vertices(
               &$start_vertex,
               &$end_vertex,
               $edge_value
           ).unwrap();
        )*)*
        graph
    }}
}

#[derive(Debug)]
struct Edge<'a, V: Hash + Eq, E> {
    v1: &'a V,
    v2: &'a V,
    pub value: E,
}

#[derive(Debug)]
pub struct Graph<'a, V: Hash + Eq, E> {
    vertices: HashSet<V>,
    edges: Vec<Edge<'a, V, E>>,
}

impl<'a, V, E> Edge<'a, V, E>
where
    V: Hash + Eq,
{
    fn new(v1: &'a V, v2: &'a V, value: E) -> Self {
        Self {
            v1: v1,
            v2: v2,
            value: value,
        }
    }
}

impl<'a, V, E> Graph<'a, V, E>
where
    V: Hash + Eq,
{
    pub fn empty() -> Self {
        Self {
            vertices: HashSet::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_vertex(&mut self, value: V) {
        self.vertices.insert(value);
    }

    pub fn contains(&self, value: &V) -> bool {
        self.vertices.contains(&value)
    }

    pub fn connect_vertices(
        &mut self,
        v1: &'a V,
        v2: &'a V,
        edge_value: E,
    ) -> Result<(), &'static str> {
        if !(self.contains(v1) && self.contains(v2)) {
            return Err("Graph does not contain both vertices.");
        }

        self.edges.push(Edge::new(v1, v2, edge_value));

        Ok(())
    }

    pub fn neighbors(&self, vertex: &V) -> Vec<(&V, &E)> {
        let mut neighbors = Vec::new();

        for edge in self.edges.iter() {
            let neighbor = if vertex == edge.v1 {
                Some(edge.v2)
            } else if vertex == edge.v2 {
                Some(edge.v1)
            } else {
                None
            };

            if let Some(other_vertex) = neighbor {
                neighbors.push((other_vertex, &edge.value))
            }
        }

        neighbors
    }

    pub fn value_between(&self, v1: &V, v2: &V) -> Option<&E> {
        for edge in self.edges.iter() {
            let forward_link = edge.v1 == v1 && edge.v2 == v2;
            let backward_link = edge.v1 == v2 && edge.v2 == v1;

            if forward_link || backward_link {
                return Some(&edge.value);
            }
        }

        None
    }
}

impl<'a, V, E> Graph<'a, V, E>
where
    V: Hash + Eq,
    E: Add<Output = E> + Ord + Clone,
{
    pub fn dijkstra_paths(&self, source: &V) -> HashMap<&V, E> {
        // this implementation of dijkstra's algorithm is a little
        // different from a typical version. since a generic type E
        // is used for the edge weights, we do not know which values
        // are analogous to zero and infinity. many implementations
        // of this algorithm would use those values as provisional
        // distances from the source, but we cannot do that here.
        let mut distances: HashMap<&V, E> = HashMap::new();
        let mut unvisited_vertices: HashSet<&V> = HashSet::new();

        // the first iteration of the algorithm happens here.
        for vertex in self.vertices.iter() {
            // skip over the source here because we're dealing with
            // its neighbors in this loop instead of the main loop.
            if vertex == source {
                continue;
            }

            unvisited_vertices.insert(vertex);

            // if the current vertex is a neighbor to the source,
            // take note of the distance of the edge between them.
            if let Some(source_dist) = self.value_between(source, vertex) {
                distances.insert(vertex, source_dist.clone());
            }
        }

        while !unvisited_vertices.is_empty() {
            // search through the unvisited vertices to find which
            // one has the lowest provisional distance.
            let &nearest_vertex = unvisited_vertices
                .iter()
                .filter(|&v| distances.contains_key(v))
                .min_by_key(|&v| distances.get(v))
                .unwrap();

            unvisited_vertices.remove(nearest_vertex);

            // this seem convoluted, but it prevents an error from
            // the coexistence of mutable and immutable references.
            let dist_entry = distances.get(nearest_vertex);
            let nearest_dist = dist_entry.unwrap().clone();

            for &(vertex, edge_len) in self.neighbors(nearest_vertex).iter() {
                // for each neighboring vertex, we check if passing
                // through the current vertex allows for a smaller
                // distance than the shortest path checked so far.
                let alt_dist = nearest_dist.clone() + edge_len.clone();
                let prev_dist = distances.get(vertex);

                if prev_dist.is_none() || alt_dist < *prev_dist.unwrap() {
                    distances.insert(vertex, alt_dist);
                }
            }
        }

        // this implementation would produce an erroneous distance
        // for the source (and we can't just set it to zero since we
        // don't know the analogous value for the generic type E) so
        // let's just get rid of it. it's a trivial result anyway.
        if distances.contains_key(source) {
            distances.remove(source);
        }

        distances
    }
}
