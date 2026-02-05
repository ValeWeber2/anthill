use rand::Rng;

use crate::{
    ai::pathfinding::a_star,
    proc_gen::{bsp::MapBSP, bsp_nodes::NodeId, mst::kruskal},
};

#[derive(Clone)]
pub struct MapEdge {
    pub(crate) source: NodeId,
    pub(crate) destination: NodeId,
    pub(crate) weight: usize,
}

impl MapBSP {
    pub fn all_edges(&self, nodes: Vec<NodeId>) -> Vec<MapEdge> {
        let mut edges: Vec<MapEdge> = Vec::new();

        for node_a_id in nodes.clone() {
            let node_a = self.get_node(node_a_id);
            let node_a_center = node_a.center();

            for node_b_id in nodes.clone() {
                if node_a_id == node_b_id {
                    continue;
                }

                let node_b = self.get_node(node_b_id);
                let node_b_center = node_b.center();

                let distance = node_a_center.distance_squared_from(node_b_center);

                edges.push(MapEdge { source: node_a_id, destination: node_b_id, weight: distance })
            }
        }

        edges
    }

    pub fn find_node_connections<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec<MapEdge> {
        let mut leaves = Vec::new();
        self.get_leaves(self.root, &mut leaves);
        let edges = self.all_edges(leaves.clone());

        let mut connections = match kruskal(edges.clone(), self.nodes.len()) {
            // Kruskal should normally return something valid.
            Some((_, connections)) => connections,

            // If not, just connect pairs, ugly, but better than nothing in an emergeynce.
            None => {
                let mut connections: Vec<MapEdge> = Vec::new();
                for pair in leaves.windows(2) {
                    connections.push(MapEdge { source: pair[0], destination: pair[1], weight: 1 });
                }
                connections
            }
        };

        // Extra corridors for Jaquaysing
        for edge in edges {
            if rng.random_bool(0.01) {
                connections.push(edge);
            }
        }

        connections
    }

    pub fn a_star_corridors<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        let connections = self.find_node_connections(rng);

        for connection in connections {
            let path = a_star(
                self.get_node(connection.source).center(),
                self.get_node(connection.destination).center(),
                |_p| true,
            )
            .expect("There should always be paths possible");

            for point in path {
                self.corridors.push(point);
            }
        }
    }
}
