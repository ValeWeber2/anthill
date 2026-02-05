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
    pub fn all_edges(&self, nodes: &[NodeId]) -> (Vec<MapEdge>, Vec<NodeId>) {
        let mut edges: Vec<MapEdge> = Vec::new();

        for (i, &node_a_id) in nodes.iter().enumerate() {
            let node_a_center = self.get_node(node_a_id).center();

            for (j, &node_b_id) in nodes.iter().enumerate().skip(i + 1) {
                // Skip to avoid repetition (+1 tos kip i=j)
                let node_b_center = self.get_node(node_b_id).center();

                let distance = node_a_center.distance_squared_from(node_b_center);

                edges.push(MapEdge { source: i, destination: j, weight: distance })
            }
        }

        (edges, nodes.to_vec())
    }

    pub fn find_node_connections<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
    ) -> (Vec<MapEdge>, Vec<NodeId>) {
        let (edges, node_ids) = self.all_edges(&self.rooms);

        let mut connections = match kruskal(edges.clone(), self.rooms.len()) {
            // Kruskal should normally return something valid.
            Ok((_, connections)) => connections,

            // If not, just connect pairs, ugly, but better than nothing in an emergeynce.
            Err(_) => {
                let mut connections: Vec<MapEdge> = Vec::new();
                for pair in self.rooms.clone().windows(2) {
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

        (connections, node_ids)
    }

    pub fn a_star_corridors<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        let (connections, node_ids) = self.find_node_connections(rng);

        for connection in connections {
            let node_a_id = node_ids[connection.source];
            let node_b_id = node_ids[connection.destination];

            let path = a_star(
                self.get_node(node_a_id).center(),
                self.get_node(node_b_id).center(),
                |_p| true,
            )
            .expect("There should always be paths possible");

            self.corridors.extend(path);
        }
    }
}
