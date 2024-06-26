# Error handling
This crate covers example code for understanding error handling in Rust.

## Building and Running

To build the `error_handling` crate, navigate to the root of the Cargo workspace and run:

```
$ cargo build --package error_handling
```

This will compile the crate and its dependencies. To run the crate, use the following command:

```
cargo run --package error_handling
```

This will execute the main function of the `error_handling` crate. Alternatively, you can also navigate to the `error_handling` directory and run the crate using:

```
$ cargo run
```

## Tests
Each individual crate has tests written, providing further examples of the concepts we've covered per section. You can run all tests from the root workspace with:

```
$ cargo test
```

You may execute individual crate tests from the root workspace with:

```
cargo test --package error_handling
```

Alternatively, you can also navigate to the `error_handling` directory (for example) and run the tests using:

```
$ cargo test
```