use crate::proc_gen::corridors::MapEdge;

/// Union-Find data structure required for the minimum spanning tree
/// Source: https://github.com/TheAlgorithms/Rust/blob/master/src/graph/disjoint_set_union.rs#L17 (adapted to our purposes)
pub struct UnionFind {
    nodes: Vec<UnionFindNode>,
}

impl UnionFind {
    /// Initializes n+1 disjoint sets, each element being its own parent.
    pub fn new(num_elements: usize) -> Self {
        let mut nodes = Vec::with_capacity(num_elements + 1);
        for index in 0..=num_elements {
            nodes.push(UnionFindNode { parent: index, size: 1 });
        }

        Self { nodes }
    }

    /// Finds representative of set containing the element.
    pub fn find_set(&mut self, element: usize) -> usize {
        if element != self.nodes[element].parent {
            self.nodes[element].parent = self.find_set(self.nodes[element].parent);
        }
        self.nodes[element].parent
    }

    /// Merges the sets containing `element_a` and `element_b` using union by size.
    pub fn merge(&mut self, element_a: usize, element_b: usize) -> usize {
        let mut root_a = self.find_set(element_a);
        let mut root_b = self.find_set(element_b);

        if root_a == root_b {
            return usize::MAX;
        }

        if self.nodes[root_a].size < self.nodes[root_b].size {
            std::mem::swap(&mut root_a, &mut root_b);
        }

        self.nodes[root_b].parent = root_a;
        self.nodes[root_a].size += self.nodes[root_b].size;

        root_a
    }
}

pub struct UnionFindNode {
    parent: usize,
    size: usize,
}

/// Create a minimum spanning tree using the Kruskal algorithm.
/// Source: https://github.com/TheAlgorithms/Rust/blob/master/src/graph/minimum_spanning_tree.rs (adapted to our purposes)
pub fn mst_kruskal(
    mut edges: Vec<MapEdge>,
    num_vertices: usize,
) -> Result<(usize, Vec<MapEdge>), &'static str> {
    let mut union_find = UnionFind::new(num_vertices);
    let mut minimum_spanning_tree_weight: usize = 0;
    let mut minimum_spanning_tree_edges: Vec<MapEdge> = Vec::with_capacity(num_vertices - 1);

    edges.sort_unstable_by_key(|edge| edge.weight);

    for edge in edges {
        if minimum_spanning_tree_edges.len() == num_vertices - 1 {
            break;
        }

        if union_find.merge(edge.source, edge.destination) != usize::MAX {
            minimum_spanning_tree_weight += edge.weight;
            minimum_spanning_tree_edges.push(edge);
        }
    }

    if minimum_spanning_tree_edges.len() == num_vertices - 1 {
        Ok((minimum_spanning_tree_weight, minimum_spanning_tree_edges))
    } else {
        Err("Not all rooms connected")
    }
}
