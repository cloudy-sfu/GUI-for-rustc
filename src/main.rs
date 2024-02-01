use rfd;
use std::process::Command;
use std::env;
use std::io::{self, Read};

fn main() {
    // Open file dialog for the user to choose a file
    let file_path = rfd::FileDialog::new().pick_file();

    if let Some(path) = file_path {
        let base_dir = path.parent().unwrap().to_str().unwrap();
        let filename = path.file_stem().unwrap().to_str().unwrap();

        // Change the current directory to the base directory of the chosen file
        env::set_current_dir(&base_dir).expect("Failed to change directory.");

        // Compile the Rust source file
        Command::new("rustc")
            .arg(format!("{}.rs", filename))
            .arg("--out-dir")
            .arg("dist")
            .status()
            .expect("Failed to compile Rust file.");

        // Build the executable path
        let executable = if cfg!(target_os = "windows") {
            format!("dist/{}.exe", filename)
        } else {
            format!("dist/{}", filename)
        };

        // Execute the compiled program
        Command::new(executable)
            .status()
            .expect("Failed to execute compiled program.");
    } else {
        println!("No file was chosen.");
    }

    // After executing the compiled program
    println!("\nPress any key to exit...");
    let mut stdin = io::stdin();
    let _ = stdin.read(&mut [0u8]).unwrap();
}
