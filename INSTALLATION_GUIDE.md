# Install mn 0.1.2

The Phase 10 kit contains a source ZIP, platform ZIPs listed in RELEASE_MANIFEST.md, SHA256SUMS and an installation guide. Unpack the kit into a fresh, trusted directory. Check the kit digest against the value delivered with it. On Linux run `sha256sum -c SHA256SUMS` inside the kit. On macOS use `shasum -a 256 -c SHA256SUMS`. On Windows use `Get-FileHash -Algorithm SHA256 -LiteralPath <archive>` and compare the value to the corresponding line. The candidate currently has no release signature.

## Build from source using Cargo

Extract mongolia-lang-0.1.2-source.zip and enter mongolia-lang. Install rustup and a native linker. On Windows MSVC, install the Visual Studio C++ build tools. Cargo uses the pinned Rust toolchain. First use needs network access or a prepared toolchain and dependency cache.

```sh
cargo install --locked --path crates/mn-cli
mn --version
mn run examples/hello.mn
```

Expect `mn 0.1.2`. Cargo normally installs to ~/.cargo/bin or %USERPROFILE%\.cargo\bin. Add that directory to PATH. For a build without installation run `cargo run --locked -p mn-cli -- run examples/hello_mn.mn`. Offline builds need cached crates and the pinned toolchain.

## Install a binary manually

Select the OS and architecture in PLATFORM_SUPPORT_MATRIX.md. Check its ZIP digest in the kit’s top-level SHA256SUMS, then extract into an empty directory. Check that package’s SHA256SUMS. BUILD.json identifies the binary path, SHA-256, target and input hashes. On Linux/macOS copy bin/mn into a trusted PATH directory and set executable permissions with chmod 755. On Windows the equivalent filename is bin/mn.exe. Keep the notices when redistributing. Run the installed executable by its full path with --version and then --help. The Linux musl artifacts contain the host interpreter. They execute .mn programs on the CPU.

## Install with the provided scripts

Extract the source ZIP for the installer scripts. Choose a directory owned by your account. Use the binary digest from the verified platform package's BUILD.json in place of BINARY_SHA256. From the source directory:

```sh
sh installers/install.sh /path/to/package/bin/mn BINARY_SHA256 "$HOME/.local/bin"
"$HOME/.local/bin/mn" --version
```

For a verified Windows package, the prepared PowerShell command is:

```powershell
.\installers\install.ps1 -Binary C:\path\to\bin\mn.exe -Sha256 BINARY_SHA256 -BinDirectory "$HOME\.local\bin"
& "$HOME\.local\bin\mn.exe" --version
```

Both installers take local files and verify the staged copy. They refuse to replace an existing installation. For side-by-side testing choose another bin directory. To uninstall a manual installation remove the specific installed executable; for Cargo use `cargo uninstall mn-cli`. Package-manager availability and native-platform verification are tracked in PLATFORM_SUPPORT_MATRIX.md and docs/PACKAGE_DISTRIBUTION.md.
