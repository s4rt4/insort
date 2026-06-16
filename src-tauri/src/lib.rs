use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::path::Path;

#[derive(serde::Serialize)]
struct PdfEntry {
    name: String,
    path: String,
}

/// One node in the folder tree: a directory (with children) or a PDF file.
#[derive(serde::Serialize)]
struct TreeNode {
    name: String,
    path: String,
    is_dir: bool,
    /// Present only for directories. Subfolders first, then PDFs, both A-Z.
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<TreeNode>>,
}

fn is_pdf(path: &Path) -> bool {
    path.extension()
        .map(|e| e.to_string_lossy().to_lowercase() == "pdf")
        .unwrap_or(false)
}

/// Build the direct children of `dir`: recursively-scanned subfolders (empty ones
/// pruned) followed by PDF files. Unreadable subfolders are skipped silently.
fn build_children(dir: &Path) -> Vec<TreeNode> {
    let mut dirs: Vec<TreeNode> = Vec::new();
    let mut files: Vec<TreeNode> = Vec::new();

    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return Vec::new(),
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            // Keep a subfolder only if it (recursively) contains at least one PDF.
            if let Some(node) = scan_dir(&path) {
                dirs.push(node);
            }
        } else if is_pdf(&path) {
            files.push(TreeNode {
                name: path.file_name().unwrap().to_string_lossy().to_string(),
                path: path.to_string_lossy().to_string(),
                is_dir: false,
                children: None,
            });
        }
    }

    dirs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    files.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    dirs.extend(files);
    dirs
}

/// Scan a subfolder. Returns `None` if it contains no PDFs anywhere (so it gets
/// pruned from the tree).
fn scan_dir(dir: &Path) -> Option<TreeNode> {
    let children = build_children(dir);
    if children.is_empty() {
        return None;
    }
    Some(TreeNode {
        name: dir
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| dir.to_string_lossy().to_string()),
        path: dir.to_string_lossy().to_string(),
        is_dir: true,
        children: Some(children),
    })
}

/// Build a recursive folder tree rooted at `dir`. The root is always returned
/// (even if it has no PDFs); empty subfolders below it are pruned.
#[tauri::command]
fn list_tree(dir: String) -> Result<TreeNode, String> {
    let p = Path::new(&dir);
    if !p.is_dir() {
        return Err(format!("Not a folder: {}", dir));
    }
    Ok(TreeNode {
        name: p
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| dir.clone()),
        path: dir.clone(),
        is_dir: true,
        children: Some(build_children(p)),
    })
}

/// List all PDF files directly inside `dir` (non-recursive), sorted by name.
/// Kept for compatibility; the UI now uses `list_tree`.
#[tauri::command]
fn list_pdfs(dir: String) -> Result<Vec<PdfEntry>, String> {
    let mut out = Vec::new();
    for entry in std::fs::read_dir(&dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_file() && is_pdf(&path) {
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            out.push(PdfEntry {
                name,
                path: path.to_string_lossy().to_string(),
            });
        }
    }
    out.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(out)
}

/// Read a PDF file and return it as a base64 string (for blob preview in the webview).
#[tauri::command]
fn read_pdf(path: String) -> Result<String, String> {
    let bytes = std::fs::read(&path).map_err(|e| e.to_string())?;
    Ok(STANDARD.encode(bytes))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![list_tree, list_pdfs, read_pdf])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
