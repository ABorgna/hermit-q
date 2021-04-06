use crate::zx::graph::*;
use petgraph;
use std::collections::{hash_set, HashSet};

#[derive(Debug, PartialEq, Eq, Default)]
struct VertexData {
    phase: Phase,
    kind: VertexKind,
}

impl VData for VertexData {
    fn phase(&self) -> Phase {
        self.phase
    }

    fn phase_mut(&mut self) -> &mut Phase {
        &mut self.phase
    }

    fn kind(&self) -> VertexKind {
        self.kind
    }

    fn kind_mut(&mut self) -> &mut VertexKind {
        &mut self.kind
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
struct EdgeData {
    kind: EdgeKind,
}

impl EData for EdgeData {
    fn kind(&self) -> EdgeKind {
        self.kind
    }

    fn kind_mut(&mut self) -> &mut EdgeKind {
        &mut self.kind
    }
}

struct GGraph<Ix = u32> {
    g: petgraph::Graph<VertexData, EdgeData, petgraph::Undirected, Ix>,
    inputs: HashSet<petgraph::graph::NodeIndex<Ix>>,
    outputs: HashSet<petgraph::graph::NodeIndex<Ix>>,
}

impl<'a, Ix: Copy> Graph<'a> for GGraph<Ix>
where
    Ix: petgraph::graph::IndexType,
{
    type VertexIx = petgraph::graph::NodeIndex<Ix>;

    type EdgeIx = petgraph::graph::EdgeIndex<Ix>;

    type VertexData = VertexData;

    type EdgeData = EdgeData;

    type EdgeIterator = petgraph::graph::EdgeIndices<Ix>;

    type VertexIterator = petgraph::graph::NodeIndices<Ix>;

    type NeighbourIterator = petgraph::graph::Neighbors<'a, EdgeData, Ix>;

    type InputIterator = hash_set::Iter<'a, petgraph::graph::NodeIndex<Ix>>;

    type OutputIterator = hash_set::Iter<'a, petgraph::graph::NodeIndex<Ix>>;

    const BACKEND: &'static str = "GGraph-rs";

    fn set_input(&mut self, v: Self::VertexIx, flag: bool) {
        todo!()
    }

    fn set_output(&mut self, v: Self::VertexIx, flag: bool) {
        todo!()
    }

    fn is_input(&self, v: Self::VertexIx) -> bool {
        todo!()
    }

    fn is_output(&self, v: Self::VertexIx) -> bool {
        todo!()
    }

    fn inputs(&self) -> Self::InputIterator {
        todo!()
    }

    fn outputs(&self) -> Self::OutputIterator {
        todo!()
    }

    fn num_vertices(&self) -> u32 {
        self.g.node_count() as u32
    }

    fn num_edges(&self) -> u32 {
        self.g.edge_count() as u32
    }

    fn vertices(&self) -> Self::VertexIterator {
        self.g.node_indices()
    }

    fn edges(&self) -> Self::EdgeIterator {
        self.g.edge_indices()
    }

    fn vertex(&'a self, v: Self::VertexIx) -> Option<&'a Self::VertexData> {
        self.g.node_weight(v)
    }

    fn vertex_mut(&'a mut self, v: Self::VertexIx) -> Option<&'a mut Self::VertexData> {
        self.g.node_weight_mut(v)
    }

    fn edge(&'a self, e: Self::EdgeIx) -> Option<&'a Self::EdgeData> {
        self.g.edge_weight(e)
    }

    fn edge_mut(&'a mut self, e: Self::EdgeIx) -> Option<&'a mut Self::EdgeData> {
        self.g.edge_weight_mut(e)
    }

    fn edge_endpoints(&self, e: Self::EdgeIx) -> Option<(Self::VertexIx, Self::VertexIx)> {
        self.g.edge_endpoints(e)
    }

    fn vertex_degree(&self, v: Self::VertexIx) -> u32 {
        self.neighbours(v).count() as u32
    }

    fn neighbours(&'a self, v: Self::VertexIx) -> Self::NeighbourIterator {
        self.g.neighbors(v)
    }

    fn connected(&self, v: Self::VertexIx, u: Self::VertexIx) -> bool {
        self.g.contains_edge(v, u)
    }

    fn adjoint(&mut self) {
        todo!()
    }

    fn compose(&mut self, other: Self) {
        todo!()
    }

    fn tensor(&mut self, other: Self) {
        todo!()
    }

    fn add_vertices(&mut self, count: u32) -> Vec<Self::VertexIx> {
        let mut res = Vec::new();
        for _ in 0..count {
            let v = self.g.add_node(Default::default());
            res.push(v);
        }
        res
    }

    fn add_vertex(&'a mut self, kind: VertexKind, phase: Phase) -> Self::VertexIx {
        self.g.add_node(VertexData { kind, phase })
    }

    fn add_edge(&mut self, v: Self::VertexIx, u: Self::VertexIx, kind: EdgeKind) -> Self::EdgeIx {
        self.g.add_edge(u, v, EdgeData { kind })
    }

    fn remove_vertex(&mut self, v: Self::VertexIx) {
        self.g.remove_node(v);
    }

    fn remove_edge(&mut self, e: Self::EdgeIx) {
        self.g.remove_edge(e);
    }
}
