#[derive (Clone)]
pub struct DataPoint {
    timestamp: u32,
    value: u32
}

impl DataPoint {
    pub fn new(timestamp: u32, value: u32) -> DataPoint {
        DataPoint { timestamp, value }
    }
}

pub struct PriceDB{
    db: Vec<DataPoint>
}

impl PriceDB {
    pub fn new() -> PriceDB {
        let db = Vec::new();
        PriceDB { db }
    }

    pub fn insert(&mut self, dp: DataPoint) {
        eprintln!("inserting value: {}, at timestamp {}", dp.value, dp.timestamp);

        if self.db.len() == 0 {
            self.db.push(dp);
            return
        }

        if self.db.len() == 1 {
            let existing = self.db.get(0).unwrap();
            if existing.timestamp > dp.timestamp {
                self.db.insert(0, dp);
            } else {
                self.db.push(dp)
            }
            return;
        }

        for i in 0..self.db.len()-1 {
            let p1 = self.db.get(i).unwrap();
            let p2 = self.db.get(i+1).unwrap();
            if p1.timestamp < dp.timestamp && p2.timestamp > dp.timestamp {
                self.db.insert(i, dp.clone());
                return
            }
            self.db.push(dp.clone());
        }

    }

    pub fn query(&self,  t1: u32, t2: u32) -> f64 {
        let prices = &self.db;

        if prices.len() == 1 {
            return prices[0].value as f64;
        }
        
        // Initialize with sentinel values in case the conditions are never met
        let mut i1 = 0;
        let mut i2 = prices.len() - 1;
        
        // Find i1 such that prices[i1].timestamp is the first point >= t1
        // Find i2 such that prices[i2].timestamp is the last point <= t2
        for i in 0..prices.len() {
            if prices[i].timestamp >= t1 {
                i1 = i;
                break;
            }
        }
        
        for i in (0..prices.len()).rev() {
            if prices[i].timestamp <= t2 {
                i2 = i;
                break;
            }
        }
        
        if i1 > i2 {
            return 0.0; // or handle this case however appropriate
        }
        
        // Now include the full inclusive range
        let mut sum = 0;
        for dp in &prices[i1..=i2] {
            eprintln!("adding {} to the total. sum is now {}", dp.value, sum);
            sum += dp.value;
        }
        

        sum as f64 / prices[i1..i2].len() as f64
    }
}

