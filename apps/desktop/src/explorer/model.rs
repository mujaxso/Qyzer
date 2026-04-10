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
    
    // Create all nodes first, without children
    let mut nodes: Vec<ExplorerNode> = sorted_entries
        .iter()
        .map(|entry| ExplorerNode::new(entry))
        .collect();
    
    // Build a map from path string to index
    let mut path_to_index: HashMap<String, usize> = HashMap::new();
    for (i, node) in nodes.iter().enumerate() {
        let path_str = node.path.to_string_lossy().to_string();
        path_to_index.insert(path_str, i);
    }
    
    // For each node, find its parent and add it as a child
    // We need to be careful about borrowing, so we'll collect parent-child relationships first
    let mut parent_to_children: HashMap<usize, Vec<usize>> = HashMap::new();
    
    for i in 0..nodes.len() {
        let node_path = &nodes[i].path;
        if let Some(parent_path) = node_path.parent() {
            let parent_str = parent_path.to_string_lossy().to_string();
            if let Some(&parent_idx) = path_to_index.get(&parent_str) {
                parent_to_children.entry(parent_idx)
                    .or_insert_with(Vec::new)
                    .push(i);
            }
        }
    }
    
    // Now, build the tree by reorganizing nodes in place
    // We need to process nodes in a way that ensures children are processed before their parents
    // But since we're building a tree, we need to process from leaves to root
    // However, we can use a different approach: build the tree by moving nodes
    
    // First, mark which nodes have parents
    let mut has_parent = vec![false; nodes.len()];
    
    for (parent_idx, child_indices) in &parent_to_children {
        for &child_idx in child_indices {
            has_parent[child_idx] = true;
        }
    }
    
    // Now, for each parent, collect their children
    for (parent_idx, child_indices) in parent_to_children {
        // Sort children indices: directories first, then files, alphabetically
        let mut sorted_child_indices: Vec<usize> = child_indices;
        sorted_child_indices.sort_by(|&a_idx, &b_idx| {
            let a = &nodes[a_idx];
            let b = &nodes[b_idx];
            if a.is_dir != b.is_dir {
                b.is_dir.cmp(&a.is_dir)
            } else {
                a.name.to_lowercase().cmp(&b.name.to_lowercase())
            }
        });
        
        // Collect children nodes
        let mut children = Vec::new();
        for child_idx in sorted_child_indices {
            // Take the node from the vector
            // We need to be careful here - we can't move out of nodes while iterating
            // So we'll use a different approach
            children.push(nodes[child_idx].clone());
        }
        
        // Set children on parent
        nodes[parent_idx].children = children;
    }
    
    // Collect root nodes (nodes without parents)
    let mut root_nodes = Vec::new();
    for i in 0..nodes.len() {
        if !has_parent[i] {
            root_nodes.push(nodes[i].clone());
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

