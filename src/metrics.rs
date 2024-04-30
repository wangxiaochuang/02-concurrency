// metrics data structure
// base feature: inc/dec/snapshot

use anyhow::Result;
use dashmap::DashMap;
use std::{fmt, sync::Arc};

#[derive(Debug, Clone)]
pub struct Metrics {
    data: Arc<DashMap<String, i64>>,
}

impl fmt::Display for Metrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for entry in self.data.iter() {
            writeln!(f, "{}: {}", entry.key(), entry.value())?;
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
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_default();
        *counter += 1;
        Ok(())
    }
}
