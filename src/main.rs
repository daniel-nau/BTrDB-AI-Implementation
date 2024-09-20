mod storage;

use storage::VersionedBTree;

fn main() {
    let mut btree = VersionedBTree::new();

    // Insert some data
    btree.insert(10, vec![1, 2, 3]);
    btree.insert(20, vec![4, 5, 6]);
    btree.insert(5, vec![7, 8, 9]);
    btree.insert(15, vec![10, 11, 12]);
    btree.insert(25, vec![13, 14, 15]);

    // Search for existing keys
    if let Some(value) = btree.search(10) {
        println!("Found value for key 10: {:?}", value);
    } else {
        println!("Key 10 not found");
    }

    if let Some(value) = btree.search(20) {
        println!("Found value for key 20: {:?}", value);
    } else {
        println!("Key 20 not found");
    }

    // Search for a non-existing key
    if let Some(value) = btree.search(15) {
        println!("Found value for key 15: {:?}", value);
    } else {
        println!("Key 15 not found");
    }

    // Search for newly inserted keys
    if let Some(value) = btree.search(25) {
        println!("Found value for key 25: {:?}", value);
    } else {
        println!("Key 25 not found");
    }

    // Search for a key that was not inserted
    if let Some(value) = btree.search(30) {
        println!("Found value for key 30: {:?}", value);
    } else {
        println!("Key 30 not found");
    }

    // Range query example
    let range_results = btree.range_query(5, 20);
    for (timestamp, value) in range_results {
        println!("Found value for timestamp {}: {:?}", timestamp, value);
    }
}