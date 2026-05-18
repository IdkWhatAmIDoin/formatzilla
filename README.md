# AGP

ever struggled with the pain of {:?} on hashmaps? sure, i guess {:#?} is a bit better, but all it does is just not make 
it a one-line blob.

well, Actual Good Printing has come to save you!

## usage

```rust
use std::collections::HashMap;
use agp::ActualGoodPrinting;

fn main() {
    let mut map = HashMap::new();
    map.insert("status", "ok");
    map.insert("code", "200");

    map.agp();
    // output (order may vary):
    // status: ok
    // code: 200
}
```

to return it, do the same but swap `.agp()` for `.return_agp()`:

```rust
use std::collections::HashMap;
use agp::ActualGoodPrinting;

fn main() {
    let mut map = HashMap::new();
    map.insert("status", "ok");
    map.insert("code", "200");

    let good_map = map.return_agp();
    println!("{good_map}");
}
```

## installation

```toml
[dependencies]
agp = "0"
```