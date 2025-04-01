use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=C:\\node_projects\\vgau-bot-editor\\src\\assets\\wasm/");
    
    // Set version info from git if available
    if let Ok(output) = Command::new("git").args(["rev-parse", "HEAD"]).output() {
        if output.status.success() {
            let git_hash = String::from_utf8_lossy(&output.stdout);
            println!("cargo:rustc-env=GIT_HASH={}", git_hash.trim());
        }
    }
    
    // Check if static directory exists, create if not
    let static_dir = PathBuf::from("C:\\node_projects\\vgau-bot-editor\\src\\assets\\wasm");
    if !static_dir.exists() {
        fs::create_dir_all(&static_dir).expect("Failed to create static directory");
        
        // Create a placeholder favicon
        let favicon_path = static_dir.join("favicon.ico");
        if !favicon_path.exists() {
            println!("cargo:warning=Creating placeholder favicon.ico");
            // We're not actually creating one, just noting we could
        }
    }
    
    // Only run the following for WASM target
    if env::var("TARGET").unwrap_or_default().contains("wasm32") {
        // Get output directory
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        println!("cargo:warning=OUT_DIR is {}", out_dir.display());
        
        // This would be used by wasm-pack or trunk to copy static assets
        println!("cargo:warning=Building for WebAssembly target");
    } else {
        println!("cargo:warning=Building for native target");
    }
} 