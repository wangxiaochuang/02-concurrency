// metrics data structure
// base feature: inc/dec/snapshot

use anyhow::{anyhow, Result};
use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, RwLock},
};

#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<RwLock<HashMap<String, i64>>>,
}

impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self.data.read().map_err(|_| fmt::Error {})?;
        for (key, value) in data.iter() {
            write!(f, "{}: {}", key, value)?;
        }

        Ok(())
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut data = self.data.write().map_err(|e| anyhow!(e.to_string()))?;
        let counter = data.entry(key.into()).or_default();
        *counter += 1;
        Ok(())
    }

    pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
        self.data
            .read()
            .map(|data| data.clone())
            .map_err(|e| anyhow!(e.to_string()))
    }
}
