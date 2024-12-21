// Purpose: Replace Arbitrum Stylus rust build with our own build wasm-file
// Do not modify this file.

use std::fs;
use std::path::Path;

fn main() {
    // This ensures that Cargo will re-run the build script if the source or the .wasm file changes
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=/source/target/wasm32-unknown-unknown/release/deps/stylus_starter_c.wasm");
    println!("cargo:rerun-if-changed=target/wasm32-unknown-unknown/release/deps/stylus_starter_c.wasm");

    let source_paths = [
        // We want to move contract.wasm to the target directory
        // So docker can pick it up and deploy it to the chain
        Path::new("contract.wasm"),
    ];

    let target_dirs = [
        // Docker target directory
        Path::new("/source/target/wasm32-unknown-unknown/release/deps"),
        // Local target directory
        Path::new("target/wasm32-unknown-unknown/release/deps"),
    ];

    // Try to copy the source file to both target locations
    for source_path in &source_paths {
        if source_path.exists() {
            for target_dir in &target_dirs {
                // Try to ensure the target directory exists
                match fs::create_dir_all(target_dir) {
                    Ok(_) => {
                        // Perform the copy operation if the directory creation is successful
                        if let Err(e) = fs::copy(source_path, target_dir.join("stylus_starter_c.wasm")) {
                            println!("cargo:warning=Failed to copy .wasm file to {}: {}", target_dir.display(), e);
                        } else {
                            println!("cargo:rerun-if-changed={}", source_path.display());
                        }
                    }
                    Err(e) => {
                        // If directory creation fails, print a warning and continue
                        println!("cargo:warning=Failed to create target directory {}: {}", target_dir.display(), e);
                    }
                }
            }
            return; // Exit once the file is copied to both locations
        }
    }

    // If the source file was not found, print a warning
    println!("cargo:warning=No source file found to copy.");
}
