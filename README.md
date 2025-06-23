# SVG Utils

Made by Oliver Shearman

## Building

Clone the repository, and run `cargo build --release` in the project root.

Rust will automatically target the system architecture unless flagged, so for building for Intel/x86 Macintosh you should run
`cargo build --release --target x86_64-apple-darwin`. Note that Apple Silicon devices can run this binary via Rosetta 2.

## Usage

Move the compiled binary into a folder and create an `input` folder alongside it. Place your SVG files/folders of SVG files (the program recursively walks every directory inside of the `input` folder) into the `input` directory.

Double-click the `svg-utils` executable and provide any of the prompted options. If something should not be changed, press `enter` to skip.

The modified files are outputted to the `output` directory, which will generate itself alongside the `input` folder.
