use dotext::{Docx, MsDoc};
use std::io::{self, Read};
use std::path::Path;
use std::fs;

/* Helper functions */
fn read_docx(path: &Path) -> std::io::Result<String>{
    let mut content = String::new();
    let mut docx = Docx::open(path)?;
    let bytes_read = docx.read_to_string(&mut content)?;

    Ok(content)
}

fn get_file_content(path: &Path) -> std::io::Result<String> {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("docx") => {
            let content = read_docx(path)?;
            return Ok(content);
        } 
        Some(ext) => {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Unsupported file type! {}", ext)));
        }
        None => {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("No file extension found!")));
        }
    }
}

#[tauri::command]
fn auto_fix(path: &Path) -> Result<String, String>{
    println!("Hi from auto_fix I got path {:?}", path);
    let content = get_file_content(path).map_err(|e| format!("Could not get file content with this error {}", e))?;
    Ok(content)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            auto_fix
        ])
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_docx_test() {
        let source_path = Path::new("./tests_files/Requirmenment for Editing Tool - My Project.docx");
        let result = read_docx(source_path);

        assert!(result.is_ok(), "Expected Ok but got error {}", result.unwrap_err());
    }
}
