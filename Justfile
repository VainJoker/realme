# ╔══════════════════════════════════════════════════════════════════════════╗
# ║                     🦀 Rust Development Justfile                          ║
# ╚══════════════════════════════════════════════════════════════════════════╝

# ============================================================================
# Configuration
# ============================================================================

set dotenv-load := true
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
set shell := ["bash", "-c"]

# Tool Categories
ESSENTIAL_TOOLS := "cargo-binstall cargo-nextest"
QUALITY_TOOLS   := "cargo-audit cargo-deny cargo-tarpaulin"
UTILITY_TOOLS   := "git-cliff typos-cli jaq cargo-cache cargo-watch cargo-outdated tokei"
ALL_TOOLS       := "{{ESSENTIAL_TOOLS}} {{QUALITY_TOOLS}} {{UTILITY_TOOLS}}"

# Build Flags
ALL_TARGETS_FLAG  := "--all-targets"
ALL_FEATURES_FLAG := "--all-features"

PACKAGES := if os() == "windows" {
    `try { $m = cargo metadata --format-version 1 --no-deps | ConvertFrom-Json; ($m.packages | ForEach-Object { $_.name }) -join ' ' } catch { '' }`
} else {
    `tool=""; if command -v jq >/dev/null 2>&1; then tool=jq; elif command -v jaq >/dev/null 2>&1; then tool=jaq; fi; if [ -n "$tool" ]; then cargo metadata --format-version 1 --no-deps | $tool -r '[.packages[].name] | join(" ")'; fi || echo ''`
}

IS_WORKSPACE := if os() == "windows" {
    `try { $m = cargo metadata --format-version 1 --no-deps | ConvertFrom-Json; if ($m.packages.Count -gt 1) { 'true' } else { 'false' } } catch { 'false' }`
} else {
    `tool=""; if command -v jq >/dev/null 2>&1; then tool=jq; elif command -v jaq >/dev/null 2>&1; then tool=jaq; fi; if [ -n "$tool" ]; then cargo metadata --format-version 1 --no-deps | $tool -r '.packages | length > 1'; else echo false; fi`
}

FEATURES := if os() == "windows" {
    `try { $m = cargo metadata --format-version 1 --no-deps | ConvertFrom-Json; ($m.packages | ForEach-Object { $_.features.PSObject.Properties.Name } | Sort-Object -Unique) -join ' ' } catch { '' }`
} else {
    `tool=""; if command -v jq >/dev/null 2>&1; then tool=jq; elif command -v jaq >/dev/null 2>&1; then tool=jaq; fi; if [ -n "$tool" ]; then cargo metadata --format-version 1 --no-deps | $tool -r '[.packages[].features | keys[]] | unique | sort | join(" ")'; fi || echo ''`
}

BIN := env_var_or_default("BIN", if os() == "windows" {
    `try { $m = cargo metadata --format-version 1 --no-deps | ConvertFrom-Json; ($m.packages | ForEach-Object { $_.targets | Where-Object { $_.kind -contains 'bin' } | ForEach-Object { $_.name } } | Sort-Object -Unique) -join ' ' } catch { '' }`
} else {
    `tool=""; if command -v jq >/dev/null 2>&1; then tool=jq; elif command -v jaq >/dev/null 2>&1; then tool=jaq; fi; if [ -n "$tool" ]; then cargo metadata --format-version 1 --no-deps | $tool -r '[.packages[].targets[] | select(.kind[] == "bin") | .name] | join(" ")'; fi || echo ''`
})

WORKSPACE_FLAG := if os() == "windows" {
    `try { $m = cargo metadata --format-version 1 --no-deps | ConvertFrom-Json; if ($m.packages.Count -gt 1) { '--workspace' } else { '' } } catch { '' }`
} else {
    `tool=""; if command -v jq >/dev/null 2>&1; then tool=jq; elif command -v jaq >/dev/null 2>&1; then tool=jaq; fi; if [ -n "$tool" ]; then cargo metadata --format-version 1 --no-deps | $tool -r 'if (.packages | length) > 1 then "--workspace" else "" end'; fi || echo ''`
}

# ============================================================================
# Default & Help
# ============================================================================

# Default workflow
default: dev

