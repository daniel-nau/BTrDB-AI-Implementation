# BTrDB Timeseries

BTrDB (Binary Time Series Database) is a specialized storage system designed for efficient processing and storage of time series data. This project implements a B-tree structure optimized for timeseries processing.

## Project Structure

- `src/main.rs`: Entry point of the application. Initializes the BTrDB tree structure and starts the timeseries processing.
- `src/lib.rs`: Defines the library interface, exporting the main functionalities of the BTrDB implementation.
- `src/storage/mod.rs`: Module for the storage system, re-exporting components from `btree.rs`.
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