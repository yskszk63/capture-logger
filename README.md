# capture-logger

[`log`](https://docs.rs/log/0.4.11/log/) implementation for testing.

### Dependencies

```toml
[dev-dependencies]
capture-logger = "0.1"
```

### Example

```rust
use capture_logger::{begin_capture, pop_captured};

begin_capture();
log::debug!("LOG");
assert_eq!(pop_captured().unwrap().message(), "LOG");
```

### License

Licensed under either of
* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.!

License: MIT OR Apache-2.0
