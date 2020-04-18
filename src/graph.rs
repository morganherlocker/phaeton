use osmpbf::{Element, IndexedReader};
use std::collections::HashMap;
use std::error::Error;

/// A graph vertex representing a geometric point
pub struct Vertex {
    /// Graph identifier, used for relational joins
    pub id: i64,
    /// Geographic longitude (datum: WGS 84, also known as EPSG:4326)
    pub lon: f32,
    /// Geographic latitude (datum: WGS 84, also known as EPSG:4326)
    pub lat: f32,
}

/// A graph edge, representing a connection between intersections
pub struct Edge {
    /// Graph identifier, used for relational joins
    pub id: i64,
    /// List of IDs corresponding to graph vertices
    pub vertices: Vec<i64>,
}

/// A key value metadata item
pub struct Tag {
    pub key: String,
    pub value: String,
}

/// Core graph data structure
pub struct Graph {
    pub vertices: HashMap<i64, Vertex>,
    pub edges: Vec<Edge>,
    pub metadata: HashMap<i64, Vec<Tag>>,
}

impl Graph {
    /// Constructs a new Graph
    ///
    /// # Example
    ///
    /// ```
    /// use phaeton::graph::*;
    /// use std::error::Error;
    ///
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let arg = std::env::args_os()
    ///         .nth(1)
    ///         .ok_or("need a *.osm.pbf file as argument")?;
    ///     let mut reader = IndexedReader::from_path(&arg)?;
    ///
    ///     let mut graph = Graph::new();
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn new() -> Graph {
        Graph {
            vertices: HashMap::new(),
            edges: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Constructs a new Graph
    ///
    /// # Example
    ///
    /// ```
    /// use phaeton::graph::*;
    /// use std::error::Error;
    ///
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let mut reader = IndexedReader::from_path("./honolulu.osm.pbf")?;
    ///
    ///     let mut graph = Graph::new();
    ///
    ///     graph.load_pbf(&arg)?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn load_pbf(&mut self, arg: &std::ffi::OsString) -> Result<(), Box<dyn Error>> {
        let mut reader = IndexedReader::from_path(&arg)?;

        reader.read_ways_and_deps(
            |way| {
                way.tags().any(|key_value| match key_value.0 {
                    "highway" => true,
                    _ => false,
                })
            },
            |element| match element {
                Element::Way(way) => {
                    let edge = Edge {
                        id: way.id(),
                        vertices: way.refs().collect(),
                    };
                    self.edges.push(edge);
                }
                Element::Node(node) => {
                    let vertex = Vertex {
                        id: node.id(),
                        lon: node.lon() as f32,
                        lat: node.lat() as f32,
                    };
                    self.vertices.insert(vertex.id, vertex);
                }
                Element::DenseNode(node) => {
                    let vertex = Vertex {
                        id: node.id,
                        lon: node.lon() as f32,
                        lat: node.lat() as f32,
                    };
                    self.vertices.insert(vertex.id, vertex);
                }
                Element::Relation(_) => {}
            },
        )?;

        Ok(())
    }
}
