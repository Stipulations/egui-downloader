use eframe::egui;
use reqwest::Client;
use rfd::FileDialog;
use std::{
    fs::File,
    io::copy,
    path::{Path, PathBuf},
};
use tokio::runtime::Runtime;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([500.0, 150.0]),
        ..Default::default()
    };

    let mut selected_folder: Option<PathBuf> = None;
    let mut current_path = std::env::temp_dir();
    let mut download_url = "https://example.com/file.file".to_string();

    eframe::run_simple_native("Downloader", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Download").clicked() {
                    if let Some(ref folder) = selected_folder {
                        let tk_runtime = Runtime::new().unwrap();
                        let url = download_url.clone();
                        let folder = folder.clone();

                        tk_runtime.block_on(async move {
                            match download_file(&url, &folder).await {
                                Ok(_) => println!("File downloaded."),
                                Err(e) => eprintln!("Failed to download: {}", e),
                            }
                        });
                    } else {
                        eprintln!("No folder selected.");
                    }
                }
                if ui.button("Select Folder").clicked() {
                    selected_folder = FileDialog::new().set_directory(&current_path).pick_folder();
                    if let Some(ref folder) = selected_folder {
                        current_path = folder.clone();
                    }
                }
            });

            ui.text_edit_singleline(&mut download_url);

            if let Some(ref folder) = selected_folder {
                ui.label(format!("Selected folder: {:?}", folder));
            } else {
                ui.label("No folder currently selected");
            }
        });
    })
}

async fn download_file(url: &str, folder: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let response = client.get(url).send().await?;
    if !response.status().is_success() {
        return Err(format!("Failed to download file: {}", response.status()).into());
    }

    let bytes = response.bytes().await?;

    let file_name = url.split('/').last().unwrap_or("downloaded_file.file");

    let destination = folder.join(file_name);
    let mut file = File::create(destination)?;
    copy(&mut bytes.as_ref(), &mut file)?;

    Ok(())
}