# did

Implements a parser for decentralized identifiers. Uses
[pest](https://github.com/pest-parser/pest) for now to define the grammar of
the generic scheme defined in the
[specification](https://w3c-ccg.github.io/did-spec/#generic-did-syntax). This
will be changed in the future to something more performant like
[nom](https://github.com/Geal/nom).

## Usage

```rust
use did_rs::DID;

fn main() {
    let d = DID::parse("did:example:").expect("failed");
    println!("{}", d);
}
```
