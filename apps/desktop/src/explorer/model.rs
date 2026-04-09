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
    
    // Create a map from path to node index
    let mut nodes: Vec<ExplorerNode> = entries.iter()
        .map(|entry| ExplorerNode::new(entry))
        .collect();
    
    // Sort nodes: directories first, then files, both alphabetically
    nodes.sort_by(|a, b| {
        if a.is_dir != b.is_dir {
            b.is_dir.cmp(&a.is_dir) // Directories first
        } else {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        }
    });
    
    // Build a map from path string to index
    let mut path_to_index: HashMap<String, usize> = HashMap::new();
    for (i, node) in nodes.iter().enumerate() {
        path_to_index.insert(node.path.to_string_lossy().to_string(), i);
    }
    
    // Group children by parent
    let mut children_by_parent: HashMap<String, Vec<usize>> = HashMap::new();
    
    for (i, node) in nodes.iter().enumerate() {
        if let Some(parent_path) = node.path.parent() {
            let parent_str = parent_path.to_string_lossy().to_string();
            if path_to_index.contains_key(&parent_str) {
                children_by_parent.entry(parent_str)
                    .or_insert_with(Vec::new)
                    .push(i);
            }
        }
    }
    
    // Build tree structure
    let mut root_indices = Vec::new();
    let mut visited = vec![false; nodes.len()];
    
    for i in 0..nodes.len() {
        if !visited[i] {
            root_indices.push(i);
            build_subtree(i, &mut nodes, &children_by_parent, &mut visited);
        }
    }
    
    // Collect root nodes in sorted order
    let mut root_nodes = Vec::new();
    for &idx in &root_indices {
        root_nodes.push(nodes[idx].clone());
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

fn build_subtree(
    index: usize,
    nodes: &mut [ExplorerNode],
    children_by_parent: &HashMap<String, Vec<usize>>,
    visited: &mut [bool],
) {
    if visited[index] {
        return;
    }
    visited[index] = true;
    
    let path_str = nodes[index].path.to_string_lossy().to_string();
    
    if let Some(child_indices) = children_by_parent.get(&path_str) {
        let mut children = Vec::new();
        for &child_idx in child_indices {
            if !visited[child_idx] {
                build_subtree(child_idx, nodes, children_by_parent, visited);
                children.push(nodes[child_idx].clone());
            }
        }
        
        // Sort children: directories first, then files, alphabetically
        children.sort_by(|a, b| {
            if a.is_dir != b.is_dir {
                b.is_dir.cmp(&a.is_dir)
            } else {
                a.name.to_lowercase().cmp(&b.name.to_lowercase())
            }
        });
        
        nodes[index].children = children;
    }
}
