VERSION := $(shell grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)"/\1/')
NAME    := fifteen-game
DIST    := dist

.PHONY: all clean linux-bin linux-deb linux-rpm windows mac setup

all: linux-bin linux-deb linux-rpm windows

clean:
	cargo clean
	rm -rf $(DIST)

$(DIST):
	mkdir -p $(DIST)

# --- Linux ---

linux-bin: $(DIST)
	cargo build --release
	cp target/release/$(NAME) $(DIST)/$(NAME)-$(VERSION)-linux-x86_64
	@echo "Built: $(DIST)/$(NAME)-$(VERSION)-linux-x86_64"

linux-deb: linux-bin
	cargo deb --no-build
	cp target/debian/*.deb $(DIST)/
	@echo "Built: $(DIST)/*.deb"

linux-rpm: linux-bin
	cargo generate-rpm
	cp target/generate-rpm/*.rpm $(DIST)/
	@echo "Built: $(DIST)/*.rpm"

# --- Windows (cross-compile with mingw, no docker needed) ---

windows: $(DIST)
	cargo build --release --target x86_64-pc-windows-gnu
	cp target/x86_64-pc-windows-gnu/release/$(NAME).exe $(DIST)/$(NAME)-$(VERSION)-windows-x86_64.exe
	@echo "Built: $(DIST)/$(NAME)-$(VERSION)-windows-x86_64.exe"

# --- macOS (cross-compile, needs osxcross or run on mac) ---

mac: $(DIST)
	cross build --release --target x86_64-apple-darwin
	cp target/x86_64-apple-darwin/release/$(NAME) $(DIST)/$(NAME)-$(VERSION)-macos-x86_64
	@echo "Built: $(DIST)/$(NAME)-$(VERSION)-macos-x86_64"

mac-arm: $(DIST)
	cross build --release --target aarch64-apple-darwin
	cp target/aarch64-apple-darwin/release/$(NAME) $(DIST)/$(NAME)-$(VERSION)-macos-aarch64
	@echo "Built: $(DIST)/$(NAME)-$(VERSION)-macos-aarch64"

# --- Setup (install build tools) ---

setup:
	sudo apt install -y mingw-w64
	rustup target add x86_64-pc-windows-gnu
	rustup target add x86_64-apple-darwin
	rustup target add aarch64-apple-darwin
	cargo install cargo-deb
	cargo install cargo-generate-rpm
	@echo "All build tools installed"

help:
	@echo "Targets:"
	@echo "  all          - Build linux-bin, linux-deb, linux-rpm, windows"
	@echo "  linux-bin    - Release binary for Linux x86_64"
	@echo "  linux-deb    - Debian .deb package"
	@echo "  linux-rpm    - RPM package"
	@echo "  windows      - Windows .exe (via mingw-w64)"
	@echo "  mac          - macOS x86_64 (via cross/osxcross)"
	@echo "  mac-arm      - macOS aarch64 (via cross/osxcross)"
	@echo "  setup        - Install all required tools"
	@echo "  clean        - Remove build artifacts"
	@echo ""
	@echo "Prerequisites: run 'make setup' first"
	@echo "Windows build needs: mingw-w64 (apt install mingw-w64)"
	@echo "macOS build needs: osxcross SDK or build on a Mac"