# Show available commands
help:
    @echo "╔══════════════════════════════════════════════════════════════════╗"
    @echo "║                    🦀 Rust Development Commands                   ║"
    @echo "╚══════════════════════════════════════════════════════════════════╝"
    @echo ""
    @just --list --unsorted
    @echo ""
    @echo "📝 Use 'just <command>' to run a command"
    @echo "📋 Use 'just info' to see project information"

# ============================================================================
# 🔄 Workflows
# ============================================================================

# Run complete development workflow
dev: format build
    @echo "╭─────────────────────────────────────╮"
    @echo "│ ✅ Development workflow complete    │"
    @echo "╰─────────────────────────────────────╯"

# Run CI/CD pipeline
ci: format lint check test release
    @echo "╭─────────────────────────────────────╮"
    @echo "│ ✅ CI/CD pipeline complete          │"
    @echo "╰─────────────────────────────────────╯"

# Quick validation
quick: format check test-unit
    @echo "╭─────────────────────────────────────╮"
    @echo "│ ✅ Quick validation complete        │"
    @echo "╰─────────────────────────────────────╯"

# ============================================================================
# 🔨 Build Commands
# ============================================================================

# Check code compilation
check:
    @echo "🔍 Checking code compilation..."
    @cargo check {{WORKSPACE_FLAG}} {{ALL_TARGETS_FLAG}} {{ALL_FEATURES_FLAG}}
    @echo "   └─ ✓ Code compiles successfully"

# Build in debug mode
build:
    @echo "🔨 Building project (debug mode)..."
    @cargo build {{WORKSPACE_FLAG}}
    @echo "   └─ ✓ Debug build complete"

# Build in release mode
release:
    @echo "🚀 Building project (release mode)..."
    @cargo build --release {{WORKSPACE_FLAG}}
    @echo "   └─ ✓ Release build complete"

# Watch for changes and run checks
watch: (_ensure-tool "cargo-watch")
    @echo "👀 Watching for changes..."
    @echo "   └─ Press Ctrl+C to stop"
    @cargo watch -x "check {{WORKSPACE_FLAG}} {{ALL_TARGETS_FLAG}}"

# ============================================================================
# 🧪 Testing
# ============================================================================

# Run all tests
test: test-unit test-doc test-examples
    @echo "╭─────────────────────────────────────╮"
    @echo "│ ✅ All tests passed                 │"
    @echo "╰─────────────────────────────────────╯"

# Run unit tests
test-unit: (_ensure-tool "cargo-nextest")
    @echo "🧪 Running unit tests..."
    @cargo nextest run {{WORKSPACE_FLAG}} {{ALL_TARGETS_FLAG}} {{ALL_FEATURES_FLAG}}

# Run examples
test-examples:
    @echo "🧪 Running example tests..."
    @cargo test --examples {{WORKSPACE_FLAG}} {{ALL_FEATURES_FLAG}}
    
# Run documentation tests
test-doc:
    @echo "📚 Running documentation tests..."
    @cargo test --doc {{WORKSPACE_FLAG}}

# Generate coverage report
coverage: (_ensure-tool "cargo-tarpaulin")
    @echo "📊 Generating test coverage report..."
    @cargo tarpaulin {{WORKSPACE_FLAG}} {{ALL_FEATURES_FLAG}} \
        --timeout 120 \
        --out Html --out Lcov --out Xml \
        --output-dir coverage
    @echo "   └─ 📁 Report saved to: coverage/"

# ============================================================================
# 🎨 Code Quality
# ============================================================================

# Format code
format:
    @echo "✨ Formatting code..."
    @cargo fmt --all 
    @echo "   └─ ✓ Code formatted"

# Run linter
lint:
    @echo "🧹 Running Clippy linter..."
    @cargo clippy {{WORKSPACE_FLAG}} {{ALL_TARGETS_FLAG}} {{ALL_FEATURES_FLAG}} -- -D warnings
    @echo "   └─ ✓ No lint issues found"

# Check documentation
lint-docs:
    @echo "📚 Checking documentation..."
    @cargo rustdoc {{WORKSPACE_FLAG}} -- -Zunstable-options --check -Dwarnings
    @echo "   └─ ✓ Documentation is valid"

# Check dependencies
lint-dependencies: (_ensure-tool "cargo-deny")
    @echo "🔍 Checking dependency compliance..."
    @cargo deny check licenses bans sources advisories
    @echo "   └─ ✓ Dependencies compliant"

