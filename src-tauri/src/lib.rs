use std::process::Command;

#[tauri::command]
fn compress_pdf(input_path: String) -> Result<String, String> {
    let output_path = input_path.replace(".pdf", "_compressed.pdf");

    let temp_dir = "/tmp/pdfcompressor";

    let _ = std::fs::remove_dir_all(temp_dir);
    std::fs::create_dir_all(temp_dir).unwrap();

    let status = Command::new("pdftoppm")
        .args([
            &input_path,
            &format!("{}/page", temp_dir),
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

    let status = Command::new("magick")
        .args([
            &format!("{}/page-*.jpg", temp_dir),
            &output_path,
        ])
        .status()
        .map_err(|e| e.to_string())?;

    if !status.success() {
        return Err("Failed to rebuild PDF".into());
    }

    let _ = std::fs::remove_dir_all(temp_dir);

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
