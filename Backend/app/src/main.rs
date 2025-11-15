mod models;
mod https_server;
mod auth;

use https_server::{run_https_server, run_dev_server};

// ===========================================
// MAIN APPLICATION
// ===========================================

#[tokio::main]
async fn main() {
    // Check command line arguments for server mode
    let args: Vec<String> = std::env::args().collect();
    let is_https = args.contains(&"--https".to_string());
    let is_dev = args.contains(&"--dev".to_string());
    
    if is_https {
        println!("🔐 Starting HTTPS server...");
        if let Err(e) = run_https_server().await {
            eprintln!("❌ HTTPS server failed to start: {}", e);
            std::process::exit(1);
        }
    } else if is_dev {
        println!("🚀 Starting development server (HTTP)...");
        if let Err(e) = run_dev_server().await {
            eprintln!("❌ Development server failed to start: {}", e);
            std::process::exit(1);
        }
    } else {
        println!("📖 Usage:");
        println!("  cargo run -- --dev     # Run HTTP development server");
        println!("  cargo run -- --https   # Run HTTPS production server");
        println!("");
        println!("🔐 For HTTPS, make sure you have certificates in ./certs/");
        println!("   Run: ./scripts/generate_dev_certs.sh");
    }
}
