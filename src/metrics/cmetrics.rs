use anyhow::{Ok, Result};
use dashmap::DashMap;
use std::{fmt, sync::Arc};

#[derive(Debug, Clone)]
pub struct CMetrics {
    data: Arc<DashMap<String, i64>>,
}

impl CMetrics {
    pub fn new() -> Self {
        Self {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }

    pub fn dec(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter -= 1;
        Ok(())
    }

    pub fn snapshot(&self) -> Result<Arc<DashMap<String, i64>>> {
        Ok(self.data.clone())
    }
}

impl Default for CMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for CMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for entry in self.data.iter() {
            write!(f, "{}: {}", entry.key(), entry.value())?;
        }
        fmt::Result::Ok(())
    }
}
