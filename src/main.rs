use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

fn main() -> io::Result<()> {
    // Use default directories if no arguments are provided
    let args: Vec<String> = env::args().collect();
    let input_dir = args.get(1).map(String::as_str).unwrap_or("input");
    let output_dir = args.get(2).map(String::as_str).unwrap_or("output");

    // Create output directory if it doesn't exist
    fs::create_dir_all(output_dir)?;
    fs::create_dir_all(input_dir)?; //creating this as well if not there already

    // Iterate over each file in the input directory
    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            println!("Processing file: {}", path.display());
            let corrected_content = process_file(&path)?;
            let output_path = Path::new(output_dir).join(path.file_name().unwrap());
            fs::write(output_path, corrected_content)?;
        }
    }

    println!("All files processed successfully.");
    Ok(())
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
    println!("{}", corrected_content);

    Ok(corrected_content)
}
