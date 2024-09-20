# Versioned B-tree Timeseries Storage

This project implements a versioned B-tree data structure in Rust, designed to store key-value pairs with versioning capabilities. Each node in the B-tree contains a `BTreeMap` of keys and values, where the keys are timestamps and the values are vectors of bytes. The `VersionedBTree` struct supports operations such as insertion, search, and range queries, with the ability to compress and decompress values using the `flate2` crate. The `NodeMetadata` struct stores additional information about each node, such as the minimum and maximum timestamps. The main function demonstrates the usage of the `VersionedBTree` by inserting key-value pairs, searching for specific keys, and performing range queries.

## Project Structure

- `src/main.rs`: Entry point of the application. Demonstrates the usage of the `VersionedBTree` by inserting key-value pairs, searching for specific keys, and performing range queries.
- `src/lib.rs`: Defines the library interface, exporting the main functionalities of the `VersionedBTree` implementation.
- `src/storage/mod.rs`: Module for the storage system, re-exporting components from `versioned_btree.rs`.
- `src/storage/versioned_btree.rs`: Implements the versioned B-tree data structure optimized for timeseries data. Exports the `VersionedBTree` struct with methods such as `insert`, `search`, and `range_query`.
- `src/storage/btree.rs`: Implements the B-tree data structure optimized for timeseries data. Exports the `BTree` struct with methods such as `insert`, `delete`, and `search`.
- `src/timeseries/mod.rs`: Module for timeseries processing, re-exporting components from `processor.rs`.
- `src/timeseries/processor.rs`: Contains the `TimeseriesProcessor` struct with methods like `process_data` and `query_data` for handling timeseries data.
- `src/utils/mod.rs`: Contains utility functions and types that support the main functionalities of the project.

## Setup Instructions

1. Clone the repository:
   ```
   git clone <repository-url>
   ```

2. Navigate to the project directory:
   ```
   cd btrdb-timeseries
   ```

3. Build the project using Cargo:
   ```
   cargo build
   ```

4. Run the application:
   ```
   cargo run
   ```

## Usage Examples

- To insert data into the B-tree, use the `insert` method from the `BTree` struct.
- To process timeseries data, utilize the `process_data` method from the `TimeseriesProcessor` struct.

## Contributing

Contributions are welcome! Please submit a pull request or open an issue for any enhancements or bug fixes.