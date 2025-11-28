#!/bin/bash
#
# Cross-platform build script for cargo-x402
# Builds binaries for all supported targets with proper naming and checksums
#
# Targets:
#   - x86_64-unknown-linux-gnu (Linux x86_64)
#   - x86_64-apple-darwin (macOS Intel)
#   - aarch64-apple-darwin (macOS Apple Silicon)
#   - x86_64-pc-windows-msvc (Windows x86_64)
#   - aarch64-unknown-linux-gnu (Linux ARM64)
#
# Usage: ./scripts/build-all.sh [VERSION]
# Example: ./scripts/build-all.sh v1.0.0

set -e

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
PROJECT_NAME="cargo-x402"
VERSION="${1:-$(git describe --tags --always 2>/dev/null || echo 'dev')}"
RELEASE_DIR="./release"
BUILD_DIR="./target"

# Target platforms
declare -a TARGETS=(
    "x86_64-unknown-linux-gnu"
    "x86_64-apple-darwin"
    "aarch64-apple-darwin"
    "x86_64-pc-windows-msvc"
    "aarch64-unknown-linux-gnu"
)

# Binary extensions
declare -A EXTENSIONS=(
    ["x86_64-unknown-linux-gnu"]=""
    ["x86_64-apple-darwin"]=""
    ["aarch64-apple-darwin"]=""
    ["x86_64-pc-windows-msvc"]=".exe"
    ["aarch64-unknown-linux-gnu"]=""
)

# Functions
print_header() {
    echo -e "${BLUE}================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}================================${NC}"
}

print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

