mod storage;
mod timeseries;

use storage::BTree;
use timeseries::TimeseriesProcessor;

fn main() {
    let mut btree = BTree::new();
    btree.insert(10, vec![]);
    btree.insert(20, vec![]);
    btree.insert(5, vec![]);

    println!("Search for 10: {}", btree.search(10)); // Should print true
    println!("Search for 15: {}", btree.search(15)); // Should print false

    let processor = TimeseriesProcessor::new();
    let data = processor.query_data(0, 10);
    println!("Queried data: {:?}", data); // Should print an empty vector
}