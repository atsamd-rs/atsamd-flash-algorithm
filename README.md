# Flash Algorithm for SAME51

This is a CMSIS-Pack flash algorithm for SAME51.
It can be used to generate new flash algoritms for usage with `probe-rs`.

## Dependencies

Run the following requirements:

```bash
cargo install cargo-binutils target-gen
rustup component add llvm-tools-preview
```

## Developing the algorithm

Just run `cargo run`. It spits out the flash algo in the probe-rs YAML format and downloads it onto a target and makes a test run.
You will also be able to see RTT messages.

You can find the generated YAML in `target/definition.yaml`.

# License

This thingy is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
