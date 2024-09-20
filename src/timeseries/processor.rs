pub struct TimeseriesProcessor {
    data: Vec<(i64, f64)>, // Example field to store timeseries data
}

impl TimeseriesProcessor {
    pub fn new() -> Self {
        TimeseriesProcessor {
            data: Vec::new(), // Initialize with an empty vector
        }
    }

    pub fn insert_data(&mut self, timestamp: i64, value: f64) {
        self.data.push((timestamp, value));
    }

    pub fn query_data(&self, start: i64, end: i64) -> Vec<(i64, f64)> {
        // Filter the data based on the start and end timestamps
        self.data.iter()
            .filter(|&&(timestamp, _)| timestamp >= start && timestamp <= end)
            .cloned()
            .collect()
    }
}