# Build Instructions for RailOSConsist

## Prerequisites

- Rust toolchain (1.70+)
- For macOS bundling: `cargo-bundle` (optional)

## Building

### Standard Build
```bash
cargo build --release
```

The release binary will be in `target/release/`.

### Platform-Specific Builds

#### Windows
Before building on Windows, set the Rust toolchain to the stable MSVC version:
```bash
rustup default stable-x86_64-pc-windows-msvc
```

The Windows build automatically embeds the application icon in the executable:
```bash
cargo build --release
```

The icon (`RailOSConsist.ico`) will be embedded in the `.exe` file.

#### macOS
For a proper macOS app bundle with the icon:

1. Install cargo-bundle (if not already installed):
```bash
cargo install cargo-bundle
```

2. Create the app bundle:
```bash
cargo bundle --release
```

The app bundle will be in `target/release/bundle/osx/`.

The bundle is created with:
- Application icon from `media/RailOSConsist.png`
- Proper macOS app structure
- Embedded data files
- Ready for distribution to App Store or direct distribution

#### Linux
Standard binary build:
```bash
cargo build --release
```

For distribution, create a tarball:
```bash
tar czf railosconsist-linux-x86_64.tar.gz -C target/release railosconsist
```

## Testing

Run the test suite:
```bash
cargo test
```

Run with verbose output:
```bash
cargo test -- --nocapture
```

## Release Build

Build optimized release binaries for all platforms:
```bash
cargo build --release
```

Or for macOS app bundle:
```bash
cargo bundle --release
```

## Documentation

The full Cargo API documentation is available in every release as a PDF file: **RailOSConsist-Developer-API-Documentation.pdf**. This is intended for developers and includes the complete API reference for all modules and components.

To generate documentation locally:
```bash
cargo doc --no-deps --document-private-items
```

The generated HTML documentation will be in `target/doc/`. Open `target/doc/RailOSConsist/index.html` in a browser to view it.

## Icon Files

The application uses icon files from the `media/` directory:
- `RailOSConsist.png` - Used by macOS bundler and runtime
- `RailOSConsist.ico` - Used by Windows resource compiler
- `icon.svg` - Scalable vector format

## Configuration

Build behavior is configured in:
- `Cargo.toml` - Package metadata and bundle settings
- `build.rs` - Windows-specific resource compilation
