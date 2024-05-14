# Concurrency and multithreading
This crate covers example code for understanding concurrency and multithreading in Rust.

## Building and Running

To build the `concurrency_and_multi_threading` crate, navigate to the root of the Cargo workspace and run:

```
$ cargo build --package concurrency_and_multi_threading
```

This will compile the crate and its dependencies. To run the crate, use the following command:

```
cargo run --package concurrency_and_multi_threading
```

This will execute the main function of the `concurrency_and_multi_threading` crate. Alternatively, you can also navigate to the `concurrency_and_multi_threading` directory and run the crate using:

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
cargo test --package concurrency_and_multi_threading
```

Alternatively, you can also navigate to the `concurrency_and_multi_threading` directory (for example) and run the tests using:

```
$ cargo t