//! Health check and diagnostics module.

use std::path::Path;

/// Run doctor command to check health and configuration.
///
/// Returns exit code: 0 if healthy, 1 if issues found.
pub fn run_doctor() -> i32 {
    println!("🏥 prompter health check");
    println!("========================");
    println!();

    let mut has_errors = false;

    // Check configuration
    println!("Configuration:");
    let home = std::env::var("HOME").unwrap_or_else(|_| "~".to_string());
    let config_path = Path::new(&home).join(".config/prompter/config.toml");

    if config_path.exists() {
        println!("  ✅ Config file: {}", config_path.display());

        // Try to parse it
        match std::fs::read_to_string(&config_path) {
            Ok(content) => {
                if toml::from_str::<toml::Value>(&content).is_ok() {
                    println!("  ✅ Config is valid TOML");
                } else {
                    println!("  ❌ Config is invalid TOML");
                    has_errors = true;
                }
            }
            Err(e) => {
                println!("  ❌ Failed to read config: {e}");
                has_errors = true;
            }
        }
    } else {
        println!("  ❌ Config file not found: {}", config_path.display());
        println!("  ℹ️  Run 'prompter init' to create default configuration");
        has_errors = true;
    }

    // Check library directory
    let library_path = Path::new(&home).join(".local/prompter/library");

    if library_path.exists() {
        println!("  ✅ Library directory: {}", library_path.display());
    } else {
        println!(
            "  ❌ Library directory not found: {}",
            library_path.display()
        );
        println!("  ℹ️  Run 'prompter init' to create default library");
        has_errors = true;
    }

    println!();

    // Version info
    println!("Version:");
    println!("  ℹ️  Current version: v{}", env!("CARGO_PKG_VERSION"));
    println!("  💡 Check https://github.com/tftio/prompter/releases for updates");

    println!();

    // Summary
    if has_errors {
        println!("❌ Errors found");
        1
    } else {
        println!("✨ Everything looks healthy!");
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_doctor_returns_valid_exit_code() {
        let exit_code = run_doctor();
        // Should return 0 or 1
        assert!(exit_code == 0 || exit_code == 1);
    }
}
