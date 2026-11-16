// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-buffer-cache: buffer cache component model.
//!
//! Real implementation: caches, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cache {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BufferCacheError {
    EmptyCacheName,
    DuplicateCache(String),
    EmptyEventSubject,
}

impl fmt::Display for BufferCacheError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyCacheName => write!(f, "cache name must not be empty"),
            Self::DuplicateCache(id) => write!(f, "duplicate cache {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for BufferCacheError {}

pub struct BufferCache {
    pub name: String,
    caches: BTreeMap<String, Cache>,
    events: BTreeMap<String, Event>,
}

impl BufferCache {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            caches: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_cache(&mut self, item: Cache) -> Result<(), BufferCacheError> {
        if item.name.trim().is_empty() {
            return Err(BufferCacheError::EmptyCacheName);
        }
        let id = item.id.clone();
        if self.caches.contains_key(&id) {
            return Err(BufferCacheError::DuplicateCache(id.clone()));
        }
        self.caches.insert(id, item);
        Ok(())
    }

    pub fn cache_count(&self) -> usize {
        self.caches.len()
    }

    pub fn get_cache(&self, id: &str) -> Option<&Cache> {
        self.caches.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), BufferCacheError> {
        if event.subject.trim().is_empty() {
            return Err(BufferCacheError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_cache(id: &str, name: &str) -> Cache {
        Cache {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = BufferCache::new("BufferCache");
        assert_eq!(m.name, "BufferCache");
        assert_eq!(m.cache_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_cache() {
        let mut m = BufferCache::new("BufferCache");
        m.add_cache(sample_cache("a1", "Alpha")).unwrap();
        assert_eq!(m.cache_count(), 1);
        assert!(m.get_cache("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = BufferCache::new("BufferCache");
        assert!(m.add_cache(sample_cache("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = BufferCache::new("BufferCache");
        m.add_cache(sample_cache("x", "One")).unwrap();
        assert!(m.add_cache(sample_cache("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = BufferCache::new("BufferCache");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
