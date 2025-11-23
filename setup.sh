#!/bin/bash
# Quick Setup Script for arXiv TUI

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}═══════════════════════════════════════════════════${NC}"
echo -e "${BLUE}   arXiv TUI - Setup Script${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════${NC}"
echo ""

# Add helix to PATH
export PATH="$PATH:/Users/qtzx/.local/bin"

# Step 1: Check Rust
echo -e "${YELLOW}[1/4]${NC} Checking Rust..."
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}✗ Rust not found${NC}"
    echo "Install from: https://rustup.rs"
    exit 1
fi
RUST_VERSION=$(rustc --version)
echo -e "${GREEN}✓ ${RUST_VERSION}${NC}"

# Step 2: Check Docker
echo -e "${YELLOW}[2/4]${NC} Checking Docker..."
if ! command -v docker &> /dev/null; then
    echo -e "${YELLOW}⚠ Docker not found${NC}"
    echo ""
    echo "Docker is required to run HelixDB locally."
    echo ""
    echo "Install options:"
    echo "  1. Download Docker Desktop: https://www.docker.com/products/docker-desktop/"
    echo "  2. Or via Homebrew: brew install --cask docker"
    echo ""
    echo "After installation:"
    echo "  • Start Docker Desktop app"
    echo "  • Run this script again"
    echo ""
    exit 1
fi

if ! docker info &> /dev/null; then
    echo -e "${YELLOW}⚠ Docker is installed but not running${NC}"
    echo ""
    echo "Please start Docker Desktop and run this script again"
    exit 1
fi
echo -e "${GREEN}✓ Docker is running${NC}"

# Step 3: Check/Install HelixDB CLI
echo -e "${YELLOW}[3/4]${NC} Checking HelixDB CLI..."
if ! command -v helix &> /dev/null; then
    echo -e "${YELLOW}⚠ HelixDB CLI not found${NC}"
    echo "Installing HelixDB CLI..."
    curl -sSL https://install.helix-db.com | bash
    export PATH="$PATH:/Users/qtzx/.local/bin"
fi
HELIX_VERSION=$(helix --version)
echo -e "${GREEN}✓ ${HELIX_VERSION}${NC}"

# Step 4: Build project
echo -e "${YELLOW}[4/4]${NC} Building project..."
if cargo build --examples; then
    echo -e "${GREEN}✓ Build successful${NC}"
else
    echo -e "${RED}✗ Build failed${NC}"
    exit 1
fi

echo ""
echo -e "${GREEN}═══════════════════════════════════════════════════${NC}"
echo -e "${GREEN}   Setup Complete! ✓${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════${NC}"
echo ""
echo "Next steps:"
echo ""
echo "  1. Deploy HelixDB:"
echo -e "     ${BLUE}./run_pipeline.sh${NC}"
echo ""
echo "  2. Or manually:"
echo -e "     ${BLUE}helix push dev${NC}                           # Deploy HelixDB"
echo -e "     ${BLUE}cargo run --example helix_integration${NC}    # Test pipeline"
echo -e "     ${BLUE}cargo run --example semantic_search${NC}      # Interactive search"
echo ""
echo "  3. Just test arXiv (no DB needed):"
echo -e "     ${BLUE}cargo run --example arxiv_search${NC}"
echo ""
