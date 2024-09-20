use std::collections::BTreeMap;
use flate2::{write::ZlibEncoder, read::ZlibDecoder, Compression};
use std::io::{Read, Write};

#[derive(Clone)]
pub struct VersionedNode {
    keys: BTreeMap<u64, Vec<u8>>, // Key-value pairs with versioning
    children: Vec<Option<Box<VersionedNode>>>,
    is_leaf: bool,
    metadata: NodeMetadata, // Additional metadata for the node
}

#[derive(Clone)]
pub struct NodeMetadata {
    min_timestamp: u64,
    max_timestamp: u64,
}

pub struct VersionedBTree {
    root: Option<Box<VersionedNode>>,
    version: u64,
}

impl VersionedBTree {
    pub fn new() -> Self {
        VersionedBTree {
            root: None,
            version: 0,
        }
    }

    pub fn insert(&mut self, timestamp: u64, value: Vec<u8>) {
        self.version += 1;
        let compressed_value = self.compress(value);
        if let Some(mut root) = self.root.take() {
            self.insert_non_full(&mut root, timestamp, compressed_value);
            self.root = Some(root);
        } else {
            let mut root = VersionedNode {
                keys: BTreeMap::new(),
                children: vec![],
                is_leaf: true,
                metadata: NodeMetadata {
                    min_timestamp: timestamp,
                    max_timestamp: timestamp,
                },
            };
            self.insert_non_full(&mut root, timestamp, compressed_value);
            self.root = Some(Box::new(root));
        }
    }

    fn insert_non_full(&mut self, node: &mut VersionedNode, timestamp: u64, value: Vec<u8>) {
        if node.is_leaf {
            node.keys.insert(timestamp, value);
            node.metadata.min_timestamp = node.metadata.min_timestamp.min(timestamp);
            node.metadata.max_timestamp = node.metadata.max_timestamp.max(timestamp);
        } else {
            let mut i = 0;
            while i < node.keys.len() && timestamp > *node.keys.keys().nth(i).unwrap() {
                i += 1;
            }
            if let Some(child) = &mut node.children[i] {
                self.insert_non_full(child, timestamp, value);
            }
        }
    }

    pub fn search(&self, timestamp: u64) -> Option<Vec<u8>> {
        self.search_node(&self.root, timestamp).map(|v| self.decompress(v.clone()))
    }

    fn search_node<'a>(&'a self, node: &'a Option<Box<VersionedNode>>, timestamp: u64) -> Option<&'a Vec<u8>> {
        if let Some(n) = node {
            if let Some(value) = n.keys.get(&timestamp) {
                return Some(value);
            }
            for child in &n.children {
                if let Some(value) = self.search_node(child, timestamp) {
                    return Some(value);
                }
            }
        }
        None
    }

    pub fn range_query(&self, start: u64, end: u64) -> Vec<(u64, Vec<u8>)> {
        let mut results = Vec::new();
        self.range_query_node(&self.root, start, end, &mut results);
        results.into_iter().map(|(k, v)| (k, self.decompress(v))).collect()
    }

    fn range_query_node(&self, node: &Option<Box<VersionedNode>>, start: u64, end: u64, results: &mut Vec<(u64, Vec<u8>)>) {
        if let Some(n) = node {
            if n.metadata.min_timestamp > end || n.metadata.max_timestamp < start {
                return;
            }
            for (&k, v) in &n.keys {
                if k >= start && k <= end {
                    results.push((k, v.clone()));
                }
            }
            for child in &n.children {
                self.range_query_node(child, start, end, results);
            }
        }
    }

    fn compress(&self, value: Vec<u8>) -> Vec<u8> {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&value).unwrap();
        encoder.finish().unwrap()
    }

    fn decompress(&self, value: Vec<u8>) -> Vec<u8> {
        let mut decoder = ZlibDecoder::new(&value[..]);
        let mut decompressed = Vec::new();
        decoder.read_to_end(&mut decompressed).unwrap();
        decompressed
    }
}