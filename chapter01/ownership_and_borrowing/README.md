# Ownership and borrowing
This crate covers example code for understanding Rust ownership and borrowing.

## Building and Running

To build the `ownership_and_borrowing` crate, navigate to the root of the Cargo workspace and run:

```
$ cargo build --package ownership_and_borrowing
```

This will compile the crate and its dependencies. To run the crate, use the following command:

```
cargo run --package ownership_and_borrowing
```

This will execute the main function of the `ownership_and_borrowing` crate. Alternatively, you can also navigate to the `ownership_and_borrowing` directory (for example) and run the crate using:

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
cargo test --package ownership_and_borrowing 
```

Alternatively, you can also navigate to the `ownership_and_borrowing` directory (for example) and run the tests using:

```
$ cargo test
```