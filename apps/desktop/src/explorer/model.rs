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
    
    // Sort entries: directories first, then files, alphabetically
    let mut sorted_entries: Vec<&DirectoryEntry> = entries.iter().collect();
    sorted_entries.sort_by(|a, b| {
        if a.is_dir != b.is_dir {
            b.is_dir.cmp(&a.is_dir) // Directories first
        } else {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        }
    });
    
    // Create a map from path to node
    let mut path_to_node: HashMap<String, ExplorerNode> = HashMap::new();
    
    // First pass: create all nodes without children
    for entry in sorted_entries {
        let path_str = entry.path.clone();
        let node = ExplorerNode::new(entry);
        path_to_node.insert(path_str, node);
    }
    
    // Second pass: build the tree
    let mut root_nodes = Vec::new();
    
    // We need to process nodes in a way that ensures parents are processed before children
    // But since we have all nodes, we can iterate through all paths
    let mut paths: Vec<String> = path_to_node.keys().cloned().collect();
    // Sort paths by length (shallowest first) to ensure parents are processed before children
    paths.sort_by_key(|path| path.len());
    
    for path in paths {
        // We need to get the node, but we can't borrow mutably while iterating
        // So we'll check if it exists and process it
        if let Some(mut node) = path_to_node.remove(&path) {
            // Find parent path
            let node_path = std::path::Path::new(&path);
            if let Some(parent_path) = node_path.parent() {
                let parent_str = parent_path.to_string_lossy().to_string();
                
                // If parent exists in our map, add this node as a child
                if let Some(parent_node) = path_to_node.get_mut(&parent_str) {
                    parent_node.children.push(node);
                    continue;
                }
            }
            // If no parent found or parent not in entries, this is a root node
            root_nodes.push(node);
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

