# Data types and structures
This crate covers example code for understanding Rust's main data types and structures

## Building and Running

To build the `data_types_and_structures` crate, navigate to the root of the Cargo workspace and run:

```
$ cargo build --package data_types_and_structures
```

This will compile the crate and its dependencies. To run the crate, use the following command:

```
cargo run --package data_types_and_structures
```

This will execute the main function of the `data_types_and_structures` crate. Alternatively, you can also navigate to the `data_types_and_structures` directory and run the crate using:

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
cargo test --package data_types_and_structures
```

Alternatively, you can also navigate to the `data_types_and_structures` directory (for example) and run the tests using:

```
$ cargo test
```