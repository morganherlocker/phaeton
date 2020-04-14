extern crate osmpbf;

use osmpbf::{Element, ElementReader};
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
struct Coordinate {
    lon: f64,
    lat: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let arg = std::env::args_os()
        .nth(1)
        .ok_or("need a *.osm.pbf file as argument")?;
    let reader = ElementReader::from_path(&arg)?;

    let mut wayCount = 0;
    let mut nodeCount = 0;
    let mut nodes = HashMap::new();

    reader.for_each(|element| {
        match element {
            Element::Way(_way) => wayCount += 1,
            Element::Node(node) => {
                let id = node.id();
                let coordinate = Coordinate {
                    lon: node.lon(),
                    lat: node.lat(),
                };

                nodes.insert(id, coordinate);

                nodeCount += 1;
            }
            Element::DenseNode(node) => {
                let id = node.id;
                let coordinate = Coordinate {
                    lon: node.lon(),
                    lat: node.lat(),
                };

                nodes.insert(id, coordinate);
                println!("{:?}", nodes.get(&id));
                nodeCount += 1;
            }
            Element::Relation(_relation) => {} // should not occur
        }
    })?;

    // Print result
    println!("ways:  {}\nnodes: {}", wayCount, nodeCount);
    Ok(())
}
