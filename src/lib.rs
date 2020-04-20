/*!
A routing engine for multimodal logistics planning.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
phaeton = "0.1.0"
```

## Example

```rust,no_run
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
```
*/

pub use graph::*;

/// Graph is the central data structure of phaeton. It exposes the nodes, edges, and metadata of the road network, as well as methods for analyzing the network.
pub mod graph;