# Auto-fix issues
fix:
    @echo "🔧 Auto-fixing issues..."
    @cargo clippy {{WORKSPACE_FLAG}} {{ALL_TARGETS_FLAG}} {{ALL_FEATURES_FLAG}} \
        --fix --allow-dirty --allow-staged -- -D warnings
    @just format
    @echo "   └─ ✓ Issues fixed"

# Check spelling
typos: (_ensure-tool "typos-cli")
    @echo "📝 Checking spelling..."
    @typos
    @echo "   └─ ✓ No typos found"

# ============================================================================
# 🔒 Security
# ============================================================================

# Run security audit
security: (_ensure-tool "cargo-audit")
    @echo "🔒 Running security audit..."
    @cargo audit
    @echo "   └─ ✓ No security vulnerabilities found"

# ============================================================================
# 📊 Analysis
# ============================================================================

# Show code statistics
stats: (_ensure-tool "tokei")
    @echo "📊 Code Statistics"
    @echo "════════════════════════════════════"
    @tokei

# Check for outdated dependencies
outdated: (_ensure-tool "cargo-outdated")
    @echo "📦 Checking for dependency updates..."
    @cargo outdated {{WORKSPACE_FLAG}}

# Show dependency tree
deps:
    @echo "🌳 Dependency Tree"
    @echo "════════════════════════════════════"
    @cargo tree {{WORKSPACE_FLAG}}

# ============================================================================
# 📚 Documentation
# ============================================================================

# Generate documentation
docs:
    @echo "📚 Generating documentation..."
    @cargo doc --no-deps {{WORKSPACE_FLAG}}
    @echo "   └─ 📁 Documentation saved to: target/doc/"

# Generate and open documentation
[windows]
docs-open: docs
    @echo "🌐 Opening documentation..."
    @Start-Process "target\doc\index.html" -ErrorAction SilentlyContinue

[unix]
docs-open: docs
    @echo "🌐 Opening documentation..."
    @xdg-open target/doc/index.html 2>/dev/null || \
     open target/doc/index.html 2>/dev/null || \
     echo "   └─ 📁 Documentation at: target/doc/"

# Generate changelog
changelog: (_ensure-tool "git-cliff")
    @echo "📝 Generating changelog..."
    @git cliff -o CHANGELOG.md
    @echo "   └─ 📄 Changelog saved to: CHANGELOG.md"

# ============================================================================
# 🚀 Execution
# ============================================================================

# Run binary
run BIN_NAME=BIN:
    @echo "▶️  Running binary: {{BIN_NAME}}"
    @echo "────────────────────────────────────"
    @cargo run --bin {{BIN_NAME}}

# Run example
run-example EXAMPLE:
    @echo "▶️  Running example: {{EXAMPLE}}"
    @echo "────────────────────────────────────"
    @cargo run --example {{EXAMPLE}}

# ============================================================================
# 🧹 Maintenance
# ============================================================================

# Clean build artifacts
clean:
    @echo "🧹 Cleaning build artifacts..."
    @cargo clean
    @echo "   └─ ✓ Build artifacts cleaned"

# Clean cargo cache
clean-cache: (_ensure-tool "cargo-cache")
    @echo "🧹 Cleaning Cargo cache..."
    @cargo cache --autoclean
    @echo "   └─ ✓ Cache cleaned"

# Deep clean (artifacts + cache)
clean-all: clean clean-cache
    @echo "   └─ ✓ Full cleanup complete"

# ============================================================================
# ℹ️ Information
# ============================================================================

# Display project information
info:
    @echo "╔══════════════════════════════════════════════════════════════════╗"
    @echo "║                      📋 Project Information                       ║"
    @echo "╚══════════════════════════════════════════════════════════════════╝"
    @echo ""
    @echo "  Workspace : {{IS_WORKSPACE}}"
    @echo "  Packages  : {{PACKAGES}}"
    @echo "  Features  : {{FEATURES}}"
    @echo "  Binaries  : {{BIN}}"
    @echo ""

# Display environment variables
env:
    @echo "🌍 Environment Variables"
    @echo "════════════════════════════════════"
    @echo "  IS_WORKSPACE   : {{IS_WORKSPACE}}"
    @echo "  PACKAGES       : {{PACKAGES}}"
    @echo "  FEATURES       : {{FEATURES}}"
    @echo "  BIN            : {{BIN}}"

