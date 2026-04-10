use std::path::PathBuf;
use core_types::workspace::DirectoryEntry;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ExplorerNode {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub children: Vec<ExplorerNode>,
}

impl ExplorerNode {
    pub fn new(entry: &DirectoryEntry) -> Self {
        Self {
            path: PathBuf::from(&entry.path),
            name: entry.name.clone(),
            is_dir: entry.is_dir,
            children: Vec::new(),
        }
    }
}

pub fn build_explorer_tree(entries: &[DirectoryEntry]) -> Vec<ExplorerNode> {
    if entries.is_empty() {
        return Vec::new();
    }
    
    // First, sort all entries: directories first, then files, alphabetically
    let mut sorted_entries: Vec<&DirectoryEntry> = entries.iter().collect();
    sorted_entries.sort_by(|a, b| {
        if a.is_dir != b.is_dir {
            b.is_dir.cmp(&a.is_dir) // Directories first
        } else {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        }
    });
    
    // Build a map from path string to ExplorerNode
    let mut node_map: HashMap<String, ExplorerNode> = HashMap::new();
    
    // First pass: create all nodes without children
    for entry in sorted_entries {
        let path_str = entry.path.clone();
        node_map.insert(path_str.clone(), ExplorerNode::new(entry));
    }
    
    // Second pass: organize nodes into a tree
    // We need to collect paths first to avoid borrowing issues
    let paths: Vec<String> = node_map.keys().cloned().collect();
    
    for path in paths {
        if let Some(node) = node_map.get_mut(&path) {
            // Find parent path
            if let Some(parent_path) = node.path.parent() {
                let parent_str = parent_path.to_string_lossy().to_string();
                if let Some(parent_node) = node_map.get_mut(&parent_str) {
                    // Add this node as a child of its parent
                    parent_node.children.push(node.clone());
                }
            }
        }
    }
    
    // Third pass: sort children of each node
    for node in node_map.values_mut() {
        node.children.sort_by(|a, b| {
            if a.is_dir != b.is_dir {
                b.is_dir.cmp(&a.is_dir) // Directories first
            } else {
                a.name.to_lowercase().cmp(&b.name.to_lowercase())
            }
        });
    }
    
    // Collect root nodes (nodes without parents in the entries)
    let mut root_nodes = Vec::new();
    for node in node_map.values() {
        let has_parent_in_entries = if let Some(parent_path) = node.path.parent() {
            let parent_str = parent_path.to_string_lossy().to_string();
            node_map.contains_key(&parent_str)
        } else {
            false
        };
        
        if !has_parent_in_entries {
            root_nodes.push(node.clone());
        }
    }
    
    // Sort root nodes
    root_nodes.sort_by(|a, b| {
        if a.is_dir != b.is_dir {
            b.is_dir.cmp(&a.is_dir)
        } else {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        }
    });
    
    root_nodes
}

