use counter::Counter;
use fraction::Fraction;
use itertools::Itertools;

/// Phase of a node, represented as a fraction of pi
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct Phase {
    f: Fraction,
}

impl Default for Phase {
    fn default() -> Self {
        Phase { f: 0.into() }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VertexKind {
    Z,
    X,
    Boundary,
    HBox,
}

impl Default for VertexKind {
    fn default() -> Self {
        VertexKind::Boundary
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeKind {
    Regular,
    Hadam,
}

impl Default for EdgeKind {
    fn default() -> Self {
        EdgeKind::Regular
    }
}

/// Attributes of a graph's vertex
pub trait VData {
    fn phase(&self) -> Phase;
    fn phase_mut(&mut self) -> &mut Phase;
    fn kind(&self) -> VertexKind;
    fn kind_mut(&mut self) -> &mut VertexKind;
}

/// Attributes of a graph's edge
///
/// Two equivalent undirected edges will have the same source and the same target.
pub trait EData {
    fn kind(&self) -> EdgeKind;
    fn kind_mut(&mut self) -> &mut EdgeKind;
}

/// Undirected open graph representation.
///
/// Both vertices and edges have an specified kind and associated data.
/// Vertices can be marked as inputs and outputs of the graph.
pub trait Graph<'a> {
    type VertexIx: Copy;
    type EdgeIx: Copy;
    type VertexData: VData;
    type EdgeData: EData;
    type EdgeIterator: Iterator<Item = Self::EdgeIx>;
    type VertexIterator: Iterator<Item = Self::VertexIx>;
    type NeighbourIterator: Iterator<Item = Self::VertexIx>;
    type InputIterator: Iterator<Item = Self::EdgeIx>;
    type OutputIterator: Iterator<Item = Self::VertexIx>;

    /// Name of the graph backend
    const BACKEND: &'static str;

    fn set_input(&mut self, v: Self::VertexIx, flag: bool);
    fn set_output(&mut self, v: Self::VertexIx, flag: bool);
    fn is_input(&self, v: Self::VertexIx) -> bool;
    fn is_output(&self, v: Self::VertexIx) -> bool;
    fn inputs(&self) -> Self::InputIterator;
    fn outputs(&self) -> Self::OutputIterator;

    /// Vertex count of the graph
    fn num_vertices(&self) -> u32;
    /// Edge count of the graph
    fn num_edges(&self) -> u32;

    /// Set of vertices of the graph
    fn vertices(&self) -> Self::VertexIterator;

    /// Set of edges of the graph
    fn edges(&self) -> Self::EdgeIterator;

    /// Vertex data
    fn vertex(&'a self, v: Self::VertexIx) -> Option<&'a Self::VertexData>;

    /// Vertex data
    fn vertex_mut(&'a mut self, v: Self::VertexIx) -> Option<&'a mut Self::VertexData>;

    /// Edge data
    fn edge(&'a self, e: Self::EdgeIx) -> Option<&'a Self::EdgeData>;

    /// Edge data
    fn edge_mut(&'a mut self, e: Self::EdgeIx) -> Option<&'a mut Self::EdgeData>;

    fn edge_endpoints(&self, e: Self::EdgeIx) -> Option<(Self::VertexIx, Self::VertexIx)>;

    /// Number of neighbours of a vertex
    fn vertex_degree(&self, v: Self::VertexIx) -> u32;

    /// Neighbours of a vertex
    fn neighbours(&'a self, v: Self::VertexIx) -> Self::NeighbourIterator;

    /// Check if two nodes are connected by any type of edge
    fn connected(&self, v: Self::VertexIx, u: Self::VertexIx) -> bool;

    /// Transform the diagram into its adjoint
    fn adjoint(&mut self);

    /// Append another graph after this one, connecting their inputs/outputs.
    fn compose(&mut self, other: Self);

    /// Parallel compose another graph, adding its inputs and outputs to the current ones.
    fn tensor(&mut self, other: Self);

    /// Add multiple unconnected vertices
    fn add_vertices(&mut self, count: u32) -> Vec<Self::VertexIx>;

    /// Add a single vertex
    fn add_vertex(&'a mut self, kind: VertexKind, phase: Phase) -> Self::VertexIx {
        let v = self.add_vertices(1)[0];
        let vdata: &'a mut _ = self.vertex_mut(v).unwrap();
        *vdata.kind_mut() = kind;
        *vdata.phase_mut() = phase;
        v
    }

    /// Add a single edge between vertices
    fn add_edge(&mut self, v: Self::VertexIx, u: Self::VertexIx, ty: EdgeKind) -> Self::EdgeIx;

    /// Remove multiple vertices
    fn remove_vertices(&mut self, vs: impl IntoIterator<Item = Self::VertexIx>) {
        for v in vs {self.remove_vertex(v)}
    }

    /// Remove multiple vertices
    fn remove_vertex(&mut self, v: Self::VertexIx);

    /// Remove multiple edges from the graph
    fn remove_edges(&mut self, es: impl IntoIterator<Item = Self::EdgeIx>) {
        for e in es {self.remove_edge(e)}
    }

    /// Remove a single edge from the graph
    fn remove_edge(&mut self, e: Self::EdgeIx);

    // Utility functions

    /// Returns a string with some information regarding the degree distribution of the graph.
    fn stats(&self) -> String {
        let degrees: Counter<u32> = self.vertices().map(|v| self.vertex_degree(v)).collect();
        let degrees_s: String = degrees
            .iter()
            .sorted()
            .map(|(k, v)| format!("  {}: {}\n", k, v))
            .collect();
        let header = format!(
            "Graph(backend={}, num_vertices={}, num_edges={})\n",
            Self::BACKEND,
            self.num_vertices(),
            self.num_edges()
        );
        header + "degree distribution: \n" + &degrees_s
    }
}

/// A graph with added qubit and row information for each vertex
pub trait CartesianGraph<'a>: Graph<'a> {
    /// Qubit identifier for vertices in a CartesianGraph
    type Qubit;
    /// Row identifier for vertices in a CartesianGraph
    type Row;

    /// Row of a vertex
    fn row(&self, v: Self::VertexIx) -> Self::Row;
    /// Qubit of a vertex
    fn qubit(&self, v: Self::VertexIx) -> Self::Qubit;

    /// Set the row of a vertex
    fn set_row(&mut self, v: Self::VertexIx, r: Self::Row);
    /// Set the qubit of a vertex
    fn set_qubit(&mut self, v: Self::VertexIx, q: Self::Qubit);
    /// Set both the row and qubit of a vertex
    fn set_position(&mut self, v: Self::VertexIx, q: Self::Qubit, r: Self::Row);

    /// Number of rows in the graph
    fn depth(&self) -> u32;
    /// Number of qubits in the graph
    fn qubit_count(&self) -> u32;
}

/// Attributes of a grounded graph's edge
///
/// Two equivalent undirected edges will have the same source and the same target.
pub trait GroundData {
    fn ground(&self) -> bool;
    fn ground_mut(&mut self) -> &mut bool;
}

/// Graph with measurement operators
pub trait GroundGraph<'a>: Graph<'a>
where
    Self::VertexData: GroundData,
{
    type GroundIterator: Iterator<Item = Self::VertexIx>;

    /// Return an iterator over the ground vertex indices
    fn grounds(&self) -> Self::GroundIterator;
}