# ============================================================================
# 🔧 Setup
# ============================================================================

# Setup development environment
[windows]
setup:
    @Write-Host '╔══════════════════════════════════════════════════════════════════╗' -ForegroundColor Cyan
    @Write-Host '║              🔧 Setting up development environment                ║' -ForegroundColor Cyan
    @Write-Host '╚══════════════════════════════════════════════════════════════════╝' -ForegroundColor Cyan
    @foreach ($tool in "{{ESSENTIAL_TOOLS}}".Split()) { just _ensure-tool "$tool" }
    @Write-Host '✅ Development environment ready!' -ForegroundColor Green

[unix]
setup:
    @echo "╔══════════════════════════════════════════════════════════════════╗"
    @echo "║              🔧 Setting up development environment                ║"
    @echo "╚══════════════════════════════════════════════════════════════════╝"
    @for tool in {{ESSENTIAL_TOOLS}}; do just _ensure-tool "$tool"; done
    @echo "✅ Development environment ready!"

# ============================================================================
# 🛠️ Internal Helpers
# ============================================================================

# Ensure tool is installed (Windows)
[windows]
_ensure-tool tool:
    @$exists = Get-Command {{tool}} -ErrorAction SilentlyContinue; \
    if (-not $exists) { \
        just _install-tool {{tool}} \
    }

# Ensure tool is installed (Unix)
[unix]
_ensure-tool tool:
    @bin={{tool}}; \
    if ! command -v "$bin" >/dev/null 2>&1; then \
        just _install-tool {{tool}}; \
    fi

# Install tool (Windows)
[windows]
_install-tool tool:
    @Write-Host "  📦 Installing {{tool}}..." -ForegroundColor Yellow
    @$hasBinstall = Get-Command cargo-binstall -ErrorAction SilentlyContinue; \
    $success = $false; \
    if ($hasBinstall) { \
        $output = cargo binstall {{tool}} --no-confirm --log-level warn 2>&1; \
        if ($LASTEXITCODE -eq 0) { \
            $success = $true \
        } else { \
            Write-Host "     ⚠️  Falling back to cargo install..." -ForegroundColor Yellow; \
            $output = cargo install {{tool}} 2>&1; \
            if ($LASTEXITCODE -eq 0) { $success = $true } \
        } \
    } else { \
        $output = cargo install {{tool}} 2>&1; \
        if ($LASTEXITCODE -eq 0) { $success = $true } \
    }; \
    if ($success) { \
        Write-Host "     └─ ✓ {{tool}} installed" -ForegroundColor Green \
    } else { \
        Write-Host "     ✗ Failed to install {{tool}}" -ForegroundColor Red; \
        $output | Select-Object -Last 5 | ForEach-Object { Write-Host "       $_" -ForegroundColor Red } \
    }

# Install tool (Unix)
[unix]
_install-tool tool:
    @echo "  📦 Installing {{tool}}..."
    @if command -v cargo-binstall >/dev/null 2>&1; then \
        if cargo binstall {{tool}} --no-confirm --log-level warn 2>&1 | grep -q "error\|failed"; then \
            echo "     ⚠️  Falling back to cargo install..."; \
            cargo install {{tool}} 2>&1 | tail -5; \
        fi \
    else \
        cargo install {{tool}} 2>&1 | tail -5; \
    fi && echo "     └─ ✓ {{tool}} installed" || echo "     ✗ Failed to install {{tool}}"

# ============================================================================
# 🚄 Aliases (Shortcuts)
# ============================================================================

# Build
alias b  := build
alias br := release

# Testing
alias t  := test
alias tu := test-unit
alias td := test-doc
alias cov := coverage

# Quality
alias f  := format
alias l  := lint
alias ld := lint-docs
alias lp := lint-dependencies
alias fx := fix
alias ty := typos

# Analysis
alias st := stats
alias od := outdated
alias dp := deps

# Execution
alias r  := run
alias re := run-example

# Documentation
alias d  := docs
alias do := docs-open

# Maintenance
alias c  := check
alias cl := clean
alias clc := clean-cache
alias cla := clean-all

# Security
alias sec := security

# Information
alias i  := info
alias e  := env

# Workflows
alias q  := quick
alias w  := watch

# Setup
alias s  := setup