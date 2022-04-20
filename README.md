# rust-rampart

Rust implementation of the [Haskell Rampart library](https://github.com/tfausak/rampart) by [Taylor Fausak](https://taylor.fausak.me/2020/03/13/relate-intervals-with-rampart).

## Examples

```rust
let a = Interval::new(2, 3);
let b = Interval::new(3, 7);
let rel = a.relate(&b); 
# Relation::Meets
```

![][interval relations]

[interval relations]: ./docs/interval-relations.svg
