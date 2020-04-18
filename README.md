# phaeton

> _"There have been, and will be again, many destructions of mankind arising out of many causes; the greatest have been brought about by the agencies of fire and water, and other lesser ones by innumerable other causes. There is a story that even you have preserved, that once upon a time, Phaeton, the son of Helios, having yoked the steeds in his father's chariot, because he was not able to drive them in the path of his father, burnt up all that was upon the earth, and was himself destroyed by a thunderbolt. Now, this has the form of a myth, but really signifies a declination of the bodies moving in the heavens around the earth, and a great conflagration of things upon the earth, which recurs after long intervals."_

> _- **Plato**, Timaeus, c. 360 BC_

![](https://i.imgur.com/f3LKXam.png)


---

## use

### cli

```sh
phaeton [cmd] [opts]
```

### lib

```rust
use phaeton::graph::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
  let mut graph = Graph::new();

  graph.load_pbf(&std::ffi::OsString::from("./honolulu.osm.pbf"))?;

  Ok(())
}
```

## install

### bin

```sh
cargo install phaeton
```

### lib

Add this to your `Cargo.toml`:

```toml
[dependencies]
osmpbf = "0.1"
```
