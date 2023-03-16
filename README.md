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

If you do not want to test run the generated flash algorithm, just
`cargo run`. It spits out the flash algo in the probe-rs YAML format.

You can find the generated YAML in `target/definition.yaml`.

If you want to test the flash algorithm, first open `.cargo/config.toml`
and uncomment the `target-gen test` runner. Then `cargo run` will download
the flashing algorithm to a connected target and test run it in addition
to generating the YAML. You will also be able to see RTT messages. Note
that the algorithm and chip will be named `algorithm-test` in the YAML
when using `target-gen test`.

# License

This thingy is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
