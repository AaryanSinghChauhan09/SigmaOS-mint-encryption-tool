// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-mint-encryption-tool: mint encryption tool component model.
//!
//! Real implementation: encryptions, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Encryption {
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
pub enum MintEncryptionToolError {
    EmptyEncryptionName,
    DuplicateEncryption(String),
    EmptyEventSubject,
}

impl fmt::Display for MintEncryptionToolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyEncryptionName => write!(f, "encryption name must not be empty"),
            Self::DuplicateEncryption(id) => write!(f, "duplicate encryption {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for MintEncryptionToolError {}

pub struct MintEncryptionTool {
    pub name: String,
    encryptions: BTreeMap<String, Encryption>,
    events: BTreeMap<String, Event>,
}

impl MintEncryptionTool {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            encryptions: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_encryption(&mut self, item: Encryption) -> Result<(), MintEncryptionToolError> {
        if item.name.trim().is_empty() {
            return Err(MintEncryptionToolError::EmptyEncryptionName);
        }
        let id = item.id.clone();
        if self.encryptions.contains_key(&id) {
            return Err(MintEncryptionToolError::DuplicateEncryption(id.clone()));
        }
        self.encryptions.insert(id, item);
        Ok(())
    }

    pub fn encryption_count(&self) -> usize {
        self.encryptions.len()
    }

    pub fn get_encryption(&self, id: &str) -> Option<&Encryption> {
        self.encryptions.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), MintEncryptionToolError> {
        if event.subject.trim().is_empty() {
            return Err(MintEncryptionToolError::EmptyEventSubject);
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

    fn sample_encryption(id: &str, name: &str) -> Encryption {
        Encryption {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = MintEncryptionTool::new("MintEncryptionTool");
        assert_eq!(m.name, "MintEncryptionTool");
        assert_eq!(m.encryption_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_encryption() {
        let mut m = MintEncryptionTool::new("MintEncryptionTool");
        m.add_encryption(sample_encryption("a1", "Alpha")).unwrap();
        assert_eq!(m.encryption_count(), 1);
        assert!(m.get_encryption("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = MintEncryptionTool::new("MintEncryptionTool");
        assert!(m.add_encryption(sample_encryption("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = MintEncryptionTool::new("MintEncryptionTool");
        m.add_encryption(sample_encryption("x", "One")).unwrap();
        assert!(m.add_encryption(sample_encryption("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = MintEncryptionTool::new("MintEncryptionTool");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