print_error() {
    echo -e "${RED}✗ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

check_prerequisites() {
    print_header "Checking Prerequisites"

    # Check if Rust is installed
    if ! command -v rustc &> /dev/null; then
        print_error "Rust is not installed. Please install Rust: https://rustup.rs"
        exit 1
    fi
    print_success "Rust is installed: $(rustc --version)"

    # Check if cargo is installed
    if ! command -v cargo &> /dev/null; then
        print_error "Cargo is not installed"
        exit 1
    fi
    print_success "Cargo is installed: $(cargo --version)"

    # Check for required tools
    if ! command -v sha256sum &> /dev/null && ! command -v shasum &> /dev/null; then
        print_error "sha256sum or shasum is required for checksums"
        exit 1
    fi
    print_success "Checksum tool found"

    echo ""
}

setup_targets() {
    print_header "Setting Up Build Targets"

    for target in "${TARGETS[@]}"; do
        if rustup target list | grep -q "^$target (installed)"; then
            print_success "Target $target is installed"
        else
            echo "Installing target: $target"
            rustup target add "$target" || print_warning "Could not install $target (may not be available on this platform)"
        fi
    done

    echo ""
}

build_binaries() {
    print_header "Building Binaries"

    # Clean previous builds
    rm -rf "$RELEASE_DIR"
    mkdir -p "$RELEASE_DIR"

    for target in "${TARGETS[@]}"; do
        print_header "Building for $target"

        if ! rustup target list | grep -q "^$target (installed)"; then
            print_warning "Skipping $target (not available on this platform)"
            continue
        fi

        # Build release binary
        if cargo build --release --target "$target" 2>&1 | tail -5; then
            print_success "Successfully built for $target"

            # Copy binary with appropriate naming
            ext="${EXTENSIONS[$target]}"
            source_binary="$BUILD_DIR/$target/release/$PROJECT_NAME$ext"
            dest_binary="$RELEASE_DIR/${PROJECT_NAME}-${VERSION}-${target}${ext}"

            if [ -f "$source_binary" ]; then
                cp "$source_binary" "$dest_binary"
                chmod +x "$dest_binary"
                print_success "Binary copied to $dest_binary"
            else
                print_error "Binary not found at $source_binary"
            fi
        else
            print_error "Failed to build for $target"
        fi

        echo ""
    done
}

generate_checksums() {
    print_header "Generating SHA256 Checksums"

    checksums_file="$RELEASE_DIR/SHA256SUMS"
    > "$checksums_file"  # Clear file

    for binary in "$RELEASE_DIR"/*; do
        if [ -f "$binary" ] && [[ ! "$binary" == *"SHA256SUMS"* ]]; then
            if command -v sha256sum &> /dev/null; then
                sha256sum "$binary" >> "$checksums_file"
            else
                shasum -a 256 "$binary" >> "$checksums_file"
            fi

            binary_name=$(basename "$binary")
            print_success "Checksum generated for $binary_name"
        fi
    done

    echo ""
    echo "All checksums written to: $checksums_file"
    echo ""
}

create_install_scripts() {
    print_header "Creating Installation Scripts"

    # Unix install script
    cat > "$RELEASE_DIR/install.sh" << 'EOF'
#!/bin/bash
# Installation script for cargo-x402 on Unix/macOS/Linux

set -e

VERSION="v1.0.0"
TARGET="${TARGET:-}"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.cargo/bin}"

# Detect OS and architecture
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    if command -v uname &> /dev/null; then
        arch=$(uname -m)
        if [ "$arch" == "x86_64" ]; then
            TARGET="x86_64-unknown-linux-gnu"
        elif [ "$arch" == "aarch64" ]; then
            TARGET="aarch64-unknown-linux-gnu"
        fi
    fi
elif [[ "$OSTYPE" == "darwin"* ]]; then
    if command -v uname &> /dev/null; then
        arch=$(uname -m)
        if [ "$arch" == "x86_64" ]; then
            TARGET="x86_64-apple-darwin"
        elif [ "$arch" == "arm64" ]; then
            TARGET="aarch64-apple-darwin"
        fi
    fi
fi

if [ -z "$TARGET" ]; then
    echo "Error: Could not detect platform. Please set TARGET environment variable."
    echo "Available targets:"
    echo "  - x86_64-unknown-linux-gnu (Linux x86_64)"
    echo "  - aarch64-unknown-linux-gnu (Linux ARM64)"
    echo "  - x86_64-apple-darwin (macOS Intel)"
    echo "  - aarch64-apple-darwin (macOS Apple Silicon)"
    exit 1
fi

BINARY_NAME="cargo-x402-${VERSION}-${TARGET}"
BINARY_PATH="$(dirname "$0")/$BINARY_NAME"

if [ ! -f "$BINARY_PATH" ]; then
    echo "Error: Binary not found at $BINARY_PATH"
    exit 1
fi

# Create install directory if it doesn't exist
mkdir -p "$INSTALL_DIR"

# Copy binary
cp "$BINARY_PATH" "$INSTALL_DIR/cargo-x402"
chmod +x "$INSTALL_DIR/cargo-x402"

echo "✓ cargo-x402 installed to $INSTALL_DIR/cargo-x402"
echo ""
echo "Make sure $INSTALL_DIR is in your PATH:"
echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
echo ""
echo "Then verify installation:"
echo "  cargo-x402 --version"
EOF

    chmod +x "$RELEASE_DIR/install.sh"
    print_success "Created Unix install script: install.sh"

    # Windows install script (PowerShell)
    cat > "$RELEASE_DIR/install.ps1" << 'EOF'
# Installation script for cargo-x402 on Windows (PowerShell)

param(
    [string]$InstallDir = "$HOME\.cargo\bin"
)

$Version = "v1.0.0"
$ProjectName = "cargo-x402"

# Detect architecture
$Architecture = [System.Runtime.InteropServices.RuntimeInformation]::ProcessArchitecture
$Target = "x86_64-pc-windows-msvc"

if ($Architecture -eq "Arm64") {
    $Target = "aarch64-pc-windows-msvc"
}

$BinaryName = "$ProjectName-$Version-$Target.exe"
$BinaryPath = Join-Path (Split-Path $MyInvocation.MyCommand.Path) $BinaryName

if (-not (Test-Path $BinaryPath)) {
    Write-Host "Error: Binary not found at $BinaryPath" -ForegroundColor Red
    exit 1
}

# Create install directory if it doesn't exist
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

# Copy binary
Copy-Item $BinaryPath "$InstallDir\cargo-x402.exe" -Force
Write-Host "✓ cargo-x402 installed to $InstallDir\cargo-x402.exe" -ForegroundColor Green
Write-Host ""
Write-Host "Make sure $InstallDir is in your PATH" -ForegroundColor Yellow
Write-Host "Then verify installation:"
Write-Host "  cargo-x402 --version" -ForegroundColor Gray
EOF

    print_success "Created Windows install script: install.ps1"
    echo ""
}

display_summary() {
    print_header "Build Summary"

    echo "Version: $VERSION"
    echo "Release directory: $RELEASE_DIR"
    echo ""

    echo "Built binaries:"
    for binary in "$RELEASE_DIR"/*; do
        if [ -f "$binary" ] && [[ ! "$binary" == *"SHA256SUMS"* ]] && [[ ! "$binary" == *".sh" ]] && [[ ! "$binary" == *".ps1" ]]; then
            size=$(du -h "$binary" | cut -f1)
            binary_name=$(basename "$binary")
            echo "  ✓ $binary_name ($size)"
        fi
    done

    echo ""
    echo "Next steps:"
    echo "  1. Verify checksums: cat $RELEASE_DIR/SHA256SUMS"
    echo "  2. Test installation: $RELEASE_DIR/install.sh"
    echo "  3. Create GitHub release with these binaries"
    echo "  4. Publish to crates.io with: cargo publish"
    echo ""
}

main() {
    print_header "cargo-x402 Cross-Platform Build (v1.0.0)"
    echo "Building for version: $VERSION"
    echo ""

    check_prerequisites
    setup_targets
    build_binaries
    generate_checksums
    create_install_scripts
    display_summary
}

main "$@"
