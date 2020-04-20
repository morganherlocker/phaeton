use osmpbf::{Element, IndexedReader};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

/// A graph vertex representing a geometric point
#[derive(Debug, Serialize, Deserialize)]
pub struct Vertex {
    /// Graph identifier, used for relational joins
    pub id: i64,
    /// Geographic longitude (datum: WGS 84, also known as EPSG:4326)
    pub lon: f32,
    /// Geographic latitude (datum: WGS 84, also known as EPSG:4326)
    pub lat: f32,
}

/// A graph node representing link between 2 or more edges
#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    /// Graph identifier, used for relational joins
    pub id: i64,
    /// List of IDs corresponding to connected graph edges
    pub edges: Vec<i64>,
}

/// A graph edge, representing a connection between intersections
#[derive(Debug, Serialize, Deserialize)]
pub struct Edge {
    /// Graph identifier, used for relational joins
    pub id: i64,
    /// List of IDs corresponding to graph vertices
    pub vertices: Vec<i64>,
}

/// A key value metadata item
#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub key: String,
    pub value: String,
}

/// Core graph data structure
#[derive(Debug, Serialize, Deserialize)]
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

    /// Parse an OpenStreetMap PBF file and represent as graph
    ///
    /// # Example
    ///
    /// ```
    /// use phaeton::graph::*;
    /// use std::error::Error;
    ///
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let mut graph = Graph::new();
    ///
    ///     graph.read_pbf(&std::ffi::OsString::from("./honolulu.osm.pbf"))?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn read_pbf(&mut self, arg: &std::ffi::OsString) -> Result<(), Box<dyn Error>> {
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
                    let id = way.id();
                    let edge = Edge {
                        id: id,
                        vertices: way.refs().collect(),
                    };
                    self.edges.push(edge);

                    let mut tags = Vec::new();
                    for (key, value) in way.tags() {
                        let tag = Tag {
                            key: key.to_string(),
                            value: value.to_string(),
                        };
                        tags.push(tag);
                    }
                    self.metadata.insert(id, tags);
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

    /// Write graph to CBOR format
    ///
    /// # Example
    ///
    /// ```
    /// use phaeton::graph::*;
    /// use std::error::Error;
    ///
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let mut graph = Graph::new();
    ///     graph.read_pbf(&std::ffi::OsString::from("./honolulu.osm.pbf"))?;
    ///
    ///     graph.write_cbor(&std::ffi::OsString::from("./honolulu-write.cbor"))?;
    ///
    ///     std::fs::remove_file(&std::ffi::OsString::from("./honolulu-write.cbor"))?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn write_cbor(&self, filepath: &std::ffi::OsString) -> Result<(), Box<dyn Error>> {
        let file = File::create(filepath)?;

        serde_cbor::to_writer(file, &self)?;

        Ok(())
    }

    /// Read graph from CBOR format
    ///
    /// # Example
    ///
    /// ```
    /// use phaeton::graph::*;
    /// use std::error::Error;
    ///
    /// fn main() -> Result<(), Box<dyn Error>> {
    ///     let mut graph = Graph::new();
    ///     graph.read_pbf(&std::ffi::OsString::from("./honolulu.osm.pbf"))?;
    ///     graph.write_cbor(&std::ffi::OsString::from("./honolulu-read.cbor"))?;
    ///
    ///     graph.read_cbor(&std::ffi::OsString::from("./honolulu-read.cbor"))?;
    ///
    ///     std::fs::remove_file(&std::ffi::OsString::from("./honolulu-read.cbor"))?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn read_cbor(&mut self, filepath: &std::ffi::OsString) -> Result<(), Box<dyn Error>> {
        let file = File::open(filepath)?;

        let graph: Graph = serde_cbor::from_reader(file)?;
        self.vertices = graph.vertices;
        self.edges = graph.edges;
        self.metadata = graph.metadata;

        Ok(())
    }
}
