use base64::{engine::general_purpose::STANDARD, Engine as _};

#[derive(serde::Serialize)]
struct PdfEntry {
    name: String,
    path: String,
}

/// List all PDF files directly inside `dir` (non-recursive), sorted by name.
#[tauri::command]
fn list_pdfs(dir: String) -> Result<Vec<PdfEntry>, String> {
    let mut out = Vec::new();
    for entry in std::fs::read_dir(&dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_file() {
            let is_pdf = path
                .extension()
                .map(|e| e.to_string_lossy().to_lowercase() == "pdf")
                .unwrap_or(false);
            if is_pdf {
                let name = path.file_name().unwrap().to_string_lossy().to_string();
                out.push(PdfEntry {
                    name,
                    path: path.to_string_lossy().to_string(),
                });
            }
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
        .invoke_handler(tauri::generate_handler![list_pdfs, read_pdf])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
