# Chapter 1

This Cargo workspace contains multiple crates, corresponding to the main sections in Chapter 1. 

## Requirements

To run the crates in this workspace, you'll need to have Rust installed. The recommended way to install Rust is using [Rustup](https://rustup.rs/), the Rust toolchain installer.

You can install Rust 2021 edition using Rustup by running the following command:

```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This will install the latest stable version of Rust.

## Workspace configuration
The workspace is configured in the root-level `Cargo.toml` file like so:

```
[workspace]
members = [ 
    "data_types_and_structures",
    "ownership_and_borrowing",
    "pattern_matching_and_control_flow",
    "error_handling", 
    "concurrency_and_multi_threading"
]
```

The `members` key lists all the crates that are part of this workspace. 

## Building the workspace
To build all crates in the workspace, simply run the following command from the root directory of the workspace:

```
$ cargo build
```

This will compile all the crates defined in the workspace's `Cargo.toml` file.

## Building individual crates
If you want to build a specific crate within the workspace, you can use the `--package` (or `-p`) flag:

```
$ cargo build --package ownership_and_borrowing
```

## Tests
Each individual crate has tests written, providing further examples of the concepts we've covered per section. You can run all tests from the root workspace with:

```
$ cargo test
```

## Testing individual crates
Similar to building individual crates, you may execute individual crate tests from the root workspace with:

```
cargo test --package ownership_and_borrowing
```

Alternatively, you can also navigate to the `ownership_and_borrowing` directory (for example) and run the tests using:

```
$ cargo test
```