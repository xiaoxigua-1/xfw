#[allow(unused_imports)]
use parking_lot::RwLock;

use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StatePath(String);

impl StatePath {
    pub fn new(path: &str) -> Self {
        Self(path.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn matches(&self, updated_path: &str) -> bool {
        if self.0 == updated_path {
            return true;
        }
        if updated_path.starts_with(&self.0) {
            let remainder = &updated_path[self.0.len()..];
            return remainder.starts_with('.');
        }
        false
    }
}

impl From<String> for StatePath {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for StatePath {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct NodeId(pub usize);

impl NodeId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }
}

impl From<usize> for NodeId {
    fn from(id: usize) -> Self {
        Self(id)
    }
}

#[derive(Debug, Clone)]
pub struct StateChange {
    pub path: StatePath,
    pub value: serde_json::Value,
}

pub trait StateSubscriber: Send + Sync {
    fn on_state_change(&self, changes: &[StateChange]);
}

pub struct StateRegistry {
    dependencies: HashMap<StatePath, Vec<NodeId>>,
    #[allow(dead_code)]
    subscribers: Vec<Arc<dyn StateSubscriber>>,
}

impl std::fmt::Debug for StateRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StateRegistry")
            .field(
                "dependencies",
                &self.dependencies.keys().collect::<Vec<_>>(),
            )
            .finish()
    }
}

impl Default for StateRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl StateRegistry {
    pub fn new() -> Self {
        Self {
            dependencies: HashMap::new(),
            subscribers: Vec::new(),
        }
    }

    pub fn register(&mut self, path: StatePath, node_id: NodeId) {
        self.dependencies.entry(path).or_default().push(node_id);
    }

    pub fn unregister_node(&mut self, node_id: NodeId) {
        for deps in self.dependencies.values_mut() {
            deps.retain(|id| *id != node_id);
        }
        self.dependencies.retain(|_, v| !v.is_empty());
    }

    pub fn get_affected_nodes(&self, changed_path: &str) -> Vec<NodeId> {
        let mut affected = Vec::new();
        for (path, node_ids) in &self.dependencies {
            if path.matches(changed_path) {
                affected.extend(node_ids.iter().copied());
            }
        }
        affected.sort();
        affected.dedup();
        affected
    }

    pub fn add_subscriber(&mut self, subscriber: Arc<dyn StateSubscriber>) {
        self.subscribers.push(subscriber);
    }

    pub fn notify_subscribers(&self, changes: &[StateChange]) {
        for subscriber in &self.subscribers {
            subscriber.on_state_change(changes);
        }
    }

    pub fn clear(&mut self) {
        self.dependencies.clear();
        self.subscribers.clear();
    }
}
