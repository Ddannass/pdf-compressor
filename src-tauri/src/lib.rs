use std::process::Command;
use tauri::Manager;

#[tauri::command]
fn compress_pdf(app: tauri::AppHandle, input_path: String) -> Result<String, String> {
    let output_path = input_path.replace(".pdf", "_compressed.pdf");

    let temp_dir = std::env::temp_dir().join("pdfcompressor");

    let _ = std::fs::remove_dir_all(&temp_dir);
    std::fs::create_dir_all(&temp_dir).unwrap();

    let resource_dir = app
        .path()
        .resource_dir()
        .map_err(|e| e.to_string())?;

    let poppler_path = resource_dir
        .join("resources")
        .join("poppler")
        .join("pdftoppm.exe");

    let magick_path = resource_dir
        .join("resources")
        .join("magick")
        .join("magick.exe");

    let page_prefix = temp_dir.join("page");

    let status = Command::new(poppler_path)
        .args([
            &input_path,
            page_prefix.to_str().unwrap(),
            "-jpeg",
            "-jpegopt",
            "quality=25",
            "-r",
            "80",
        ])
        .status()
        .map_err(|e| e.to_string())?;

    if !status.success() {
        return Err("Failed to convert PDF pages".into());
    }

    let pattern = temp_dir.join("page-*.jpg");

    let status = Command::new(magick_path)
        .args([
            pattern.to_str().unwrap(),
            &output_path,
        ])
        .status()
        .map_err(|e| e.to_string())?;

    if !status.success() {
        return Err("Failed to rebuild PDF".into());
    }

    let _ = std::fs::remove_dir_all(&temp_dir);

    Ok(output_path)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![compress_pdf])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
