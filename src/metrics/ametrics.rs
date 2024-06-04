use anyhow::Result;
use std::{
    collections::HashMap,
    fmt,
    sync::{
        atomic::{AtomicI64, Ordering},
        Arc,
    },
};

#[derive(Debug)]
pub struct AMetrics {
    data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl AMetrics {
    pub fn new(metrics_names: &[&'static str]) -> Self {
        let map = metrics_names
            .iter()
            .map(|&name| (name, AtomicI64::new(0)))
            .collect();
        Self {
            data: Arc::new(map),
        }
    }

    pub fn inc(&self, key: impl AsRef<str>) -> Result<()> {
        let key = key.as_ref();
        let counter = self
            .data
            .get(key)
            .ok_or_else(|| anyhow::anyhow!("key {} not found", key))?;
        counter.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}

impl Clone for AMetrics {
    fn clone(&self) -> Self {
        AMetrics {
            data: Arc::clone(&self.data),
        }
    }
}

impl fmt::Display for AMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (key, value) in self.data.iter() {
            writeln!(f, "{} :{}", key, value.load(Ordering::Relaxed))?;
        }
        Ok(())
    }
}
