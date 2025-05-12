use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use native_dialog::{FileDialog, MessageDialog, MessageType};
use eframe::egui;


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        //initial_window_size: Some((400.0, 400.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Laserhelfer",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    input_dir: String,
    output_dir: String,
    single_file: Option<PathBuf>,
    processed_files: Vec<String>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            input_dir: "input".to_string(),
            output_dir: "output".to_string(),
            single_file: None,
            processed_files: Vec::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Laserhelfer");
            ui.separator();

            if ui.button("Use Default Directories").clicked() {
                self.process_directory();
            }

            if ui.button("Select Input and Output Directories").clicked() {
                if let Some(input) = FileDialog::new().show_open_single_dir().ok().flatten() {
                    self.input_dir = input.to_string_lossy().to_string();
                }
                if let Some(output) = FileDialog::new().show_open_single_dir().ok().flatten() {
                    self.output_dir = output.to_string_lossy().to_string();
                }
                self.process_directory();
            }

            if ui.button("Select Single File").clicked() {
                if let Some(file) = FileDialog::new().show_open_single_file().ok().flatten() {
                    self.single_file = Some(file);
                    self.process_single_file();
                }
            }

            ui.label(format!("Input Directory: {}", self.input_dir));
            ui.label(format!("Output Directory: {}", self.output_dir));
            if let Some(file) = &self.single_file {
                ui.label(format!("Single File: {}", file.display()));
            }

            ui.separator();
            ui.heading("Processed Files:");
            egui::ScrollArea::vertical().show(ui, |ui| {
                for file in &self.processed_files {
                    ui.label(file);
                }
            });
            if ui.button("Clear Processed Files").clicked() {
                self.processed_files.clear();
            }
        });
    }
}

impl MyApp {
    fn process_directory(&mut self) {
        if let Err(e) = fs::create_dir_all(&self.output_dir) {
            MessageDialog::new()
                .set_type(MessageType::Error)
                .set_title("Error")
                .set_text(&format!("Failed to create output directory: {}", e))
                .show_alert()
                .unwrap();
            return;
        }

        for entry in fs::read_dir(&self.input_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() {
                self.process_file(&path);
            }
        }

        MessageDialog::new()
            .set_type(MessageType::Info)
            .set_title("Success")
            .set_text("All files processed successfully.")
            .show_alert()
            .unwrap();
    }

    fn process_single_file(&mut self) {
        if self.single_file.is_some() {
            let file = self.single_file.take().unwrap(); // Take the file out, avoiding borrow conflicts
            self.process_file(&file);
            MessageDialog::new()
                .set_type(MessageType::Info)
                .set_title("Success")
                .set_text("Single file processed successfully.")
                .show_alert()
                .unwrap();
        }
    }


    fn process_file(&mut self, path: &Path) {
        let corrected_content = process_file(path).unwrap();
        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
        let corrected_file_name = format!("c_{}", file_name);
        let output_path = PathBuf::from(&self.output_dir).join(&corrected_file_name);
        fs::write(output_path, corrected_content).unwrap();
        self.processed_files.push(corrected_file_name);
    }
}

fn process_file(path: &Path) -> io::Result<String> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let mut corrected_content = String::new();
    let flag = "G0";
    let insert_before = "M5";
    let insert_after = "M3";

    for line in reader.lines() {
        let line = line?;
        if line.starts_with(flag) {
            corrected_content.push_str(&format!("{}\n{}\n{}\n", insert_before, line, insert_after));
        } else {
            corrected_content.push_str(&format!("{}\n", line));
        }
    }

    Ok(corrected_content)
}
