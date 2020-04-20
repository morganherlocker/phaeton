use phaeton::graph::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let arg = std::env::args_os()
        .nth(1)
        .ok_or("need a *.osm.pbf file as argument")?;

    let mut graph = Graph::new();

    graph.read_pbf(&arg)?;

    Ok(())
}
