/// Build script for RailOSConsist
///
/// Configures platform-specific build settings:
/// - Windows: Embeds the application icon in the executable using winres
/// - macOS: Configured in Cargo.toml metadata.bundle section for cargo-bundle
/// - Linux: Uses system icon theme defaults
use embed_resource;

fn main() {
    #[cfg(target_os = "windows")]
    {
        // Tell Cargo to watch both the script and the icon for changes
        println!("cargo:rerun-if-changed=icon.rc");
        println!("cargo:rerun-if-changed=media/RailOSConsist.ico");

        // Compile the resource script
        embed_resource::compile("icon.rc", embed_resource::NONE);
    }
}
